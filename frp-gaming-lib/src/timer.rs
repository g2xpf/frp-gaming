use sodium_rust::{Cell, CellSink, SodiumCtx, Stream, StreamSink};
use std::time::{Duration, Instant};

// Stream: tick, elapsed
// Cell: current time
pub struct Timer {
    ctx: SodiumCtx,
    start_time_sink: CellSink<Instant>,
    pub(crate) tick_sink: StreamSink<usize>,
    pub(crate) time_sink: CellSink<Instant>,

    pub tick: Stream<usize>,
    pub elapsed: Cell<Duration>,
    pub delta_elapsed: Stream<Duration>,
}

// --tick--> snapshot >--delta_elapsed->
//    |         ^
//    |        prev
//    |         ^
//    +----> hold(now)
impl Timer {
    pub fn new(sodium_ctx: &SodiumCtx) -> Self {
        let ctx = sodium_ctx.clone();
        let tick_sink = sodium_ctx.new_stream_sink();
        let tick = tick_sink.stream();
        let now = Instant::now();
        let start_time_sink = sodium_ctx.new_cell_sink(now);
        let start_time_cell = start_time_sink.cell();
        let time_sink = sodium_ctx.new_cell_sink(now);
        let time = time_sink.cell();
        let time_stream = tick.snapshot(&time, |_: &_, time: &Instant| *time);
        let prev_time = time_stream.hold(now);
        let delta_elapsed = time_stream
            .snapshot(&prev_time, |cur_time: &Instant, prev_time: &Instant| {
                *cur_time - *prev_time
            });
        let elapsed = time.lift2(
            &start_time_cell,
            |current_time: &Instant, start_time: &Instant| *current_time - *start_time,
        );

        Timer {
            ctx,
            tick_sink,
            tick,
            time_sink,
            elapsed,
            delta_elapsed,
            start_time_sink,
        }
    }

    pub(crate) fn initialize(&self) {
        let now = Instant::now();
        self.ctx.transaction(|| {
            self.time_sink.send(now);
            self.start_time_sink.send(now);
        });
    }
}

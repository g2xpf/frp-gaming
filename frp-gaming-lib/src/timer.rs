use sodium_rust::{Cell, CellSink, SodiumCtx, Stream, StreamSink};
use std::time::{Duration, Instant};

// Stream: tick, elapsed
// Cell: current time
pub struct Timer {
    pub(crate) tick_sink: StreamSink<usize>,
    pub(crate) time_sink: CellSink<Instant>,

    pub tick: Stream<usize>,
    pub elapsed: Cell<Duration>,
    pub delta_elapsed: Stream<Duration>,
}

impl Timer {
    pub fn new(sodium_ctx: &SodiumCtx) -> Self {
        let tick_sink = sodium_ctx.new_stream_sink();
        let tick = tick_sink.stream();
        let now = Instant::now();
        let start_time_cell = sodium_ctx.new_cell(now);
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
            tick_sink,
            tick,
            time_sink,
            elapsed,
            delta_elapsed,
        }
    }
}

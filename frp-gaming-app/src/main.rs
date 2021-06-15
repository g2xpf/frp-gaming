use std::io::stdout;
use std::io::Write;
use std::time::Duration;

use frp_gaming_app::drawer::create_drawer_command;
use frp_gaming_lib::input::Input;
use frp_gaming_lib::main_loop::MainLoop;
use frp_gaming_lib::output::Output;
use frp_gaming_lib::stream_util;
use frp_gaming_lib::timer::Timer;
use frp_gaming_lib::{glium, sodium_rust};

use frp_gaming_app::control_flow;

use glium::glutin::event;

use sodium_rust::SodiumCtx;

fn main() {
    let ctx = SodiumCtx::new();
    let event_sink = ctx.new_stream_sink();
    let timer = Timer::new(&ctx);

    let event = event_sink.stream();

    let received_character = stream_util::filter_map(&event, |event: &_| {
        if let event::Event::WindowEvent {
            event: event::WindowEvent::ReceivedCharacter(c),
            ..
        } = event
        {
            Some(*c)
        } else {
            None
        }
    });

    let drawer_command = create_drawer_command(&event, &timer);

    received_character.listen(|c: &char| {
        let c = match c {
            '\r' => '\n',
            c => *c,
        };
        print!("{}", c);
        stdout().flush().unwrap();
    });

    let event_emiiter_command = ctx.new_stream();

    let control_flow = control_flow::control_flow::<()>(&event);

    let input = Input {
        event: event_sink,
        timer,
    };
    let output = Output {
        drawer_command,
        control_flow,
        event_emiiter_command,
    };

    let main_loop = MainLoop::new();
    main_loop.run(input, output);
}

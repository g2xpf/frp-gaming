use std::time::Duration;

use frp_gaming_lib::drawer::{Drawer, DrawerCommand};
use frp_gaming_lib::glium::glutin::event::Event;
use frp_gaming_lib::glium::{Frame, Surface};
use frp_gaming_lib::sodium_rust::Stream;
use frp_gaming_lib::timer::Timer;

pub fn create_drawer_command<CustomEvent>(
    _event: &Stream<Event<'static, CustomEvent>>,
    timer: &Timer,
) -> Stream<DrawerCommand> {
    let elapsed = timer
        .tick
        .snapshot(&timer.elapsed, |_: &_, time: &Duration| *time);
    elapsed.map(|elapsed: &Duration| {
        let color = duration_to_color(elapsed);
        DrawerCommand::AddSingle(Drawer::new(move |frame: &mut Frame| {
            let (red, green, blue, alpha) = color;
            frame.clear_color(red, green, blue, alpha)
        }))
    })
}

fn duration_to_color(elapsed: &Duration) -> (f32, f32, f32, f32) {
    let mut seconds = elapsed.as_secs_f32() % 3.0;
    let mut color = (0.25, 0.25, 0.25, 1.0);

    if seconds < 1.0 {
        color.0 += 1.0 - seconds;
        color.1 += seconds;
    } else if seconds < 2.0 {
        seconds -= 1.0;
        color.1 += 1.0 - seconds;
        color.2 += seconds;
    } else {
        seconds -= 2.0;
        color.2 += 1.0 - seconds;
        color.0 += seconds;
    }
    color
}

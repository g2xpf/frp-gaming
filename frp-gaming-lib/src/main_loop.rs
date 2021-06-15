use std::time::Instant;

use glium::{
    glutin::event::Event,
    glutin::event_loop::{ControlFlow, EventLoop, EventLoopProxy},
    glutin::window,
    glutin::ContextBuilder,
    Display,
};

use crate::command::Command;
use crate::input::Input;
use crate::output::Output;

pub struct MainLoop<CustomEvent: 'static> {
    display: Display,
    event_loop_proxy: EventLoopProxy<CustomEvent>,
    event_loop: EventLoop<CustomEvent>,
}

impl<CustomEvent> Default for MainLoop<CustomEvent>
where
    CustomEvent: Clone + Send + Sync + 'static,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<CustomEvent> MainLoop<CustomEvent>
where
    CustomEvent: Clone + Send + Sync + 'static,
{
    pub fn new() -> Self {
        let event_loop = EventLoop::with_user_event();
        let event_loop_proxy = event_loop.create_proxy();
        let wb = window::WindowBuilder::new();
        let cb = ContextBuilder::new().with_vsync(true);
        let display = glium::Display::new(wb, cb, &event_loop).unwrap();

        MainLoop {
            display,
            event_loop_proxy,
            event_loop,
        }
    }

    pub fn run(self, input: Input<CustomEvent>, output: Output<CustomEvent>) -> ! {
        let event_loop = self.event_loop;
        let event_loop_proxy = self.event_loop_proxy;
        let display = self.display;

        let drawers = Command::accum_command_stream(&output.drawer_command);
        let event_emitters = Command::accum_command_stream(&output.event_emiiter_command);

        let control_flow_cell = output.control_flow.hold(ControlFlow::Poll);

        let mut frame_count = 0;
        event_loop.run(move |event, _, control_flow| {
            match &event {
                Event::NewEvents(glium::glutin::event::StartCause::Init) => {
                    input.timer.initialize();
                }
                Event::MainEventsCleared => {
                    input.timer.time_sink.send(Instant::now());
                    input.timer.tick_sink.send(frame_count);

                    display.gl_window().window().request_redraw();
                }
                Event::RedrawRequested(_) => {
                    let drawers = drawers.sample();
                    let mut frame = display.draw();
                    for drawer in drawers {
                        drawer.call(&mut frame)
                    }
                    frame.finish().unwrap();
                }
                Event::RedrawEventsCleared => {
                    frame_count += 1;
                    let event_emiiters = event_emitters.sample();
                    for event_emiiter in event_emiiters {
                        event_emiiter.call(&event_loop_proxy)
                    }
                }
                _ => {}
            }

            if let Some(event) = event.to_static() {
                input.event.send(event);
                *control_flow = control_flow_cell.sample();
            }
        })
    }
}

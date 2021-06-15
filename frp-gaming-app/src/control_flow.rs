use frp_gaming_lib::{
    glium::glutin::event::KeyboardInput,
    glium::glutin::event::VirtualKeyCode,
    glium::glutin::{event, event::Event, event_loop::ControlFlow},
    sodium_rust::Stream,
};

pub fn control_flow<CustomEvent>(input: &Stream<Event<'static, CustomEvent>>) -> Stream<ControlFlow>
where
    CustomEvent: Clone + Send + 'static,
{
    input.map(|event: &Event<'static, CustomEvent>| {
        if let event::Event::WindowEvent {
            event: event::WindowEvent::CloseRequested,
            ..
        } = event
        {
            ControlFlow::Exit
        } else if let event::Event::WindowEvent {
            event:
                event::WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            state: event::ElementState::Pressed,
                            virtual_keycode: Some(VirtualKeyCode::Escape),
                            ..
                        },
                    ..
                },
            ..
        } = event
        {
            ControlFlow::Exit
        } else {
            ControlFlow::Wait
        }
    })
}

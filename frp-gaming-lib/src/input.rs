use crate::timer::Timer;
use glium::glutin::event::Event;
use sodium_rust::StreamSink;

pub struct Input<CustomEvent>
where
    CustomEvent: 'static,
{
    pub event: StreamSink<Event<'static, CustomEvent>>,
    pub timer: Timer,
}

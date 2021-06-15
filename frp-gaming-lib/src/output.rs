use crate::drawer::DrawerCommand;
use crate::event_emitter::EventEmitterCommand;
use glium::glutin::event_loop::ControlFlow;
use sodium_rust::Stream;

pub struct Output<CustomEvent>
where
    CustomEvent: Clone + Send + Sync + 'static,
{
    pub control_flow: Stream<ControlFlow>,
    pub drawer_command: Stream<DrawerCommand>,
    pub event_emiiter_command: Stream<EventEmitterCommand<CustomEvent>>,
}

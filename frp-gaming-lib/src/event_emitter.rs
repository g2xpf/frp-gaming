use crate::command::Command;
use dyn_clone::{clone_trait_object, DynClone};
use glium::glutin::event_loop::EventLoopProxy;

pub trait EventEmittable<CustomEvent>:
    DynClone + Fn(&EventLoopProxy<CustomEvent>) + Send + Sync + 'static
{
}
clone_trait_object!(<CustomEvent> EventEmittable<CustomEvent> where CustomEvent: Clone + Send + 'static);

#[derive(Clone)]
pub struct EventEmitter<CustomEvent>(Box<dyn EventEmittable<CustomEvent>>)
where
    CustomEvent: Clone + Send + Sync + 'static;

impl<CustomEvent> EventEmitter<CustomEvent>
where
    CustomEvent: Clone + Send + Sync + 'static,
{
    pub fn new(f: impl EventEmittable<CustomEvent>) -> Self {
        EventEmitter(Box::new(f))
    }

    pub fn call(&self, event_loop_proxy: &EventLoopProxy<CustomEvent>) {
        (self.0)(event_loop_proxy)
    }
}

pub type EventEmitterCommand<CustomEvent> = Command<EventEmitter<CustomEvent>>;

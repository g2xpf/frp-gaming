use crate::command::Command;
use dyn_clone::{clone_trait_object, DynClone};
use glium::Frame;

impl<T> Drawable for T where T: Fn(&mut Frame) + Clone + Send + Sync + 'static {}

pub trait Drawable: DynClone + Fn(&mut Frame) + Send + Sync + 'static {}
clone_trait_object!(Drawable);

#[derive(Clone)]
pub struct Drawer(Box<dyn Drawable>);
impl Drawer {
    pub fn new(f: impl Drawable) -> Self {
        Drawer(Box::new(f))
    }

    pub fn call(&self, frame: &mut Frame) {
        (self.0)(frame);
    }
}

pub type DrawerCommand = Command<Drawer>;

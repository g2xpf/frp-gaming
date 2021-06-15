use im::{vector, Vector};
use sodium_rust::{Cell, Stream};

#[derive(Clone)]
pub enum Command<T> {
    AddSingle(T),
    AddMultiple(Vector<T>),
    Clear,
}

impl<T> Command<T>
where
    T: Clone,
{
    pub fn merge(&self, other: &Command<T>) -> Command<T> {
        use Command::*;
        match (self, other) {
            (Clear, t2) => t2.clone(),
            (_, Clear) => Clear,
            (AddSingle(t1), AddSingle(t2)) => {
                let t1 = t1.clone();
                let t2 = t2.clone();
                AddMultiple(vector![t1, t2])
            }
            (AddSingle(t1), AddMultiple(t2)) => {
                let t1 = t1.clone();
                let t2 = t2.clone();
                AddMultiple({
                    let mut v = vector![t1];
                    v.append(t2);
                    v
                })
            }
            (AddMultiple(t1), AddSingle(t2)) => {
                let mut t1 = t1.clone();
                let t2 = t2.clone();
                t1.push_back(t2);
                AddMultiple(t1)
            }
            (AddMultiple(t1), AddMultiple(t2)) => {
                let mut t1 = t1.clone();
                let t2 = t2.clone();
                t1.append(t2);
                AddMultiple(t1)
            }
        }
    }
}

impl<T> Command<T>
where
    // im::Vector<T> satisfies `Send` only when T satisfies `Send + Sync`
    T: Clone + Send + Sync + 'static,
{
    pub fn accum_command_stream(command: &Stream<Self>) -> Cell<Vector<T>> {
        command.accum(
            vector![],
            |command: &Self, old_state: &Vector<T>| -> Vector<T> {
                use Command::*;
                let mut new_state = old_state.clone();
                match command {
                    AddSingle(drawer) => new_state.push_back(drawer.clone()),
                    AddMultiple(drawers) => new_state.append(drawers.clone()),
                    Clear => new_state.clear(),
                }
                new_state
            },
        )
    }
}

use std::fmt::Debug;

use crate::{
    traits::TState,
    transition::Transition,
    types::{Action, Condition},
};

pub struct State<K, T> {
    pub state: Box<dyn TState<T>>,
    pub transitions: Vec<Transition<K, T>>,
}

impl<K, T> State<K, T>
where
    T: Debug,
{
    pub fn new<S: TState<T> + 'static>(state: S) -> Self {
        Self {
            state: Box::new(state),
            transitions: vec![],
        }
    }

    pub fn transition(mut self, to: K, condition: Condition<T>, actions: Vec<Action<T>>) -> Self {
        self.transitions.push(Transition {
            to,
            condition,
            actions,
        });
        self
    }

    pub fn decide(&self, context: &mut T) -> Option<&Transition<K, T>>
    where
        K: Clone,
    {
        for transition in self.transitions.iter() {
            if (transition.condition)(context) {
                return Some(transition);
            }
        }
        None
    }
}

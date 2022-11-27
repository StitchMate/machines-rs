use std::{collections::HashMap, fmt::Debug, hash::Hash};

use crate::{state::State, types::Action};

pub struct Machine<K, T> {
    states: HashMap<K, State<K, T>>,
    active_state: K,
}

impl<K: Hash + Eq + Debug, T: Debug> Machine<K, T> {
    pub fn new(active_state: K) -> Self {
        Self {
            states: Default::default(),
            active_state,
        }
    }

    pub fn state(mut self, id: K, state: State<K, T>) -> Self {
        self.states.insert(id, state);
        self
    }

    pub fn set_active_state(&mut self, id: K, actions: Vec<Action<T>>, context: &mut T) {
        if let Some(state) = self.states.get_mut(&self.active_state) {
            state.state.exit(context);
        }
        for action in actions {
            (action)(context)
        }
        if let Some(state) = self.states.get_mut(&id) {
            state.state.entry(context);
            self.active_state = id;
        }
    }

    pub fn decide(&mut self, context: &mut T)
    where
        K: Clone,
    {
        if let Some(state) = self.states.get(&self.active_state) {
            if let Some(selected) = state.decide(context) {
                self.set_active_state(selected.to.clone(), selected.actions.clone(), context);
            }
        }
    }

    pub fn update(&mut self, context: &mut T) {
        if let Some(state) = self.states.get_mut(&self.active_state) {
            state.state.update(context);
        }
    }
}

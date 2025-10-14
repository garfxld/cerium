use std::{
    any::{Any, TypeId},
    collections::HashMap,
    sync::RwLock,
};

use crate::event::Event;

pub struct Events {
    listeners: RwLock<HashMap<TypeId, Vec<Box<dyn Fn(&mut dyn Any) + Send + Sync>>>>,
}

impl Events {
    pub fn new() -> Self {
        Self {
            listeners: RwLock::new(HashMap::new()),
        }
    }

    pub fn subscribe<E, F>(&self, f: F) -> &Self
    where
        E: Event + 'static,
        F: Fn(&mut E) + Send + Sync + 'static,
    {
        let type_id = TypeId::of::<E>();
        let mut listeners = self.listeners.write().unwrap();
        let listeners = listeners.entry(type_id).or_insert(Vec::new());

        let wrapper = Box::new(move |event: &mut dyn Any| {
            if let Some(concrete_event) = event.downcast_mut::<E>() {
                f(concrete_event);
            }
        });

        listeners.push(wrapper);
        self
    }

    pub fn fire<E>(&self, event: &mut E)
    where
        E: Event + 'static,
    {
        let type_id = TypeId::of::<E>();
        let listeners = &self.listeners.read().unwrap();
        if let Some(listeners) = listeners.get(&type_id) {
            for listener in listeners {
                listener(event);
            }
        }
    }
}

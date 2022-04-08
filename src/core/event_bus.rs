#![allow(dead_code)]

use super::Event;
use super::Subscriber;

pub struct EventBus<T> {
    pub events: Vec<Event<T>>,
    pub listeners: Vec<Box<dyn Subscriber<Input = T>>>,
}

impl<T> EventBus<T> {
    pub fn new() -> EventBus<T> {
        EventBus {
            events: Vec::new(),
            listeners: Vec::new(),
        }
    }

    pub fn subscribe_listener(&mut self, listener: Box<dyn Subscriber<Input = T>>) {
        self.listeners.push(listener);
    }

    pub fn publish(&mut self, message: Event<T>) {
        self.events.push(message);
    }

    pub fn clear(&mut self) {
        self.events.clear();
    }

    /* Upon run, messages will be cleared! */

    pub fn run(&mut self) {
        for message in self.events.drain(..) {
            for listener in self.listeners.iter_mut() {
                listener.on_event(&message);
            }
        }
    }
}

// Default

impl<T> Default for EventBus<T> {
    fn default() -> Self {
        Self::new()
    }
}

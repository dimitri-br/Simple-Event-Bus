use super::Subscriber;
use super::Event;

pub struct EventBus<T> {
    pub messages: Vec<Event<T>>,
    pub listeners: Vec<Box<dyn Subscriber<Input = T>>>,
}

impl<T> EventBus<T> {
    pub fn new() -> EventBus<T> {
        EventBus {
            messages: Vec::new(),
            listeners: Vec::new(),
        }
    }

    pub fn subscribe_listener(&mut self, listener: Box<dyn Subscriber<Input = T>>) {
        self.listeners.push(listener);
    }

    pub fn publish(&mut self, message: Event<T>) {
        self.messages.push(message);
    }

    pub fn run(&mut self) {
        for message in self.messages.drain(..) {
            for listener in self.listeners.iter_mut() {
                listener.on_message(&message);
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

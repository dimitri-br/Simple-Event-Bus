#![allow(dead_code)]

use super::Event;
use super::Subscriber;


/// # Event Bus
/// 
/// The event bus is a central hub for all events.
/// It is responsible for managing all subscribers and publishing events
/// related to the event bus.
/// 
/// ## Fields
/// 
/// * `events` - A vec of events that have been published to the event bus.
/// 
/// * `subscribers` - A vec of all subscribers to the event bus.
/// 
/// ## Methods
/// 
/// * `publish` - Publishes an event to the event bus.
/// 
/// * `subscribe_listener` - Subscribes a listener to the event bus.
/// 
/// * `run` - Runs through each event, and calls each listener's on_event method.
/// 
/// * `clear` - Clears all events from the event bus.
pub struct EventBus<T> {
    /// A vec of events that have been published to the event bus.
    events: Vec<Event<T>>,
    /// A vec of all subscribers that are linked to the event bus.
    subscribers: Vec<Box<dyn Subscriber<Input = T>>>,
}

impl<T> EventBus<T> {
    /// # New
    /// 
    /// Creates a new event bus.
    pub fn new() -> EventBus<T> {
        EventBus {
            events: Vec::new(),
            subscribers: Vec::new(),
        }
    }

    /// # Publish
    /// 
    /// Publishes an event to the event bus.
    pub fn publish(&mut self, message: Event<T>) {
        self.events.push(message);
    }

    /// # Subscribe Listener
    /// 
    /// Subscribes a listener to the event bus.
    pub fn subscribe_listener(&mut self, listener: Box<dyn Subscriber<Input = T>>) {
        self.subscribers.push(listener);
    }

    /* Upon run, messages will be cleared! */

    /// # Run
    /// 
    /// Runs through each event, and calls each listener's on_event method.
    pub fn run(&mut self) {
        for event in self.events.drain(..) {
            for listener in self.subscribers.iter_mut() {
                listener.on_event(&event);
            }
        }
    }

    /// # Clear
    /// 
    /// Clears all events from the event bus.
    pub fn clear(&mut self) {
        self.events.clear();
    }
}

// Default
impl<T> Default for EventBus<T> {
    fn default() -> Self {
        Self::new()
    }
}

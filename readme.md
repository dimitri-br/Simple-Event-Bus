# Simple Event Bus

Minimal event bus in rust.
It provides a basic event bus system that works with (probably) any type.


Example Usage:

```rust
use simple_event_bus::{Event, EventBus, Subscriber};

struct ExampleSubscriber {
    pub name: String,
}

impl ExampleSubscriber {
    pub fn new(name: String) -> ExampleSubscriber {
        ExampleSubscriber { name }
    }
}

impl Subscriber for ExampleSubscriber {
    type Input = String;

    fn on_event(&mut self, event: &Event<Self::Input>) {
        println!("{} received message: {}", self.name, event.get_data());
    }
}

fn main() {
    let mut event_bus = EventBus::new();
    event_bus.subscribe_listener(ExampleSubscriber::new("Subscriber 1".to_string()));
    event_bus.subscribe_listener(ExampleSubscriber::new("Subscriber 2".to_string()));

    event_bus.publish(Event::new("hello".to_string()));
    event_bus.publish(Event::new("world".to_string()));

    // Runs through each event, and calls each listener's on_event method.
    event_bus.run();
}
```

Feel free to fork this implementation to add your own features!
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

    // We have to manually create and add each subscriber to the event bus.
    event_bus.subscribe_listener(ExampleSubscriber::new("Subscriber 1".to_string()));
    event_bus.subscribe_listener(ExampleSubscriber::new("Subscriber 2".to_string()));

    // We can manually define an event and publish it to the event bus.
    event_bus.publish(Event::new("hello".to_string()));
    event_bus.publish(Event::new("world".to_string()));

    // Alternatively, we can use the From trait to define an event and publish it to the event bus.
    // This is as simple as using ".into()" on the data (so long as it matches the event bus's type parameter).
    event_bus.publish("!".to_string().into());

    // Here we show another example, this time directly from a String instead of using &str.
    let test_string = String::from("Hello, World!");
    event_bus.publish(test_string.into());

    // Runs through each event, and calls each listener's on_event method.
    event_bus.run();
}

```

Feel free to fork this implementation to add your own features!
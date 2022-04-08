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

    fn on_event(&mut self, message: &Event<Self::Input>) {
        println!("{} received message: {}", self.name, message.get_data());
    }
}

fn main() {
    let mut message_queue = EventBus::new();
    message_queue.subscribe_listener(Box::new(ExampleSubscriber::new("listener 1".to_string())));
    message_queue.subscribe_listener(Box::new(ExampleSubscriber::new("listener 2".to_string())));

    message_queue.publish(Event::new("hello".to_string()));
    message_queue.publish(Event::new("world".to_string()));

    // Runs through each event, and calls each listener's on_event method.
    message_queue.run();
}

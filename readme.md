# Message Queue

Minimal message queue in rust.


Example Usage:

```rust
struct ExampleListener{
    pub name: String,
}

impl ExampleListener{
    pub fn new(name: String) -> ExampleListener{
        ExampleListener{
            name: name,
        }
    }
}

impl Listener for ExampleListener{
    type Input = String;
    fn on_message(&mut self, message: &Message<Self::Input>){
        println!("{} received message: {}", self.name, message.get_data());
    }
}

fn main(){
    let mut message_queue = MessageQueue::new();
    message_queue.register_listener(Box::new(ExampleListener::new("listener 1".to_string())));

    message_queue.push(Message::new("hello".to_string()));
    message_queue.push(Message::new("world".to_string()));
    message_queue.push(Message::new("REEE".to_string()));
    
    message_queue.run();
}
```

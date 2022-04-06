use super::Listener;
use super::Message;


pub struct MessageQueue<T>{
    pub messages: Vec<Message<T>>,
    pub listeners: Vec<Box<dyn Listener<Input=T>>>,
}

impl<T> MessageQueue<T>{
    pub fn new() -> MessageQueue<T>{
        MessageQueue{
            messages: Vec::new(),
            listeners: Vec::new(),
        }
    }

    pub fn register_listener(&mut self, listener: Box<dyn Listener<Input=T>>){
        self.listeners.push(listener);
    }

    pub fn push(&mut self, message: Message<T>){
        self.messages.push(message);
    }

    pub fn run(&mut self){
        for message in self.messages.drain(..){
            for listener in self.listeners.iter_mut(){
                listener.on_message(&message);
            }
        }
    }
}
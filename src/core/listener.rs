use super::Message;


pub trait Listener{
    type Input;
    
    fn on_message(&mut self, message: &Message<Self::Input>);
}
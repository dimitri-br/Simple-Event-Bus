use super::Event;

pub trait Subscriber {
    type Input;

    fn on_message(&mut self, message: &Event<Self::Input>);
}

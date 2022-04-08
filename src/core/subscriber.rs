use super::Event;

pub trait Subscriber {
    type Input;

    fn on_event(&mut self, message: &Event<Self::Input>);
}

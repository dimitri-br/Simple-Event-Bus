pub struct Event<T> {
    pub data: T,
}

impl<T> Event<T> {
    pub fn new(data: T) -> Event<T> {
        Event { data }
    }

    pub fn get_data(&self) -> &T {
        &self.data
    }
}

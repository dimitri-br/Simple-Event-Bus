/// # Event
///
/// An event is a struct that can
/// hold any data type. It is
/// then published to the event
/// bus. Once published, the event
/// is then passed to each subscriber
/// when the event bus runs.
///
/// ## Fields
///
/// * `data` - The data that is held by the event.
///
/// ## Methods
///
/// * `new` - Creates a new event.
///
/// * `get_data` - Returns the data held by the event.
pub struct Event<T> {
    /// The data that is held by the event.
    pub data: T,
}

impl<T> Event<T> {
    /// # New
    ///
    /// Creates a new event.
    pub fn new(data: T) -> Event<T> {
        Event { data }
    }

    /// # Get Data
    ///
    /// Returns the data held by the event.
    pub fn get_data(&self) -> &T {
        &self.data
    }
}


impl<T> From<T> for Event<T> {
    fn from(data: T) -> Self {
        Event::new(data)
    }
}
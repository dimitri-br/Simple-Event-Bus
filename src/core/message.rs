pub struct Message<T>{
    pub data: T,
}

impl<T> Message<T>{
    pub fn new(data: T) -> Message<T>{
        Message{
            data: data,
        }
    }

    pub fn get_data(&self) -> &T{
        &self.data
    }

    pub fn get_data_mut(&mut self) -> &mut T{
        &mut self.data
    }
}

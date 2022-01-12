use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Object<T>{
    pub data: Vec<T>
}

impl<T> Object<T> {
    /// Get a reference to the object's data.
    pub fn data(&self) -> &[T] {
        self.data.as_ref()
    }
}
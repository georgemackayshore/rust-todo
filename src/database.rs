use crate::todo::Todo;
use std::collections::HashMap;
use std::sync::Mutex;

pub type Db = Mutex<HashMap<u32, Todo>>;

pub fn create_map() -> Db {
    Mutex::new(HashMap::<u32, Todo>::new())
}

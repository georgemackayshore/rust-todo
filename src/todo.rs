use std::fmt;

#[derive(Serialize, Deserialize)]
pub struct Todo {
    id: Option<u32>,
    name: String,
    complete: Option<bool>,
}

impl Todo {
    pub fn new(id: u32, name: String, complete: bool) -> Self {
        Self {
            id: Some(id),
            name,
            complete: Some(complete),
        }
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn is_complete(&self) -> bool {
        self.complete.unwrap_or(false)
    }

    pub fn mark_complete(&mut self) {
        self.complete = Some(true);
    }
}

impl fmt::Display for Todo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let complete = if self.is_complete() { "✅" } else { "❎" };

        write!(f, "({}) {} - {}", self.id.unwrap(), self.name, complete)
    }
}

use std::fmt;

pub struct Task {
    description: String,
    pub is_done: bool,
}

impl Task {
    pub fn new(desc: String) -> Task {
        Task { description: desc, is_done: false }
    }
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}] {}", if self.is_done { "√" } else { "X" }, self.description)
    }
}

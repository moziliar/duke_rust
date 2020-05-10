use std::fmt;

pub trait Task: fmt::Display {
    fn is_done(&self) -> bool;
    fn complete(&mut self);
}

pub struct ToDo {
    description: String,
    is_done: bool,
}

impl Task for ToDo {
    fn is_done(&self) -> bool {
        self.is_done
    }
    fn complete(&mut self) {
        self.is_done = true;
    }
}

impl ToDo {
    pub fn new(description: String) -> Self {
        ToDo {
            description,
            is_done: false,
        }
    }
}

impl fmt::Display for ToDo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "[T][{}] {}",
            if self.is_done { "√" } else { "X" },
            self.description
        )
    }
}

pub struct Event {
    description: String,
    timing: String,
    is_done: bool,
}

impl Task for Event {
    fn is_done(&self) -> bool {
        self.is_done
    }
    fn complete(&mut self) {
        self.is_done = true;
    }
}

impl Event {
    pub fn new(description: String, timing: String) -> Self {
        Event {
            description,
            timing,
            is_done: false,
        }
    }
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "[E][{}] {} (at: {})",
            if self.is_done { "√" } else { "X" },
            self.description,
            self.timing,
        )
    }
}

pub struct Deadline {
    description: String,
    deadline: String,
    is_done: bool,
}

impl Task for Deadline {
    fn is_done(&self) -> bool {
        self.is_done
    }
    fn complete(&mut self) {
        self.is_done = true;
    }
}

impl Deadline {
    pub fn new(description: String, deadline: String) -> Self {
        Deadline {
            description,
            deadline,
            is_done: false,
        }
    }
}

impl fmt::Display for Deadline {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "[D][{}] {} (by: {})",
            if self.is_done { "√" } else { "X" },
            self.description,
            self.deadline,
        )
    }
}

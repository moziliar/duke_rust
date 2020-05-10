use std::fmt;

pub trait Task: fmt::Display {
    fn is_done(&self) -> bool;
    fn complete(&mut self);
    fn to_serializable_string(&self) -> String;
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
    fn to_serializable_string(&self) -> String {
        format!(
            "{} | {} | {}\n",
            "T",
            if self.is_done() { "1" } else { "0" },
            self.description
        )
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
    fn to_serializable_string(&self) -> String {
        format!(
            "{} | {} | {} | {}\n",
            "T",
            if self.is_done() { "1" } else { "0" },
            self.description,
            self.timing
        )
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
    fn to_serializable_string(&self) -> String {
        format!(
            "{} | {} | {} | {}\n",
            "T",
            if self.is_done() { "1" } else { "0" },
            self.description,
            self.deadline
        )
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_new_todo_not_done_only_done_after_complete() {
        let mut new_todo = ToDo::new("".to_string());
        assert!(!new_todo.is_done());
        new_todo.complete();
        assert!(new_todo.is_done());
    }

    #[test]
    fn test_new_event_not_done_only_done_after_complete() {
        let mut new_event = Event::new("".to_string(), "".to_string());
        assert!(!new_event.is_done());
        new_event.complete();
        assert!(new_event.is_done());
    }

    #[test]
    fn test_new_deadline_not_done_only_done_after_complete() {
        let mut new_deadline = Deadline::new("".to_string(), "".to_string());
        assert!(!new_deadline.is_done());
        new_deadline.complete();
        assert!(new_deadline.is_done());
    }
}

use std::fmt;

use crate::tasks::task::Task;

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
}

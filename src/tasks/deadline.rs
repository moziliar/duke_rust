use std::fmt;

use chrono::NaiveDateTime;

use crate::tasks::task::Task;

pub struct Deadline {
    description: String,
    deadline: NaiveDateTime,
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
            "D",
            if self.is_done() { "1" } else { "0" },
            self.description,
            self.deadline
        )
    }
}

impl Deadline {
    pub fn new(description: String, deadline: NaiveDateTime) -> Self {
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
            self.deadline.format("%Y-%m-%d %H:%M:%S").to_string(),
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_new_deadline_not_done_only_done_after_complete() {
        let mut new_deadline = Deadline::new(
            "".to_string(),
            NaiveDateTime::parse_from_str("2020-05-11 10:47:00", "%Y-%m-%d %H:%M:%S").unwrap(),
        );
        assert!(!new_deadline.is_done());
        new_deadline.complete();
        assert!(new_deadline.is_done());
    }
}

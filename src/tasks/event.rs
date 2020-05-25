use std::fmt;

use chrono::NaiveDateTime;

use crate::tasks::task::Task;

pub struct Event {
    description: String,
    timing: NaiveDateTime,
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
            "E",
            if self.is_done() { "1" } else { "0" },
            self.description,
            self.timing
        )
    }
}

impl Event {
    pub fn new(description: String, timing: NaiveDateTime) -> Self {
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
            self.timing.format("%Y-%m-%d %H:%M:%S").to_string(),
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_new_event_not_done_only_done_after_complete() {
        let mut new_event = Event::new(
            "".to_string(),
            NaiveDateTime::parse_from_str("2020-05-11 10:47:00", "%Y-%m-%d %H:%M:%S").unwrap(),
        );
        assert!(!new_event.is_done());
        new_event.complete();
        assert!(new_event.is_done());
    }
}

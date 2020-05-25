use std::fmt;

pub trait Task: fmt::Display {
    fn is_done(&self) -> bool;
    fn complete(&mut self);
    fn to_serializable_string(&self) -> String;
}

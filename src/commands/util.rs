extern crate chrono;

use std::error::Error;

use chrono::NaiveDateTime;

use crate::tasks::{deadline::Deadline, event::Event, task::Task, todo::ToDo};
use crate::Command;

pub fn parse_command(input: &str) -> Result<Command, &'static str> {
    match input {
        "bye" => Ok(Command::ByeCommand),
        "list" => Ok(Command::ListCommand),
        _ if input.starts_with("done ") => {
            let mut args = input.split(' ');
            args.next();
            match args.next() {
                Some(int) => match int.parse::<usize>() {
                    Ok(index) => Ok(Command::DoneCommand(index)),
                    Err(_) => Err("Invalid number"),
                },
                None => Err("Invalid command"),
            }
        }
        _ if input.starts_with("todo ")
            || input.starts_with("event ")
            || input.starts_with("deadline ") =>
        {
            Ok(Command::NewTaskCommand(String::from(input)))
        }
        _ if input.starts_with("delete ") => {
            let mut args = input.split(' ');
            args.next();
            match args.next() {
                Some(int) => match int.parse::<usize>() {
                    Ok(index) => Ok(Command::DeleteCommand(index)),
                    Err(_) => Err("Invalid number"),
                },
                None => Err("Invalid command"),
            }
        }
        _ if input.starts_with("find ") => {
            let task = input.replacen("find ", "", 1);
            Ok(Command::FindCommand(task))
        }
        _ => Err("Invalid command"),
    }
}

pub fn parse_new_task_command(task_str: String) -> Result<Box<dyn Task>, Box<dyn Error>> {
    let (task_type, task_string) = parse_task_type(task_str);

    println!("parsing {} {}", task_type, task_string);
    match task_type.as_str() {
        "todo" => Ok(Box::new(ToDo::new(task_string))),
        "event" => {
            let mut iter = task_string.split("/at");
            let event: String = iter.next().unwrap().to_string();
            match iter.next() {
                None => Err("no timing given".into()),
                Some(s) => {
                    let timing = NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S")?;
                    Ok(Box::new(Event::new(event, timing)))
                }
            }
        }
        "deadline" => {
            let mut iter = task_string.split("/by");
            let event: String = iter.next().unwrap().to_string();
            match iter.next() {
                Some(s) => {
                    let timing = NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S")?;
                    Ok(Box::new(Deadline::new(event, timing)))
                }
                None => Err("no timing given".into()),
            }
        }
        _ => Err("unknown type".into()),
    }
}

fn parse_task_type(task_string: String) -> (String, String) {
    let mut iter = task_string.split(' ');
    let task_type = iter.next().unwrap().to_string();
    (task_type, iter.collect::<Vec<&str>>().join(" "))
}

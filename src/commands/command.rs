extern crate chrono;

use std::error::Error;

use chrono::NaiveDateTime;

use crate::common::msg::{DONE_MESSAGE, NO_TASK_MESSAGE, REMOVED_MESSAGE};
use crate::tasks::{deadline::Deadline, event::Event, task::Task, todo::ToDo};

#[derive(PartialEq, Eq, Debug)]
pub enum Command {
    ByeCommand,
    ListCommand,
    NewTaskCommand(String),
    DoneCommand(usize),
    DeleteCommand(usize),
}

pub fn list_tasks(tasks: &[Box<dyn Task>]) -> String {
    if tasks.is_empty() {
        NO_TASK_MESSAGE.to_string()
    } else {
        let mut tasks_string: String = String::new();

        for (ind, task) in tasks.iter().enumerate() {
            tasks_string.push_str(&*format!("{}: {}\n", ind + 1, task));
        }

        tasks_string.trim().to_string()
    }
}

pub fn add_task(
    tasks: &mut Vec<Box<dyn Task>>,
    task_string: String,
) -> Result<String, Box<dyn Error>> {
    match parse_new_task_command(task_string) {
        Ok(task) => {
            let output = format!("added: {}", task);
            tasks.push(task);
            Ok(output)
        }
        Err(e) => Err(e),
    }
}

fn parse_new_task_command(task_str: String) -> Result<Box<dyn Task>, Box<dyn Error>> {
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

pub fn done_task(tasks: &mut Vec<Box<dyn Task>>, index: usize) -> Result<String, &'static str> {
    if index > tasks.len() || index == 0 {
        return Err("index out of bound!");
    }
    assert!(index <= tasks.len() || index > 0);

    if !tasks[index - 1].is_done() {
        tasks[index - 1].complete();
    }
    Ok(format!("{}\n   {}", DONE_MESSAGE, tasks[index - 1]))
}

pub fn delete_task(tasks: &mut Vec<Box<dyn Task>>, index: usize) -> Result<String, &'static str> {
    if index > tasks.len() || index == 0 {
        return Err("index out of bound!");
    }
    assert!(index <= tasks.len() || index > 0);

    let task_removed = tasks.remove(index - 1);
    Ok(format!(
        "{}\n   {}\nNow you have {} tasks in the list.",
        REMOVED_MESSAGE,
        task_removed,
        tasks.len()
    ))
}

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
        _ => Err("Invalid command"),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_list_no_task() {
        let tasks: Vec<Box<dyn Task>> = Vec::new();
        assert_eq!(NO_TASK_MESSAGE.to_string(), list_tasks(&tasks));
    }

    #[test]
    fn test_list_with_tasks() {
        let mut tasks: Vec<Box<dyn Task>> = Vec::new();
        let task_string_1 = "todo test";
        let task_string_2 = "todo test 2";
        let _ = add_task(&mut tasks, task_string_1.to_string());
        let _ = add_task(&mut tasks, task_string_2.to_string());

        assert_eq!(list_tasks(&tasks), "1: [T][X] test\n2: [T][X] test 2");
    }

    #[test]
    fn test_delete_task_from_empty_task_bank() {
        let mut tasks: Vec<Box<dyn Task>> = Vec::new();
        assert_eq!(delete_task(&mut tasks, 1), Err("index out of bound!"));
    }

    #[test]
    fn test_delete_task_from_non_empty_task_bank() {
        let mut tasks: Vec<Box<dyn Task>> = Vec::new();

        let task_string = "todo test";
        let new_task = match parse_new_task_command(task_string.to_string()) {
            Ok(task) => task,
            Err(_) => panic!(""),
        };
        let _ = add_task(&mut tasks, task_string.to_string());
        assert_eq!(
            delete_task(&mut tasks, 1),
            Ok(format!(
                "{}\n   {}\nNow you have {} tasks in the list.",
                REMOVED_MESSAGE,
                new_task,
                tasks.len()
            ))
        );
    }

    // #[test]
    // fn test_delete_task_from_empty_task_bank() {
    //     let mut tasks: Vec<Box<dyn Task>> = Vec::new();
    //     match delete_task(&mut tasks, 1) {
    //         Err(e) => assert_eq!("Index out of bound!", e),
    //         Ok(_) =>  panic!("wrong"),
    //     };
    // }
}

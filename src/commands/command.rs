extern crate chrono;

use std::error::Error;

use crate::commands::util::parse_new_task_command;
use crate::common::msg::{DONE_MESSAGE, NO_TASK_MESSAGE, REMOVED_MESSAGE};
use crate::tasks::task::Task;

#[derive(PartialEq, Eq, Debug)]
pub enum Command {
    ByeCommand,
    ListCommand,
    NewTaskCommand(String),
    DoneCommand(usize),
    DeleteCommand(usize),
    FindCommand(String),
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

#[allow(clippy::borrowed_box)]
pub fn find_task(tasks: &[Box<dyn Task>], task: String) -> Result<&Box<dyn Task>, &'static str> {
    for t in tasks {
        if t.desc() == task {
            return Ok(t);
        }
    }
    Err("No task found")
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

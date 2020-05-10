use crate::task::{Task};
use crate::util::print_formatted_message;
use crate::msg::{
    // BYE_MESSAGE,
    // WELCOME_MESSAGE,
    // INDEX_OUT_OF_BOUND_MESSAGE,
    // INVALID_INPUT_MESSAGE,
    NO_TASK_MESSAGE,
    DONE_MESSAGE,
};

#[derive(PartialEq, Eq, Debug)]
pub enum Command {
    ByeCommand,
    ListCommand,
    NewTaskCommand(String),
    DoneCommand(usize),
}

pub fn list_tasks(tasks: &Vec<Task>) {
    if tasks.len() <= 0 {
        print_formatted_message(NO_TASK_MESSAGE);
    } else {
        let mut tasks_string: String = String::new();
        
        for (ind, task) in tasks.iter().enumerate() {
            tasks_string.push_str(&*format!("{}: {}\n", ind+1, task));
        }

        print_formatted_message(&*tasks_string.trim());
    }
}

pub fn add_task(tasks: &mut Vec<Task>, task: String) {
    tasks.push(Task::new(task.clone()));
    print_formatted_message(format!("added: {}", task).as_str());
}

pub fn done_task(tasks: &mut Vec<Task>, index: usize) -> Result<(), &'static str> {
    if index > tasks.len() || index <= 0 {
        return Err("index out of bound!")
    }
    assert!(index <= tasks.len() || index > 0);

    let is_done = tasks[index-1].is_done;
    if !is_done {
        tasks[index-1].is_done = true;
        print_formatted_message(format!("{}\n   {}", DONE_MESSAGE, tasks[index-1]).as_str());
    } 
    Ok(())
}

pub fn parse_command(input: &str) -> Result<Command, &'static str> {
    match input {
        "bye" => Ok(Command::ByeCommand),
        "list" => Ok(Command::ListCommand),
        _ if input.starts_with("done ") => {
            let mut args = input.split(" ");
            args.next();
            if let Some(int) = args.next() {
                if let Ok(index) = int.parse::<usize>() {
                    Ok(Command::DoneCommand(index))
                } else {
                    Err("Invalid number")
                }
            } else {
                Err("Invalid command")
            }
        }
        _ => Ok(Command::NewTaskCommand(String::from(input))),
    }
}
use crate::task::{ Task, ToDo, Event, Deadline };
use crate::util::print_formatted_message;
use crate::msg::{ NO_TASK_MESSAGE, DONE_MESSAGE, };

#[derive(PartialEq, Eq, Debug)]
pub enum Command {
    ByeCommand,
    ListCommand,
    NewTaskCommand(String),
    DoneCommand(usize),
}

pub fn list_tasks(tasks: &[Box<dyn Task>]) {
    if tasks.is_empty() {
        print_formatted_message(NO_TASK_MESSAGE);
    } else {
        let mut tasks_string: String = String::new();
        
        for (ind, task) in tasks.iter().enumerate() {
            tasks_string.push_str(&*format!("{}: {}\n", ind+1, task));
        }

        print_formatted_message(&*tasks_string.trim());
    }
}

pub fn add_task(tasks: &mut Vec<Box<dyn Task>>, task_string: String) -> Result<(), &'static str> {
    match parse_new_task_command(task_string) {
        Ok(task) => {
            print_formatted_message(format!("added: {}", task).as_str());
            tasks.push(task);
            Ok(())
        },
        Err(e) => Err(e),
    }
}

fn parse_new_task_command(task_str: String) -> Result<Box<dyn Task>, &'static str> {
    let (task_type, task_string) = parse_task_type(task_str);

    println!("parsing {} {}", task_type, task_string);
    match task_type.as_str() {
        "todo" => {
            Ok(Box::new(ToDo::new(task_string)))
        },
        "event" => {
            let mut iter = task_string.split("/at");
            let event: String = iter.next().unwrap().to_string();
            let timing: String = match iter.next() {
                Some(t) => t.to_string(),
                None => return Err("no timing given")
            };
            Ok(Box::new(Event::new(event, timing)))
        },
        "deadline" => {
            let mut iter = task_string.split("/by");
            let event: String = iter.next().unwrap().to_string();
            let timing: String = iter.next().unwrap().to_string();
            Ok(Box::new(Deadline::new(event, timing)))
        },
        _ => Err("unknown type")
    }
}

fn parse_task_type(task_string: String) -> (String, String) {
    let mut iter = task_string.split(' ');
    let task_type = iter.next().unwrap().to_string();
    (task_type, iter.collect::<String>())
}

pub fn done_task(tasks: &mut Vec<Box<dyn Task>>, index: usize) -> Result<(), &'static str> {
    if index > tasks.len() || index == 0 {
        return Err("index out of bound!")
    }
    assert!(index <= tasks.len() || index > 0);

    if !tasks[index-1].is_done() {
        tasks[index-1].complete();
        print_formatted_message(format!("{}\n   {}", DONE_MESSAGE, tasks[index-1]).as_str());
    } 
    Ok(())
}

pub fn parse_command(input: &str) -> Result<Command, &'static str> {
    match input {
        "bye" => Ok(Command::ByeCommand),
        "list" => Ok(Command::ListCommand),
        _ if input.starts_with("done ") => {
            let mut args = input.split(' ');
            args.next();
            if let Some(int) = args.next() {
                match int.parse::<usize>() {
                    Ok(index) => Ok(Command::DoneCommand(index)),
                    Err(_) => Err("Invalid number"),
                }
            } else {
                Err("Invalid command")
            }
        },
        _ if input.starts_with("todo ") 
            || input.starts_with("event ")
            || input.starts_with("deadline ") => Ok(Command::NewTaskCommand(String::from(input))),
        _ => Err("Invalid command"),
    }
}

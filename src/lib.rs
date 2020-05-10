use std::fs::{self, File};
use std::io::prelude::*;
use std::io::{self, BufReader};
use std::process;

mod command;
mod msg;
mod task;
mod util;

use command::{add_task, delete_task, done_task, list_tasks, parse_command, Command};
use msg::{BYE_MESSAGE, INDEX_OUT_OF_BOUND_MESSAGE, INVALID_INPUT_MESSAGE, WELCOME_MESSAGE};
use task::{Deadline, Event, Task, ToDo};
use util::print_formatted_message;

static FILEPATH: &str = "./data/storage.txt";

pub fn start() {
    let mut tasks: Vec<Box<dyn Task>> = read_file(FILEPATH);

    print_formatted_message(WELCOME_MESSAGE);

    loop {
        // handle user input
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => (),
            Err(_) => {
                println!("{}", INVALID_INPUT_MESSAGE);
                continue;
            }
        };

        let command: Command = match parse_command(input.trim()) {
            Ok(cmd) => cmd,
            Err(e) => {
                print_formatted_message(e);
                continue;
            }
        };

        // handle commands
        match command {
            Command::ByeCommand => exit(),
            Command::ListCommand => print_formatted_message(list_tasks(&tasks).as_str()),
            Command::NewTaskCommand(task) => match add_task(&mut tasks, task) {
                Ok(msg) => {
                    print_formatted_message(msg.as_str());
                    let _ = write_file(FILEPATH, &tasks);
                }
                Err(s) => {
                    print_formatted_message(s);
                    continue;
                }
            },
            Command::DoneCommand(ind) => match done_task(&mut tasks, ind) {
                Ok(msg) => {
                    print_formatted_message(msg.as_str());
                    let _ = write_file(FILEPATH, &tasks);
                }
                Err(_) => {
                    print_formatted_message(INDEX_OUT_OF_BOUND_MESSAGE);
                    continue;
                }
            },
            Command::DeleteCommand(ind) => match delete_task(&mut tasks, ind) {
                Ok(msg) => {
                    print_formatted_message(msg.as_str());
                    let _ = write_file(FILEPATH, &tasks);
                }
                Err(_) => {
                    print_formatted_message(INDEX_OUT_OF_BOUND_MESSAGE);
                    continue;
                }
            },
        }
    }
}

fn read_file(filepath: &str) -> Vec<Box<dyn Task>> {
    println!("{}", filepath);
    let f = match File::open(filepath) {
        Ok(f) => f,
        Err(_) => match File::create(filepath) {
            Ok(_) => return Vec::new(),
            Err(e) => panic!("Failed to create storage file. {}", e),
        },
    };
    let f = BufReader::new(f);

    let mut tasks: Vec<Box<dyn Task>> = Vec::new();
    for line in f.lines() {
        tasks.push(read_task(line.unwrap()));
    }
    tasks
}

fn read_task(text: String) -> Box<dyn Task> {
    let mut args = text.split('|');
    let task_type = args.next().expect("File corrupted").trim();
    let is_done = args.next().expect("File corrupted").trim();
    let desc = args.next().expect("File corrupted").trim();
    let timing = match args.next() {
        Some(t) => t,
        None => {
            if task_type != 'T'.to_string() {
                panic!("File corrupted");
            }
            "not needed"
        }
    };
    match task_type {
        "T" => {
            let mut todo = ToDo::new(desc.to_string());
            if is_done == "1" {
                todo.complete();
            }
            Box::new(todo)
        }
        "E" => {
            let mut event = Event::new(desc.to_string(), timing.to_string());
            if is_done == "1" {
                event.complete();
            }
            Box::new(event)
        }
        "D" => {
            let mut deadline = Deadline::new(desc.to_string(), timing.to_string());
            if is_done == "1" {
                deadline.complete();
            }
            Box::new(deadline)
        }
        _ => panic!("File corrupted"),
    }
}

fn write_file(filepath: &str, tasks: &[Box<dyn Task>]) -> Result<(), &'static str> {
    let mut serialized_tasks = String::new();

    for task in tasks {
        serialized_tasks.push_str(task.to_serializable_string().as_str());
    }

    fs::write(filepath, serialized_tasks).expect("Failed to write");
    Ok(())
}

fn exit() {
    print_formatted_message(BYE_MESSAGE);
    process::exit(0);
}

extern crate chrono;

use std::error::Error;
use std::fs::{self, File};
use std::io::prelude::*;
use std::io::{self, BufReader, ErrorKind};
use std::process;

use chrono::NaiveDateTime;

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
                    print_formatted_message(s.to_string().as_str());
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
    let f = File::open(filepath).unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create(filepath).unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    let f = BufReader::new(f);

    let mut tasks: Vec<Box<dyn Task>> = Vec::new();
    for line in f.lines() {
        match read_task(line.unwrap()) {
            Ok(t) => tasks.push(t),
            Err(e) => println!("{}", e),
        }
    }
    tasks
}

fn read_task(text: String) -> Result<Box<dyn Task>, Box<dyn Error>> {
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
            Ok(Box::new(todo))
        }
        "E" => {
            let mut event = Event::new(
                desc.to_string(),
                NaiveDateTime::parse_from_str(timing, "%Y-%m-%d %H:%M:%S")?,
            );
            if is_done == "1" {
                event.complete();
            }
            Ok(Box::new(event))
        }
        "D" => {
            let mut deadline = Deadline::new(
                desc.to_string(),
                NaiveDateTime::parse_from_str(timing, "%Y-%m-%d %H:%M:%S")?,
            );
            if is_done == "1" {
                deadline.complete();
            }
            Ok(Box::new(deadline))
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

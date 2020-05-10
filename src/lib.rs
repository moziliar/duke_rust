use std::io;
use std::process;

mod command;
mod msg;
mod task;
mod util;

use command::{add_task, done_task, list_tasks, parse_command, Command};
use msg::{BYE_MESSAGE, INDEX_OUT_OF_BOUND_MESSAGE, INVALID_INPUT_MESSAGE, WELCOME_MESSAGE};
use task::Task;
use util::print_formatted_message;

pub fn start() {
    print_formatted_message(WELCOME_MESSAGE);

    let mut tasks: Vec<Box<dyn Task>> = Vec::new();

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
            Command::ListCommand => list_tasks(&tasks),
            Command::NewTaskCommand(task) => match add_task(&mut tasks, task) {
                Ok(_) => (),
                Err(s) => {
                    print_formatted_message(s);
                    continue;
                }
            },
            Command::DoneCommand(ind) => match done_task(&mut tasks, ind) {
                Ok(_) => (),
                Err(_) => {
                    print_formatted_message(INDEX_OUT_OF_BOUND_MESSAGE);
                    continue;
                }
            },
        }
    }
}

fn exit() {
    print_formatted_message(BYE_MESSAGE);
    process::exit(0);
}

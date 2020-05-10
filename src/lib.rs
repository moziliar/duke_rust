use std::io;
use std::process;

mod msg;
mod command;
mod task;
mod util;

use util::print_formatted_message;
use command::{
    Command, parse_command, list_tasks, add_task, done_task,
};
use task::Task;
use msg::{
    BYE_MESSAGE, WELCOME_MESSAGE,
    INDEX_OUT_OF_BOUND_MESSAGE, INVALID_INPUT_MESSAGE,
};

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
                continue
            }
        };
            
        let command: Command = match parse_command(input.trim()) {
            Ok(cmd) => cmd,
            Err(e) => {
                print_formatted_message(e);
                continue
            },
        };

        // handle commands
        match command {
            Command::ByeCommand => exit(),
            Command::ListCommand => list_tasks(&tasks),
            Command::NewTaskCommand(task) => match add_task(&mut tasks, task) {
                Ok(_) => (),
                Err(s) => {
                    print_formatted_message(s);
                    continue
                }
            },
            Command::DoneCommand(ind) => {
                match done_task(&mut tasks, ind) {
                    Ok(_) => (),
                    Err(_) => {
                        print_formatted_message(INDEX_OUT_OF_BOUND_MESSAGE);
                        continue
                    },
                }
            },
        }
    }
}

fn exit() {
    print_formatted_message(BYE_MESSAGE);
    process::exit(0);
}

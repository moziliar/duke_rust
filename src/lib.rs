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

    let mut tasks: Vec<Task> = Vec::new();

    loop {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => (),
            Err(_) => {
                println!("{}", INVALID_INPUT_MESSAGE);
                continue
            }
        };
            
        let input_trimmed = input.trim();
        // println!("{}", input_trimmed);
        let command: Command = parse_command(input_trimmed).unwrap();
        // println!("=Command==");
        // println!("{:?}", command);
        // println!("==========");

        match command {
            Command::ByeCommand => exit(),
            Command::ListCommand => list_tasks(&tasks),
            Command::NewTaskCommand(task) => add_task(&mut tasks, task),
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

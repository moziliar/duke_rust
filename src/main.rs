use std::io;
use std::process;

static DIVIDER_LENGTH: usize = 30;
static WELCOME_MESSAGE: &str = "Hello! I'm Duke\nWhat can I do for you?";
static INVALID_INPUT_MESSAGE: &str = "Sorry, invalid input.";
static BYE_MESSAGE: &str = "Bye. Hope to see you again soon!";
static NO_TASK_MESSAGE: &str = "Currently no task available.";

#[derive(PartialEq, Eq, Debug)]
enum Command {
    ByeCommand,
    ListCommand,
    NewTaskCommand(String),
}

fn main() {
    start();
}

fn start() {
    print_formatted_message(WELCOME_MESSAGE);


    let mut tasks: Vec<String> = Vec::new();

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
        let command = parse_command(input_trimmed);
        // println!("=Command==");
        // println!("{:?}", command);
        // println!("==========");

        match command {
            Command::ByeCommand => exit(),
            Command::ListCommand => list_tasks(&tasks),
            Command::NewTaskCommand(task) => add_task(&mut tasks, task),
        }
    }
}

fn list_tasks(tasks: &Vec<String>) {
    if tasks.len() <= 0 {
        print_formatted_message(NO_TASK_MESSAGE);
    } else {
        let mut tasks_string: String = String::new();
        
        for (ind, task) in tasks.iter().enumerate() {
            tasks_string.push_str(&*format!("{}: {}\n", ind, task));
        }

        print_formatted_message(&*tasks_string.trim());
    }
}

fn add_task<'a>(tasks: &mut Vec<String>, task: String) {
    tasks.push(task.clone());
    print_formatted_message(format!("added: {}", task).as_str());
}

fn parse_command(input: &str) -> Command {
    match input {
        "bye" => Command::ByeCommand,
        "list" => Command::ListCommand,
        _ => Command::NewTaskCommand(String::from(input)),
    }
}

fn exit() {
    print_formatted_message(BYE_MESSAGE);
    process::exit(0);
}

fn print_formatted_message(msg: &str) {
    print_divider();
    println!("{}", msg);
    print_divider();
}

fn print_divider() {
    println!("{:-<1$}", "", DIVIDER_LENGTH);
}

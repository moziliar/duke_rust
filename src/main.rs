use std::fmt;
use std::io;
use std::process;

static DIVIDER_LENGTH: usize = 30;
static WELCOME_MESSAGE: &str = "Hello! I'm Duke\nWhat can I do for you?";
static INVALID_INPUT_MESSAGE: &str = "Sorry, invalid input.";
static BYE_MESSAGE: &str = "Bye. Hope to see you again soon!";
static NO_TASK_MESSAGE: &str = "Currently no task available.";
static DONE_MESSAGE: &str = "Nice! I've marked this task as done: ";
static INDEX_OUT_OF_BOUND_MESSAGE: &str = "Index given is out of bound! Please try again!";

#[derive(PartialEq, Eq, Debug)]
enum Command {
    ByeCommand,
    ListCommand,
    NewTaskCommand(String),
    DoneCommand(usize),
}

struct Task {
    description: String,
    done: bool,
}

impl Task {
    fn new(desc: String) -> Task {
        Task { description: desc, done: false }
    }
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}] {}", if self.done { "√" } else { "X" }, self.description)
    }
}

fn main() {
    start();
}

fn start() {
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

fn list_tasks(tasks: &Vec<Task>) {
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

fn add_task(tasks: &mut Vec<Task>, task: String) {
    tasks.push(Task::new(task.clone()));
    print_formatted_message(format!("added: {}", task).as_str());
}

fn done_task(tasks: &mut Vec<Task>, index: usize) -> Result<(), &'static str> {
    if index > tasks.len() || index <= 0 {
        return Err("index out of bound!")
    }
    assert!(index <= tasks.len() || index > 0);

    let done = tasks[index-1].done;
    if !done {
        tasks[index-1].done = true;
        print_formatted_message(format!("{}\n   {}", DONE_MESSAGE, tasks[index-1]).as_str());
    } 
    Ok(())
}

fn parse_command(input: &str) -> Result<Command, &'static str> {
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

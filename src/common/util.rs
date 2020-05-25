static DIVIDER_LENGTH: usize = 30;

pub fn print_formatted_message(msg: &str) {
    print_divider();
    println!("{}", msg);
    print_divider();
}

fn print_divider() {
    println!("{:-<1$}", "", DIVIDER_LENGTH);
}

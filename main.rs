use std::io::{self, Write};

fn main() {
    let mut todos = Vec::new();
    loop {
        print!("Enter task (or 'exit'): ");
        io::stdout().flush().unwrap();
        let mut task = String::new();
        io::stdin().read_line(&mut task).unwrap();
        let task = task.trim();
        if task == "exit" { break; }
        todos.push(task.to_string());
    }
    println!("Your tasks: {:?}", todos);
}

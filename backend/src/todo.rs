use db::{create_task, establish_connection, query_task};
use std::env;

fn help() {
    println!("subcommands:");
    println!("    new <title>: create a new task");
    println!("    show: print all tasks");
}

fn new_task(args: &[String]) {
    if args.len() < 1 {
        println!("new: missing <title>");
        help();
        return;
    }

    let conn = establish_connection();
    create_task(&conn, &args[0]);
}

// ANCHOR: show_tasks
fn show_tasks(args: &[String]) {
    if args.len() > 0 {
        println!("show: unexpected argument");
        help();
        return;
    }

    let conn = establish_connection();
    println!("TASKS\n-----");
    for task in query_task(&conn) {
        println!("{}", task.title);
    }
}
// ANCHOR_END: show_tasks

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        help();
        return;
    }

    let subcommand = &args[1];
    match subcommand.as_ref() {
        "new" => new_task(&args[2..]),
        "show" => show_tasks(&args[2..]),
        _ => help(),
    }
}

#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use full_stack::db::models::Task;
use full_stack::db::{query_task, establish_connection};

#[get("/tasks")]
fn tasks_get() -> String {
    "This is a response\n".into()
}

fn main() {
    rocket::ignite()
        .mount("/", routes![tasks_get])
        .launch();
}
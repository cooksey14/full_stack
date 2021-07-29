#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde;

use diesel::query_source;
use rocket_contrib::json::Json;
use backend::db::models::Task;
use backend::db::{query_task, establish_connection};
use full_stack::JsonApiResponse;


#[get("/tasks")]
fn tasks_get() -> Json<JsonApiResponse> {
    let mut response = JsonApiResponse { data: vec![],};

    let conn = establish_connection();
    for db_task in query_task(&conn) {
        let api_task = full_stack::Task {
            id: db_task.id,
            title: db_task.title,
        };
        response.data.push(api_task);
    }
    Json(response)
}



fn main() {
    rocket::ignite()
        .mount("/", routes![tasks_get])
        .launch();
}
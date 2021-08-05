#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde;

use backend::db::models::Task;
use backend::db::{establish_connection, query_task};
use diesel::query_source;
use full_stack::JsonApiResponse;
use rocket_contrib::json::Json;
use rocket_cors::{AllowedHeaders, AllowedOrigins, Error};

#[get("/tasks")]
fn tasks_get() -> Json<JsonApiResponse> {
    let mut response = JsonApiResponse { data: vec![] };

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

//to run backend currently, we have to run with nightly.
//There are a few features that cannot be used on the stable release channel yet.
//Experimental features: #![feature(proc_macro_hygiene, decl_macro)]
fn main() -> Result<(), Error> {
    let allowed_origins = AllowedOrigins::all();

    let cors = rocket_cors::CorsOptions {
        allowed_origins,
        allowed_headers: AllowedHeaders::some(&["Authorization", "Accept"]),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()?;

    rocket::ignite()
        .mount("/", routes![tasks_get])
        .attach(cors)
        .launch();

    Ok(())
}



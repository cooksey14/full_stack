#[macro_use]
extern crate seed;
use futures::Future;
use seed::prelude::*;
use seed::{fetch, Request};

use full_stack::{JsonApiResponse, Task};

struct Model {
    tasks: Vec<Task>,
}

#[derive(Clone, Debug)]
enum Msg {
    FetchedTasks(fetch::ResponseDataResult<JsonApiResponse>),
}

fn update(msg: Msg, model: &mut Model, _orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::FetchedTasks(Ok(mut result)) => {
            model.tasks.clear();
            model.tasks.append(&mut result.data);
        }
        Msg::FetchedTasks(Err(reason)) => {
            log!(format!("Error fetching: {:?}", reason));
        }
    }
}

//View function that displays the tasks on the frontend
fn view(model: &Model) -> impl View<Msg> {
    let tasks: Vec<Node<Msg>> = model
        .tasks
        .iter()
        .map(|t| li![{ t.title.clone() }])
        .collect();

    h1![{ "Tasks" }, ul![tasks,],]
}

//fetch tasks from the backend
fn fetch_drills() -> impl Future<Output = Result<Msg, Msg>> {
    Request::new("http://localhost:8000/tasks/").fetch_json_data(Msg::FetchedTasks)
}

fn init(_url: Url, orders: &mut impl Orders<Msg>) -> AfterMount<Model> {
    orders.perform_cmd(fetch_drills());
    AfterMount::new(Model { tasks: vec![] })
}

#[wasm_bindgen(start)]
pub fn render() {
    seed::App::builder(update, view)
        .after_mount(init)
        .build_and_start();
}

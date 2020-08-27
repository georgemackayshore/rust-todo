#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;

mod counter;
mod database;
mod todo;

use counter::Counter;
use database::Db;
use rocket::State;
use rocket_contrib::json::{Json, JsonValue};
use todo::Todo;

#[get("/todos", format = "json")]
fn get_all_todos(db: State<Db>) -> JsonValue {
    let map = db.lock().expect("mutex lock");

    let values: Vec<&Todo> = map.values().collect::<Vec<_>>();
    json!(values)
}

#[post("/todos", format = "json", data = "<todo>")]
fn new_todo(db: State<Db>, counter: State<Counter>, todo: Json<Todo>) -> JsonValue {
    let mut map = db.lock().expect("mutex lock");
    let id = counter.increment();

    let new_todo = Todo::new(id, todo.get_name().clone(), false);
    let returned_value = json!(new_todo);
    map.insert(id, new_todo);

    returned_value
}

#[get("/todos/<id>", format = "json")]
fn get_todo(db: State<Db>, id: u32) -> Option<JsonValue> {
    let map = db.lock().unwrap();

    map.get(&id).map(|todo| json!(todo))
}

#[put("/todos/<id>", format = "json", data = "<new_todo>")]
fn update_todo(db: State<Db>, id: u32, new_todo: Json<Todo>) -> Option<JsonValue> {
    let mut map = db.lock().unwrap();
    if map.contains_key(&id) {
        let todo = Todo::new(id, new_todo.0.get_name().clone(), new_todo.0.is_complete());
        let old_todo = map.insert(id, todo).unwrap();
        Some(json!(old_todo))
    } else {
        None
    }
}

#[delete("/todos/<id>")]
fn delete_todo(db: State<Db>, id: u32) -> Option<JsonValue> {
    let mut map = db.lock().unwrap();

    map.remove(&id).map(|todo| json!(todo))
}

#[catch(404)]
fn not_found() -> JsonValue {
    json!({ "status": 404, "error": "id not found" })
}

fn main() {
    rocket::ignite()
        .mount(
            "/",
            routes![get_all_todos, new_todo, get_todo, update_todo, delete_todo],
        )
        .manage(database::create_map())
        .manage(counter::create_counter())
        .register(catchers![not_found])
        .launch();
}

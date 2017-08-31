use rocket_contrib::Json;
use diesel::prelude::*;
use diesel::insert;

use todo::{Todo, NewTodo};
use db;
use schema::todos;

#[get("/todos")]
pub fn get_todos(
  conn: db::DbConn
) -> QueryResult<Json<Vec<Todo>>> {
  todos::dsl::todos.load::<Todo>(&**conn)
    .map(Json)
}

#[get("/todos/<todo>")]
pub fn get_todo(
  todo: Todo
) -> Json<Todo> {
  Json(todo)
}

#[post("/todos", data="<todo_data>")]
pub fn create_todo(
  conn: db::DbConn,
  todo_data: Json<NewTodo>
) -> QueryResult<Json<Todo>> {
  insert(&*todo_data)
    .into(todos::table)
    .get_result(&**conn)
    .map(Json)
}

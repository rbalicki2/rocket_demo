use rocket_contrib::Json;
use diesel::prelude::*;
use diesel::{insert, update};

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

#[patch("/todos/<todo>", data="<todo_data>")]
pub fn update_todo(
  todo: Todo,
  todo_data: Json<NewTodo>,
  conn: db::DbConn,
) -> QueryResult<Json<Todo>> {
  update(todos::dsl::todos.find(todo.id))
    .set(&*todo_data)
    .get_result(&**conn)
    .map(Json)
}

use rocket_contrib::Json;
use diesel::prelude::*;

use todo::Todo;
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

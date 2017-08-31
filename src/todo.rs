use rocket::request::FromParam;
use rocket::http::RawStr;
use diesel::prelude::*;

use schema::todos;
use db;

// Our basic type that we retrieve from the database
#[derive(Queryable, Serialize, Deserialize, Identifiable, Clone, Debug)]
pub struct Todo {
  pub id: i32,
  pub text: String,
  pub completed: bool,
}

// This is the workhorse that allows us to directly receive a Todo
// in our request handler. The from_param method establishes a connection
// to the DB, and then retrieves the Todo.
impl<'a> FromParam<'a> for Todo {
  type Error = ();
  fn from_param(param: &'a RawStr) -> Result<Todo, Self::Error> {
    use schema::todos::dsl::todos;
    let conn = db::establish_connection()
      .map_err(|_| ())?;
    let id: i32 = str::parse::<i32>(param)
      .map_err(|_| ())?;

    let todo: Todo = todos
      .find(id)
      .first::<Todo>(&conn)
      .map_err(|_| ())?;

    Ok(todo)
  }
}

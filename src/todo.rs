use schema::todos;

// Our basic type that we retrieve from the database
#[derive(Queryable, Serialize, Deserialize, Identifiable, Clone, Debug)]
pub struct Todo {
  pub id: i32,
  pub text: String,
  pub completed: bool,
}

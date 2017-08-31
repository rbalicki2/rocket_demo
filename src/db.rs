use rocket::request::{FromRequest, Request, self};
use rocket::{Outcome, State};
use rocket::http::Status;

use diesel::pg::PgConnection;
use r2d2::{Pool, Config};
use std;
use std::ops::Deref;

use r2d2::PooledConnection;
use r2d2_diesel::ConnectionManager;

use diesel::{Connection, ConnectionResult};

pub type ConnectionPool = Pool<ConnectionManager<PgConnection>>;
pub type DbConnection = PooledConnection<ConnectionManager<PgConnection>>;

// we need to define our own type
pub struct DbConn (pub DbConnection);

pub fn establish_connection_pool() -> ConnectionPool {
  let config = Config::default();
  let connection_manager = ConnectionManager::<PgConnection>::new(std::env::var("DATABASE_URL").unwrap());
  Pool::new(config, connection_manager).unwrap()
}

pub fn establish_connection() -> ConnectionResult<PgConnection> {
  let database_url = std::env::var("DATABASE_URL").unwrap();
  PgConnection::establish(&database_url)
}

impl<'a, 'r> FromRequest<'a, 'r> for DbConn {
  type Error = ();

  fn from_request(request: &'a Request<'r>) -> request::Outcome<DbConn, ()> {
    let pool: State<ConnectionPool> = request.guard::<State<ConnectionPool>>()?;
    match pool.get() {
      Ok(conn) => Outcome::Success(DbConn(conn)),
      Err(_) => Outcome::Failure((Status::ServiceUnavailable, ()))
    }
  }
}

impl Deref for DbConn {
  type Target = DbConnection;
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

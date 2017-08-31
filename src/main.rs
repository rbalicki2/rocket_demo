#![plugin(rocket_codegen)]
#![feature(plugin, custom_attribute, custom_derive)]
#![recursion_limit = "1024"]

// https://github.com/SergioBenitez/Rocket/issues/174
#![allow(unmounted_route)]

extern crate rocket;
extern crate rocket_contrib;

#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_codegen;

#[macro_use] extern crate serde_derive;
extern crate serde_json;
extern crate serde;

extern crate r2d2;
extern crate r2d2_diesel;
extern crate dotenv;

extern crate chrono;

use rocket::Route;
use dotenv::dotenv;

pub mod db;
pub mod schema;
pub mod routes;
pub mod todo;

fn main() {
  let routes: Vec<Route> = routes![
    routes::get_todos,
    routes::get_todo,
    routes::create_todo,
    routes::update_todo,
  ];

  dotenv().ok();
  rocket::ignite()
    .manage(db::establish_connection_pool())
    .mount("/", routes)
    .launch();
}
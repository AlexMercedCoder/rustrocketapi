#![feature(proc_macro_hygiene, decl_macro)]

// IMPORT ROCKET LIBRARY
#[macro_use] extern crate dotenv;
#[macro_use] extern crate rocket;
#[macro_use] extern crate serde;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate serde_json;
#[macro_use] extern crate postgres;

pub mod routes;
pub mod personroutes;
use dotenv::dotenv;
use std::env;


// our main function, the entry to our application starting the server, loading routes
fn main() {


    let uri = env::var("DATABASE_URL2");
    print!("{:?}", uri.ok());

    rocket::ignite()
    .mount("/", routes![routes::index, routes::cheese, routes::queso])
    .mount("/people", routes![personroutes::index, personroutes::create, personroutes::update])
    .launch();
}

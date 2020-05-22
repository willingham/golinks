#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

pub mod models;
pub mod routes;
pub mod schema;

#[database("golinks")]
pub struct DbConn(diesel::SqliteConnection);

fn main() {
    rocket::ignite()
        .mount("/", routes![
            routes::index,
            routes::list_links,
            routes::go,
            routes::create_route,
            routes::delete,
        ])
        .attach(DbConn::fairing())
        .launch();
}
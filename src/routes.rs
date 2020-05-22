use diesel::{self, prelude::*};

use rocket::response::Redirect;
use rocket_contrib::json::Json;

use crate::models::{InsertableRoute, Route};
use crate::schema;
use crate::DbConn;

#[get("/")]
pub fn index() -> &'static str {
    "Welcome to GoLinks in Rust."
}

#[get("/<go_slug>")]
pub fn go(conn: DbConn, go_slug: String) -> Result<Redirect, String> {
    // Redirects request according to route
    use crate::schema::routes::dsl::*;
    let route = routes.filter(slug.eq(go_slug))
    .first::<Route>(&conn.0).map_err(|err| -> String {
        println!("Error slug not found: {:?}", err);
        "Error slug not found".into()
    })?;
    Ok(Redirect::to(route.destination))
}


#[post("/<go_slug>", data = "<destination>")]
pub fn create_route(
    conn: DbConn,
    go_slug: String,
    destination: String,
) -> Result<String, String> {
    // Creates new route in the database
    let route = InsertableRoute{slug: go_slug, destination};
    let inserted_rows = diesel::insert_into(schema::routes::table)
        .values(route)
        .execute(&conn.0)
        .map_err(|err| -> String {
            println!("Error inserting row: {:?}", err);
            "Error inserting row into database".into()
        })?;

    Ok(format!("Inserted {} row(s).", inserted_rows))
}


#[delete("/<go_slug>")]
pub fn delete(conn: DbConn, go_slug: String) -> Result<String, String> {
    // Deletes existing route
    use crate::schema::routes::dsl::*;
    
    let num_deleted = diesel::delete(routes.filter(slug.like(go_slug)))
    .execute(&conn.0)
    .map_err(|err| -> String {
        println!("Error deleting slug: {:?}", err);
        "Error deleting slug".into()
    })?;
    Ok(format!("Successfully deleted: {:?}", num_deleted))
}


#[get("/links")]
pub fn list_links(conn: DbConn) -> Result<Json<Vec<Route>>, String> {
    // Returns a JSON list of existing links(routes)
    use crate::schema::routes::dsl::*;

    routes.load(&conn.0).map_err(|err| -> String {
        println!("Error querying page views: {:?}", err);
        "Error querying page views from the database".into()
    }).map(Json)
}
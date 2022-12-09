mod routes;
mod models;
mod database;
#[macro_use]
extern crate rocket;

use rocket::serde::json::{json,Value};



#[launch]
async fn rocket() -> _ {
    rocket::build()
    .attach(database::module::DataBase::fairing())
    .mount("/", routes![routes::get_teams::route,routes::get_match::route,routes::get_matches::route,routes::get_sort_types::route,db_start])
}


#[get("/init")]
async fn db_start(conn: database::module::DataBase) -> Value{
    let result = conn.run(move |c| database::module::init(c)).await;
    match result {
        Ok(_) => json!({ "status": "ok"}),
        Err(err) => json!({ "status": "error", "description": err})
    }
}

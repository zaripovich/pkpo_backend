mod routes;
mod models;
mod database;
#[macro_use]
extern crate rocket;

use rocket::serde::json::{json,Value};
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::{Request, Response};



#[launch]
async fn rocket() -> _ {
    rocket::build()
    .mount("/", routes![routes::get_teams::route,routes::get_match::route,routes::get_matches::route,routes::get_sort_types::route,db_start])
    .attach(database::module::DataBase::fairing())
    .attach(Cors)
} 


pub struct Cors;

#[rocket::async_trait]
impl Fairing for Cors {
    fn info(&self) -> Info {
        Info {
            name: "Cross-Origin-Resource-Sharing Fairing",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, PATCH, PUT, DELETE, HEAD, OPTIONS, GET",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}



#[get("/init")]
async fn db_start(conn: database::module::DataBase) -> Value{
  let result = conn.run(move |c| database::module::init(c)).await;
  match result {
      Ok(_) => json!({ "status": "ok"}),
      Err(err) => json!({ "status": "error", "description": err})
  }
}

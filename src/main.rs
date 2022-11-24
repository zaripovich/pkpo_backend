mod routes;
mod init;

#[macro_use]
extern crate rocket;
use rocket::serde::ser::StdError;


thread_local!(static MATCHES: Result<Vec<init::Match>,Box<dyn StdError>>= init::init());

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![routes::get_teams::route,routes::get_match::route])
}

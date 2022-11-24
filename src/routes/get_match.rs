use rocket::serde::json::Value;
use std::error::Error;
use rocket::serde::json::json;

use crate::MATCHES;
use crate::init::Match;


#[get("/getMatch/<id>")]
pub fn route(id:usize) -> Value {
    let json_:Value = MATCHES.with(|matches| {
        let result: Result<&Match,Box<dyn Error>> = match matches{
            Ok(result)=> Ok(result.get(id).unwrap()),
            Err(why) => Err(From::from(why.to_string())),
        };
        match result{
            Ok(ok)=> {
                let value = rocket::serde::json::to_value(&ok);
                json!({ "status": "ok", "match": value.unwrap()})
            },
            Err(error) => json!({"status": "error", "description": error.to_string()}),
        }
    });
    json_
}
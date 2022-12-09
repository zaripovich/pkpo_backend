
use rocket::serde::json::{json,Value};
use crate::database::module;


#[get("/getMatch/<id>")]
pub async fn route(conn: module::DataBase,id:i32) -> Value {
  let result = conn.run(move |c| module::get_match(c,id)).await;
  match result{
    Ok(ok)=> {
        let value = rocket::serde::json::to_value(ok);
        json!({ "status": "ok", "match": value.unwrap()})
    },
    Err(error) => json!({"status": "error", "description": error.to_string()}),
  }
}
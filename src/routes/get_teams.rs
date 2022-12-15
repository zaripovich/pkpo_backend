
use rocket::serde::json::{json,Value};
use crate::database::module;

#[get("/getTeams")]
pub async fn route(conn: module::DataBase) -> Value {
  let result =conn.run(|c| module::get_all_teams(c)).await;
  match result{
      Ok(ok)=> json!({ "status": "ok", "teams": ok}),
      Err(error) => json!({"status": "error", "description": error.to_string()}),
  }
}

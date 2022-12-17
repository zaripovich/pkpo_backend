
use rocket::serde::Deserialize;
use rocket::serde::json::{json,Json,Value};
use crate::models::SortType;
use crate::database::module;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct SearchParameters {
  pub count: i64,
  pub offset: i64,
  pub sort_type:i32,
  #[serde(default)]
  pub both_team:Option<bool>,
  #[serde(default)]
  pub teams_for_sort:Option<Vec<String>>
}

#[post("/getMatches", data = "<parameters>")]
pub async fn route(conn: module::DataBase, parameters: Json<SearchParameters>) -> Value {
  let result = conn.run(move |c| module::DataProcessor::get_matches_preview(c,parameters.count, parameters.offset, SortType::try_from(parameters.sort_type).unwrap(), parameters.teams_for_sort.clone(), parameters.both_team)).await;
  match result{
    Ok(ok)=> {
        let value = rocket::serde::json::to_value(&ok);
        json!({ "status": "ok", "matches": value.unwrap()})
    },
    Err(error) => json!({"status": "error", "description": error.to_string()}),
  }
}
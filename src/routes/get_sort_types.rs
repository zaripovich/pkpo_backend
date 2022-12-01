use rocket::serde::json::Value;
use rocket::serde::json::json;
use serde::Serialize;
use crate::init::SortType;
use strum::IntoEnumIterator;

#[derive(Serialize)]
struct TypeOfSort{
    pub name:String,
    pub id:u32
}

#[get("/getSortTypes")]
pub fn route() -> Value {
    let mut values = Vec::<TypeOfSort>::new();
    for (index,sort_type) in SortType::iter().enumerate() {
        let value = TypeOfSort{name: sort_type.try_into().unwrap(),id: index as u32};
        values.push(value)
    }
    let result = rocket::serde::json::to_value(values);
    json!({"status":"ok","sort_types":result.unwrap()})
}
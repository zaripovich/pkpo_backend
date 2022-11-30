use rocket::serde::{Deserialize};
use rocket::serde::json::{json,Json,Value};
use crate::MATCHES;
use std::error::Error;
use crate::init::SortType;
use crate::init::Match;


#[derive(Deserialize,FromForm)]
#[serde(crate = "rocket::serde")]
pub struct SearchParameters {
    pub count: u32,
    pub offset: u32,
    pub sort_type:u32,
    pub teams_for_sort:Vec<String>
}


pub fn sort_and_get_matches(mut matches: Vec<Match>,parameters: Json<SearchParameters>)->Vec<Match>{
    if parameters.offset >= matches.len() as u32{
        matches.clear();
        return matches
    }
    for _ in 0..parameters.offset{
        matches.remove(0);
    }
    match parameters.sort_type.try_into(){
        //Ok(SortType::byDate) => matches.sort_by_key(|m| m.match_date),
        Ok(SortType::byTeam) => matches.sort_by_key(|m| parameters.teams_for_sort.contains(&m.team_1) || parameters.teams_for_sort.contains(&m.team_2) ),
        Err(_) => (),
    }
    let mut result = Vec::<Match>::new();
    for (index,element) in matches.iter().enumerate(){
        if index as u32 <= parameters.count {
            result.push(element.clone());
        }else{
            break;
        }
    }
    result
}

#[post("/getMatches", data = "<parameters>")]
pub fn route(parameters: Json<SearchParameters>) -> Value {
    MATCHES.with(|matches| {
        let result:Result<Vec<Match>, Box<dyn Error>> = match matches{
            Ok(result) => Ok(sort_and_get_matches(result.to_vec(),parameters)),
            Err(why) => Err(From::from(why.to_string())),
        };
        match result{
            Ok(ok)=> {
                let value = rocket::serde::json::to_value(&ok);
                json!({ "status": "ok", "matches": value.unwrap()})
            },
            Err(error) => json!({"status": "error", "description": error.to_string()}),
        }
    })
}
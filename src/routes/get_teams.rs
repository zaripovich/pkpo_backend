
use rocket::serde::json::Value;
use std::error::Error;
use rocket::serde::json::json;

use crate::MATCHES;

fn get_all_team() -> Result<Vec<String>, Box<dyn Error>> {
    let mut teams: Vec<String> = Vec::<String>::new();

    MATCHES.with(|matches| {
        match &matches{
            Ok(result)=>
                for element in result{
                    if !!!teams.contains(&element.team_1){
                        teams.push(element.team_1.to_string());
                    }
                    if !!!teams.contains(&element.team_2){
                        teams.push(element.team_2.to_string());
                    }
                },
            Err(why) => println!("error: {}",why),
        };
    });
    teams.sort();
    let result = match teams.len()>0{
        false => Err(From::from("Нет команд!")),
        true => Ok(teams),
    };
    return result;
}

#[get("/getTeams")]
pub fn route() -> Value {
    let teams = get_all_team();
    match teams{
        Ok(ok)=> json!({ "status": "ok", "teams": ok}),
        Err(error) => json!({"status": "error", "description": error.to_string()}),
    }
}

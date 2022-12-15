use crate::models::Match;
use std::error::Error;
use csv::ReaderBuilder;


pub fn init() -> Result<Vec<Match>,Box<dyn Error>>{
  let mut matches: Vec<Match> = Vec::<Match>::new();
  let mut rdr = ReaderBuilder::new().from_path("data/csgo_games.csv")?;
  for record in rdr.deserialize(){
      let record_t: Match = record?;
      matches.push(record_t);
  }
  return Ok(matches);
}


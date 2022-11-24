use serde::{Deserialize,Serialize};
use std::error::Error;
use csv::{ReaderBuilder};


#[derive(Debug, Deserialize,Serialize)]
pub struct Match{
    pub match_date:String,
    pub team_1: String,
    pub team_2: String,
    pub t1_points: u32,
    pub t2_points: u32,
    pub t1_world_rank: u32,
    pub t2_world_rank: u32,
    pub t1_h2h_win_perc: f64,
    pub t2_h2h_win_perc: f64,
    pub winner: String,

    pub t1_player1_rating: f64,
    pub t1_player1_impact: f64,
    pub t1_player1_kdr: f64,
    pub t1_player1_dmr: f64,
    
    pub t1_player2_rating: f64,
    pub t1_player2_impact: f64,
    pub t1_player2_kdr: f64,
    pub t1_player2_dmr: f64,
    
   
    pub t1_player3_rating: f64,
    pub t1_player3_impact: f64,
    pub t1_player3_kdr: f64,
    pub t1_player3_dmr: f64,
    
    pub t1_player4_rating: f64,
    pub t1_player4_impact: f64,
    pub t1_player4_kdr: f64,
    pub t1_player4_dmr: f64,
    
    pub t1_player5_rating: f64,
    pub t1_player5_impact: f64,
    pub t1_player5_kdr: f64,
    pub t1_player5_dmr: f64,
    
    pub t2_player1_rating: f64,
    pub t2_player1_impact: f64,
    pub t2_player1_kdr: f64,
    pub t2_player1_dmr: f64,
    
    pub t2_player2_rating: f64,
    pub t2_player2_impact: f64,
    pub t2_player2_kdr: f64,
    pub t2_player2_dmr: f64,
    
    pub t2_player3_rating: f64,
    pub t2_player3_impact: f64,
    pub t2_player3_kdr: f64,
    pub t2_player3_dmr: f64,
    
    pub t2_player4_rating: f64,
    pub t2_player4_impact: f64,
    pub t2_player4_kdr: f64,
    pub t2_player4_dmr: f64,
    
    pub t2_player5_rating: f64,
    pub t2_player5_impact: f64,
    pub t2_player5_kdr: f64,
    pub t2_player5_dmr: f64,
    #[serde(default)]
    pub id: u32
}



pub fn init() -> Result<Vec<Match>,Box<dyn Error>>{
    let mut matches: Vec<Match> = Vec::<Match>::new();
    let mut rdr = ReaderBuilder::new().from_path("data/csgo_games.csv")?;
    let mut index = 0;
    /* rdr.set_headers(StringRecord::from(vec![
        "match_date",
        "team_1",
        "team_2",
        "t1_points",
        "t2_points",
        "t1_world_rank",
        "t2_world_rank",
        "t1_h2h_win_perc",
        "t2_h2h_win_perc",
        "winner",
        "t1_player1_rating",
        "t1_player1_impact",
        "t1_player1_kdr",
        "t1_player1_dmr",
        "t1_player2_rating",
        "t1_player2_impact",
        "t1_player2_kdr",
        "t1_player2_dmr",
        "t1_player3_rating",
        "t1_player3_impact",
        "t1_player3_kdr",
        "t1_player3_dmr",
        "t1_player4_rating",
        "t1_player4_impact",
        "t1_player4_kdr",
        "t1_player4_dmr",
        "t1_player5_rating",
        "t1_player5_impact",
        "t1_player5_kdr",
        "t1_player5_dmr",
        "t2_player1_rating",
        "t2_player1_impact",
        "t2_player1_kdr",
        "t2_player1_dmr",
        "t2_player2_rating",
        "t2_player2_impact",
        "t2_player2_kdr",
        "t2_player2_dmr",
        "t2_player3_rating",
        "t2_player3_impact",
        "t2_player3_kdr",
        "t2_player3_dmr",
        "t2_player4_rating",
        "t2_player4_impact",
        "t2_player4_kdr",
        "t2_player4_dmr",
        "t2_player5_rating",
        "t2_player5_impact",
        "t2_player5_kdr",
        "t2_player5_dmr"])); */
    for record in rdr.deserialize(){
        let mut record: Match = record?;
        record.id = index;
        //println!("data: {}, {} vs {}\n",record.match_date,record.team_1,record.team_2);
        matches.push(record);
        index+=1
    }
    return Ok(matches);
}
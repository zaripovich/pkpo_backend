use serde::{Deserialize,Serialize};
use std::error::Error;
use csv::{ReaderBuilder};


#[derive(Deserialize,PartialEq)]
#[serde(crate = "rocket::serde")]
pub enum SortType{
    //byDate,
    byTeam
}

impl TryFrom<u32> for SortType {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        //if value == 0 {
        //    return Ok(SortType::byDate)
        //}
        if value == 1 {
            return Ok(SortType::byTeam)
        } 
        Err(())
    }
}

#[derive(Debug, Deserialize,Serialize,Clone)]
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
    for record in rdr.deserialize(){
        let mut record: Match = record?;
        record.id = index;
        //println!("data: {}, {} vs {}\n",record.match_date,record.team_1,record.team_2);
        matches.push(record);
        index+=1
    }
    return Ok(matches);
}
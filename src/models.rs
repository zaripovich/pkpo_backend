use serde::{Deserialize,Serialize};

use rocket_sync_db_pools::postgres;
use strum_macros::EnumIter;


#[derive(Deserialize,PartialEq,EnumIter)]
#[serde(crate = "rocket::serde")]
pub enum SortType{
  ByDate,
  ByTeam,
  ById
}


impl TryFrom<i32> for SortType {
  type Error = ();

  fn try_from(value: i32) -> Result<Self, Self::Error> {
      if value == 0 {
          return Ok(SortType::ByDate)
      }
      if value == 1 {
          return Ok(SortType::ByTeam)
      } 
      if value == 2 {
          return Ok(SortType::ById)
      } 
      Err(())
  }
}

impl TryFrom<SortType> for String {
  type Error = ();

  fn try_from(value: SortType) -> Result<Self, Self::Error> {
      if value == SortType::ByDate {
          return Ok("ByDate".to_string())
      }
      if value == SortType::ByTeam {
          return Ok("ByTeam".to_string())
      }
      if value == SortType::ById {
          return Ok("ById".to_string())
      } 
      Err(())
  }
}

#[derive(Debug, Deserialize,Serialize,Clone)]
pub struct Match{
  #[serde(skip_deserializing)]
  pub id: i32,
  pub match_date: String,
  pub team_1: String,
  pub team_2: String,
  pub t1_points: i32,
  pub t2_points: i32,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub t1_world_rank: Option<i32>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub t2_world_rank: Option<i32>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub t1_h2h_win_perc: Option<f32>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub t2_h2h_win_perc: Option<f32>,
  pub winner: String,

  #[serde(skip_serializing_if = "Option::is_none")]
  pub t1_player1_rating: Option<f32>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub t1_player1_impact: Option<f32>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub t1_player1_kdr: Option<f32>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub t1_player1_dmr: Option<f32>,
  
  #[serde(skip_serializing_if = "Option::is_none")]
  pub t1_player2_rating: Option<f32>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub t1_player2_impact: Option<f32>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub t1_player2_kdr: Option<f32>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub t1_player2_dmr: Option<f32>,
  
  
  #[serde(skip_serializing_if = "Option::is_none")]
  pub t1_player3_rating: Option<f32>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub t1_player3_impact: Option<f32>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub t1_player3_kdr: Option<f32>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub t1_player3_dmr: Option<f32>,
  

  #[serde(skip_serializing_if = "Option::is_none")]
  pub t1_player4_rating: Option<f32>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub t1_player4_impact: Option<f32>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub t1_player4_kdr: Option<f32>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub t1_player4_dmr: Option<f32>,
  

  #[serde(skip_serializing_if = "Option::is_none")]
  pub t1_player5_rating: Option<f32>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub t1_player5_impact: Option<f32>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub t1_player5_kdr: Option<f32>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub t1_player5_dmr: Option<f32>,
  
  #[serde(skip_serializing_if = "Option::is_none")]
  pub t2_player1_rating: Option<f32>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub t2_player1_impact: Option<f32>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub t2_player1_kdr: Option<f32>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub t2_player1_dmr: Option<f32>,
  

  #[serde(skip_serializing_if = "Option::is_none")]
  pub t2_player2_rating: Option<f32>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub t2_player2_impact: Option<f32>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub t2_player2_kdr: Option<f32>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub t2_player2_dmr: Option<f32>,
  
  #[serde(skip_serializing_if = "Option::is_none")]
  pub t2_player3_rating: Option<f32>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub t2_player3_impact: Option<f32>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub t2_player3_kdr: Option<f32>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub t2_player3_dmr: Option<f32>,
  
  #[serde(skip_serializing_if = "Option::is_none")]
  pub t2_player4_rating: Option<f32>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub t2_player4_impact: Option<f32>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub t2_player4_kdr: Option<f32>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub t2_player4_dmr: Option<f32>,
  
  #[serde(skip_serializing_if = "Option::is_none")]
  pub t2_player5_rating: Option<f32>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub t2_player5_impact: Option<f32>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub t2_player5_kdr: Option<f32>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub t2_player5_dmr: Option<f32>
}

impl Match{
  pub fn get_params(&self)->Vec::<&(dyn postgres::types::ToSql + Sync)>{
    let mut params = Vec::<&(dyn postgres::types::ToSql + Sync)>::new();
    params.push(&self.match_date);
    params.push(&self.team_1);
    params.push(&self.team_2);
    params.push(&self.t1_points);
    params.push(&self.t2_points);
    params.push(&self.t1_world_rank);
    params.push(&self.t2_world_rank);
    params.push(&self.t1_h2h_win_perc);
    params.push(&self.t2_h2h_win_perc);
    params.push(&self.winner);

    params.push(&self.t1_player1_rating);
    params.push(&self.t1_player1_impact);
    params.push(&self.t1_player1_kdr);
    params.push(&self.t1_player1_dmr);

    params.push(&self.t1_player2_rating);
    params.push(&self.t1_player2_impact);
    params.push(&self.t1_player2_kdr);
    params.push(&self.t1_player2_dmr);

    params.push(&self.t1_player3_rating);
    params.push(&self.t1_player3_impact);
    params.push(&self.t1_player3_kdr);
    params.push(&self.t1_player3_dmr);

    params.push(&self.t1_player4_rating);
    params.push(&self.t1_player4_impact);
    params.push(&self.t1_player4_kdr);
    params.push(&self.t1_player4_dmr);

    params.push(&self.t1_player5_rating);
    params.push(&self.t1_player5_impact);
    params.push(&self.t1_player5_kdr);
    params.push(&self.t1_player5_dmr);

    params.push(&self.t2_player1_rating);
    params.push(&self.t2_player1_impact);
    params.push(&self.t2_player1_kdr);
    params.push(&self.t2_player1_dmr);

    params.push(&self.t2_player2_rating);
    params.push(&self.t2_player2_impact);
    params.push(&self.t2_player2_kdr);
    params.push(&self.t2_player2_dmr);

    params.push(&self.t2_player3_rating);
    params.push(&self.t2_player3_impact);
    params.push(&self.t2_player3_kdr);
    params.push(&self.t2_player3_dmr);

    params.push(&self.t2_player4_rating);
    params.push(&self.t2_player4_impact);
    params.push(&self.t2_player4_kdr);
    params.push(&self.t2_player4_dmr);

    params.push(&self.t2_player5_rating);
    params.push(&self.t2_player5_impact);
    params.push(&self.t2_player5_kdr);
    params.push(&self.t2_player5_dmr);
    
    return params;
  }

  pub fn preview(_id:i32,_match_date: String, _team_1: String, _team_2: String,_t1_points:i32,_t2_points:i32, _winner: String)-> Match{
    Match { 
      id: _id,
      match_date: _match_date,
      team_1: _team_1,
      team_2: _team_2,
      t1_points: _t1_points,
      t2_points: _t2_points,
      t1_world_rank: None,
      t2_world_rank: None,
      t1_h2h_win_perc: None,
      t2_h2h_win_perc: None,
      winner: _winner,
      t1_player1_rating: None,
      t1_player1_impact: None,
      t1_player1_kdr: None,
      t1_player1_dmr: None,
      t1_player2_rating: None,
      t1_player2_impact: None,
      t1_player2_kdr: None,
      t1_player2_dmr: None,
      t1_player3_rating: None,
      t1_player3_impact: None,
      t1_player3_kdr: None,
      t1_player3_dmr: None,
      t1_player4_rating: None,
      t1_player4_impact: None,
      t1_player4_kdr: None,
      t1_player4_dmr: None,
      t1_player5_rating: None,
      t1_player5_impact: None,
      t1_player5_kdr: None,
      t1_player5_dmr: None,
      t2_player1_rating: None,
      t2_player1_impact: None,
      t2_player1_kdr: None,
      t2_player1_dmr: None,
      t2_player2_rating: None,
      t2_player2_impact: None,
      t2_player2_kdr: None,
      t2_player2_dmr: None,
      t2_player3_rating: None,
      t2_player3_impact: None,
      t2_player3_kdr: None,
      t2_player3_dmr: None,
      t2_player4_rating: None,
      t2_player4_impact: None,
      t2_player4_kdr: None,
      t2_player4_dmr: None,
      t2_player5_rating: None,
      t2_player5_impact: None,
      t2_player5_kdr: None,
      t2_player5_dmr: None }
  }
}



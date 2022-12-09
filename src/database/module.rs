use crate::models::{Match, SortType};
use crate::database::init;
use rocket_sync_db_pools::{postgres,database};
#[database("pkpodb")]
pub struct DataBase(postgres::Client);

fn insert_match(client: &mut postgres::Client,_match:&Match) -> bool{
  let mut query = String::from("INSERT INTO pkpo.matches VALUES(DEFAULT");
  for index in 1..51{
    query.push_str(format!(", ${}",index).as_str());
  }
  query.push_str(");");
  let result = client.execute(query.as_str(),&_match.get_params()[..]);
  match result {
    Ok(_)=> return true,
    Err(err)=>{
      println!("Error: {}",err);
      return false;
    }
  }
}


pub fn init(client: &mut postgres::Client) -> Result<(),String> {
  let queries:Vec<&str> = vec![
    "DROP SCHEMA IF EXISTS pkpo CASCADE;",
    "CREATE SCHEMA pkpo;",
    "CREATE TABLE pkpo.matches( 
      ID INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
      match_date text,
      team_1 text,
      team_2 text,
      t1_points INTEGER,
      t2_points INTEGER,
      t1_world_rank INTEGER,
      t2_world_rank INTEGER,
      t1_h2h_win_perc REAL,
      t2_h2h_win_perc REAL,
      winner VARCHAR(4),
    
      t1_player1_rating REAL,
      t1_player1_impact REAL,
      t1_player1_kdr REAL,
      t1_player1_dmr REAL,
      
      t1_player2_rating REAL,
      t1_player2_impact REAL,
      t1_player2_kdr REAL,
      t1_player2_dmr REAL,
    
    
      t1_player3_rating REAL,
      t1_player3_impact REAL,
      t1_player3_kdr REAL,
      t1_player3_dmr REAL,
    
      t1_player4_rating REAL,
      t1_player4_impact REAL,
      t1_player4_kdr REAL,
      t1_player4_dmr REAL,
    
      t1_player5_rating REAL,
      t1_player5_impact REAL,
      t1_player5_kdr REAL,
      t1_player5_dmr REAL,
    
      t2_player1_rating REAL,
      t2_player1_impact REAL,
      t2_player1_kdr REAL,
      t2_player1_dmr REAL,
    
      t2_player2_rating REAL,
      t2_player2_impact REAL,
      t2_player2_kdr REAL,
      t2_player2_dmr REAL,
    
      t2_player3_rating REAL,
      t2_player3_impact REAL,
      t2_player3_kdr REAL,
      t2_player3_dmr REAL,
    
      t2_player4_rating REAL,
      t2_player4_impact REAL,
      t2_player4_kdr REAL,
      t2_player4_dmr REAL,
    
      t2_player5_rating REAL,
      t2_player5_impact REAL,
      t2_player5_kdr REAL,
      t2_player5_dmr REAL
    );"];
  for query in queries{
    let result = client.batch_execute(query);
    if result.is_err() {
      return Err(result.err().unwrap().to_string());
    }
  }
  let result = init::init();
  match result{
    Ok(r) => {
      for element in r.iter(){
        insert_match(client,element);
      };
      Ok(())
    },
    Err(err) => {
      println!("Error : {}",err);
      Err(err.to_string())
    }
  }
}


pub fn get_all_teams(client: &mut postgres::Client) -> Result<Vec<String>,postgres::Error> {
  let mut teams:Vec<String> = Vec::<String>::new();
  for row in client.query("SELECT team_1, team_2, winner FROM pkpo.matches;",&[])? {
    if !teams.contains(&row.get(0)){
      teams.push(row.get(0));
    }
  }
  teams.sort();
  Ok(teams)
}




pub fn get_matches_preview(client: &mut postgres::Client,count:i64,offset:i64,sort_by: SortType,teams_for_sort: Option<Vec<String>>,both_team: Option<bool>) -> Result<Vec<Match>,postgres::Error>{
  let mut matches:Vec<Match> = Vec::<Match>::new();
  let mut sort_params = String::from("");
  let mut params = Vec::<&(dyn postgres::types::ToSql + Sync)>::new();
  let teams = teams_for_sort.unwrap_or([].to_vec());
  match sort_by {
    SortType::ByDate => sort_params.push_str("ORDER BY match_date"),
    SortType::ByTeam => {
      match both_team{
        Some(v) => {
          if v {
            sort_params.push_str("WHERE team_1 = any($1) AND team_2 = any($2)");
          }else{
            sort_params.push_str("WHERE team_1 = any($1) OR team_2 = any($2)");
          }
        },
        None => {
          sort_params.push_str("WHERE team_1 = any($1) OR team_2 = any($2)");
        }
      }
      params.push(&teams);
      params.push(&teams);
      sort_params.push_str(" ORDER BY match_date")
    },
    SortType::ById => sort_params.push_str("ORDER BY ID")
  }
  let mut query = String::from(format!("SELECT match_date, team_1, team_2, t1_points, t2_points , winner FROM pkpo.matches {} ",sort_params));
  if params.len()==2{
    query.push_str("LIMIT $3 OFFSET $4;");
  }else{
    query.push_str("LIMIT $1 OFFSET $2;");
  }
  params.push(&count);
  params.push(&offset);
  for row in client.query(&query,&params)? {
    let _match = Match::preview(
      row.get(0),
      row.get(1),
      row.get(2),
      row.get(3),
      row.get(4),
      row.get(5)
    );
    matches.push(_match);
  }
  Ok(matches)
}

pub fn get_match(client: &mut postgres::Client,id:i32)->Result<Match,postgres::Error>{
  let row = client.query("SELECT * FROM pkpo.matches WHERE ID=$1 ORDER BY match_date LIMIT 1;",&[&id])?;
  Ok(Match {  match_date:         row[0].get(1), 
            team_1:             row[0].get(2), 
            team_2:             row[0].get(3),
            t1_points:          row[0].get(4), 
            t2_points:          row[0].get(5),
            t1_world_rank:      row[0].get(6),
            t2_world_rank:      row[0].get(7),
            t1_h2h_win_perc:    row[0].get(8),
            t2_h2h_win_perc:    row[0].get(9),
            winner:             row[0].get(10),
            t1_player1_rating:  row[0].get(11),
            t1_player1_impact:  row[0].get(12),
            t1_player1_kdr:     row[0].get(13),
            t1_player1_dmr:     row[0].get(14),
            t1_player2_rating:  row[0].get(15),
            t1_player2_impact:  row[0].get(16),
            t1_player2_kdr:     row[0].get(17),
            t1_player2_dmr:     row[0].get(18),
            t1_player3_rating:  row[0].get(19),
            t1_player3_impact:  row[0].get(20),
            t1_player3_kdr:     row[0].get(21),
            t1_player3_dmr:     row[0].get(22),
            t1_player4_rating:  row[0].get(23),
            t1_player4_impact:  row[0].get(24),
            t1_player4_kdr:     row[0].get(25),
            t1_player4_dmr:     row[0].get(26),
            t1_player5_rating:  row[0].get(27),
            t1_player5_impact:  row[0].get(28),
            t1_player5_kdr:     row[0].get(29),
            t1_player5_dmr:     row[0].get(30),
            t2_player1_rating:  row[0].get(31),
            t2_player1_impact:  row[0].get(32),
            t2_player1_kdr:     row[0].get(33),
            t2_player1_dmr:     row[0].get(34),
            t2_player2_rating:  row[0].get(35),
            t2_player2_impact:  row[0].get(36),
            t2_player2_kdr:     row[0].get(37),
            t2_player2_dmr:     row[0].get(38),
            t2_player3_rating:  row[0].get(39),
            t2_player3_impact:  row[0].get(40),
            t2_player3_kdr:     row[0].get(41),
            t2_player3_dmr:     row[0].get(42),
            t2_player4_rating:  row[0].get(43),
            t2_player4_impact:  row[0].get(44),
            t2_player4_kdr:     row[0].get(45),
            t2_player4_dmr:     row[0].get(46),
            t2_player5_rating:  row[0].get(47),
            t2_player5_impact:  row[0].get(48),
            t2_player5_kdr:     row[0].get(49),
            t2_player5_dmr:     row[0].get(50) }) 
              
}



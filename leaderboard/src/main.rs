use chrono::{DateTime, Utc};
use chrono::serde::ts_seconds;
use serde::{Serialize, Deserialize};

use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

#[derive(Serialize, Deserialize, Debug)]
struct DayLevel {
  #[serde(with = "ts_seconds")]
  get_star_ts: DateTime<Utc>,
  star_index: usize,
}

#[derive(Serialize, Deserialize, Debug)]
struct Member {
  id: usize,
  name: String,
  local_score: usize,
  global_score: usize,
  stars: usize,
  completion_day_level: HashMap<usize, HashMap<usize, DayLevel>>,
  #[serde(with = "ts_seconds")]
  last_star_ts: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Leaderboard {
  members: HashMap<usize, Member>
}

fn main() {
  let mut file = File::open("leaderboard.json").unwrap();
  let mut contents = String::new();
  file.read_to_string(&mut contents).unwrap();

  let leaderboard: Leaderboard = serde_json::from_str(&contents).unwrap();

  let mut lowest_global_index = usize::MAX;
  let mut lowest_global_day = 0;
  let mut lowest_global_level = 0;
  let mut lowest_global_player:String = String::new();
  for (_, member) in leaderboard.members {
    if member.local_score == 0 { continue; }
    println!("{} {}", member.name, member.local_score);
    for (d,l) in member.completion_day_level {
      for (level, details) in l {
        println!("  {d} {level} {} {}", details.get_star_ts, details.star_index);
        if details.star_index < lowest_global_index && details.star_index > 0 {
          lowest_global_index = details.star_index;
          lowest_global_day = d;
          lowest_global_level = level;
          lowest_global_player = member.name.clone();
        }
      }

    }
  }

  println!("best global index {lowest_global_index} by {lowest_global_player} on day {lowest_global_day} challenge {lowest_global_level}");

}

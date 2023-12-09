use chrono::{DateTime, Utc, Duration};
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

  let mut daily_ranks = HashMap::<(usize, usize), Vec<(usize, String, DateTime<Utc>)>>::default();
  for (_, member) in leaderboard.members {
    if member.local_score == 0 { continue; }
    //println!("{} {}", member.name, member.local_score);
    for (d,l) in member.completion_day_level {
      for (level, details) in l {
        //println!("  {d} {level} {} {}", details.get_star_ts, details.star_index);
        if details.star_index < lowest_global_index && details.star_index > 0 {
          lowest_global_index = details.star_index;
          lowest_global_day = d;
          lowest_global_level = level;
          lowest_global_player = member.name.clone();
        }
        let new_entry = (details.star_index, member.name.clone(), details.get_star_ts);
        daily_ranks.entry((d,level)).or_default().push(new_entry);
      }

    }
  }

  for v in daily_ranks.values_mut() {
    v.sort_by(|a,b| a.0.cmp(&b.0));
  }

  println!("best global index {lowest_global_index} by {lowest_global_player} on day {lowest_global_day} challenge {lowest_global_level}");
  println!();
  for day in 1..=25 {
    for level in 1..=2 {
      if let Some(v) = daily_ranks.get(&(day as usize,level as usize)) {
        println!("day {day} challenge {level} entries: {}", v.len());
      }
    }
  }
  println!();
  for day in 1..=25 {
    for level in 1..=2 {
      if let Some(v) = daily_ranks.get(&(day as usize,level as usize)) {
        println!("day {day} challenge {level}");
        for (rank,name,ts) in v {
          println!("{rank} {name} {ts}");
        }
      }
    }
  }
  
  for day in 1..=25 {
    let mut diffs = Vec::<(Duration, String)>::new();
    let mut day1 = HashMap::<String, DateTime<Utc>>::default();
    let mut day2 = HashMap::<String, DateTime<Utc>>::default();
    if let (Some(v), Some(w)) = (daily_ranks.get(&(day as usize, 1)), daily_ranks.get(&(day as usize, 2))) {
      for r in v {
        day1.insert(r.1.clone(), r.2);
      }
      for r in w {
        day2.insert(r.1.clone(), r.2);
      }

      for (name, ts) in day2 {
        diffs.push( (ts - day1.get(&name).unwrap(), name.clone()));
      }


      diffs.sort_by(|a,b|a.0.cmp(&b.0));
      println!("\nday {day} 2nd star duration");
      for (duration, name) in diffs {
        println!("{name} {}m{}s", duration.num_minutes(), duration.num_seconds() % 60);
      }
    }
  }
  
}

use std::fs::File;
use std::io::prelude::*;

enum Move {
  Rock,
  Paper,
  Scissors
}

enum Outcome {
  Lose,
  Draw,
  Win,
}

fn strategy_to_outcome(strat:&str) -> Outcome {
  match strat {
    "X" => Outcome::Lose,
    "Y" => Outcome::Draw,
    "Z" => Outcome::Win,
    &_ => panic!(),
  }
}
//how you should respond to mv to obtain outcome
fn response_for_outcome(outcome:Outcome, mv:Move) -> Move {
  match outcome {
    Outcome::Draw => mv,
    Outcome::Lose => match mv {
      Move::Rock => Move::Scissors,
      Move::Paper=> Move::Rock,
      Move::Scissors => Move::Paper,
    },
    Outcome::Win => match mv {
      Move::Rock => Move::Paper,
      Move::Paper => Move::Scissors,
      Move::Scissors => Move::Rock,
    },
  }
}

fn strategy_to_move(strat:&str) -> Move {
  match strat {
    "A" => Move::Rock,
    "B" => Move::Paper,
    "C" => Move::Scissors,
    "X" => Move::Rock,
    "Y" => Move::Paper,
    "Z" => Move::Scissors,
    &_ => panic!(),
  }
}

fn move_to_value(mv:Move) -> i32 {
  match mv {
  Move::Rock => 1,
  Move::Paper => 2,
  Move::Scissors => 3,
  }
}

/* tells the score for player 1 who makes move 1 */
fn match_score(mv1:Move, mv2:Move) -> i32 {
  
  let m1 = move_to_value(mv1);
  let m2 = move_to_value(mv2);
  return m1 + 3 * ( (4+m1-m2)%3)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match_score() {
        assert_eq!(match_score(Move::Rock, Move::Paper), 1);
        assert_eq!(match_score(Move::Rock, Move::Rock), 4);
        assert_eq!(match_score(Move::Rock, Move::Scissors), 7);
        assert_eq!(match_score(Move::Paper, Move::Paper), 5);
        assert_eq!(match_score(Move::Paper, Move::Rock), 8);
        assert_eq!(match_score(Move::Paper, Move::Scissors), 2);
        assert_eq!(match_score(Move::Scissors, Move::Paper), 3+6);
        assert_eq!(match_score(Move::Scissors, Move::Rock), 3+0);
        assert_eq!(match_score(Move::Scissors, Move::Scissors), 3+3);
    }
}

fn main() -> std::io::Result<()> {
  let mut file = File::open("input.txt")?;
  let mut contents = String::new();
  file.read_to_string(&mut contents)?;


  let mut score:i32 = 0;
  let mut score2:i32 = 0;
  for line in contents.lines() {
    let mut ws_iter = line.split_whitespace();
    let opp = ws_iter.next().expect("bad input");
    let me = ws_iter.next().expect("bad input");
    score += match_score(strategy_to_move(me), strategy_to_move(opp));
    score2 += match_score(
          response_for_outcome( 
              strategy_to_outcome(me), 
              strategy_to_move(opp)),
          strategy_to_move( opp));
  }

  println!("{score}, {score2}");
  Ok(())
}

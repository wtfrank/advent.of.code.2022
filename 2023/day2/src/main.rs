
use std::fs::File;
use std::io::Read;
use clap::Parser;
use log::debug;


use rustc_hash::FxHashMap;
type HashMap<T,U> = FxHashMap<T,U>;

/// Day 19 of Advent of Code 2022
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
   /// Name of the person to greet
   #[arg(short, long, default_value_t=false)]
   benchmark: bool,
}

#[derive(sscanf::FromScanf)]
#[derive(Debug,PartialEq)]
enum BallColour {
    #[sscanf("red")]
    Red,
    #[sscanf("blue")]
    Blue,
    #[sscanf("green")]
    Green,
}


#[derive(Debug)]
struct DrawBall {
  count: u16,
  colour: BallColour,
}

#[derive(Debug)]
struct GameDraw {
  balls: Vec<DrawBall>,
}

#[derive(Debug)]
struct Game{
  id: u16,
  draws: Vec<GameDraw>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
      let line = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
      let game = parse_line(&line);
      assert_eq!(game.id, 1);
      assert_eq!(game.draws.len(), 3);
      assert_eq!(game.draws[0].balls[0].count, 3);
      assert_eq!(game.draws[0].balls[0].colour, BallColour::Blue);

    }

    #[test]
    fn test_load1() {
      let games = load_games( "testinput.txt" );
      let game_score = eval_games(&games);
      assert_eq!(game_score, 8);
    }
    #[test]
    fn test_load2() {
      let games = load_games( "testinput.txt" );
      let game_score = eval_games2(&games);
      assert_eq!(game_score, 2286);
    }
}

fn parse_line( line: &str) -> Game {
  let r = sscanf::sscanf_unescaped!(line, "Game {u16}:{String}").unwrap();
  let id = r.0;
  let game_text = r.1;
  let mut draws = Vec::<GameDraw>::new();
  for draw in game_text.split(';') {
    let mut db = Vec::<DrawBall>::new();
    for ball in draw.split(',') {
      println!("ball: \"{ball}\"");
      let b = sscanf::sscanf_unescaped!( ball, " {u16} {BallColour}" ).unwrap();
      //let b = sscanf::sscanf_unescaped!( draw, " {u16} {String:/red|blue|green/}" ).unwrap();
      println!("{} {} {:?}", r.0, b.0, b.1);
      db.push( DrawBall{ count: b.0, colour: b.1 } );
    }
    draws.push( GameDraw{ balls: db} );
  }
  Game{ id, draws }
}

fn eval_games ( games: &Vec<Game> ) -> u16 {
  let mut possible = 0;
  for game in games {
    let mut valid = true;
    for draw in &game.draws {
      for ball in &draw.balls {
        match ball.colour {
          BallColour::Red => { if ball.count > 12 { valid = false;} },
          BallColour::Green => { if ball.count > 13 { valid = false; } },
          BallColour::Blue => { if ball.count > 14 { valid = false; } },
        }
      }
    }

    if !valid {continue;}
    possible += game.id;
  }
  possible
}

fn eval_games2( games: &Vec<Game> ) -> usize {
  let mut power = 0;
  for game in games {
    let mut min_red = 0;
    let mut min_blue = 0;
    let mut min_green = 0;

    for draw in &game.draws {
      for ball in &draw.balls {
        match ball.colour {
          BallColour::Red => { if ball.count > min_red { min_red = ball.count; } },
          BallColour::Blue => { if ball.count > min_blue { min_blue = ball.count; } },
          BallColour::Green => { if ball.count > min_green { min_green = ball.count; } },
        }
      }
    }

    let p = min_red * min_blue * min_green;
    power += p as usize;

  }

  power
}

fn load_games( filename: &str) -> Vec<Game>
{
  let mut file = File::open(filename).unwrap();
  let mut contents = String::new();
  file.read_to_string(&mut contents).unwrap();

  let mut games = Vec::<Game>::new();
  for line in contents.lines() {
    games.push( parse_line( line ) );
  }

  games
}



fn main() {
    env_logger::init();

    let args = Args::parse();
    if args.benchmark {
      return;
    }

    let games = load_games( "input2.txt" );
    let answer1 = eval_games( &games );
    println!("answer1: {answer1}");
    let answer2 = eval_games2( &games );
    println!("answer2: {answer2}");

}

/*
 * There are 8 pieces of state that need to be tracked - ore, clay, obsidian, geodes, and their
 * associated robots. There are 24 time steps. Because there are so many pieces of state that need
 * to be tracked, a dynamic programming approach may not be efficient.
 * It seems like the right approach might be depth-first search. To reduce the state space, we
 * don't need to simulate each time step individually, instead we can simulate the build sequence,
 * which should reduce the width and depth of the tree, as inventory often won't allow a particular
 * robot to be built. If simulated each timestep individually we would explore lots of cases where
 * we don't build anything for 1 step then build something in the following step where inventory
 * was available in the earlier step. Building later would at best be equal and often worse to
 * building earlier. So we can skip that computation if we only focus on the next item that is
 * built, regardless of how many timesteps there were.
 *
 * Last time I implemented depth-first search, I had a structure which contained the current state
 * and stored that in the stack. This time I'm going to try a minimal structure in the stack, and
 * reuse the state variables. This means when we explore a new state, we update the variables and
 * when we leave the state we reverse that update (e.g. increment inventory on the way in and
 * decrement it on the way out). This means we carry out extra calculation when we leave the state,
 * but it may mean that by avoiding storing copies of the state, we have good cache locality.
 *
 * Note: this was a nicely-posed problem. My implementation took a matter of seconds to complete
 * part 1. I looked at part 2, and it seemed to be simply a matter of changing a few constants
 * and letting it run for a bit longer. But the depth going from 24 -> 32 increases the size of
 * the problem by approximately 4^8. So the runtime extended from a few seconds to about 24 minutes.
 *
 * INTERESTING FINDING:
 * Implementation 1 is substantially faster in debug mode
 * depth 28 method 1 took 5.174142798s
 * depth 28 method 2 took 14.505950026s
 *
 * Implementation 2 is slightly faster in release mode!
 * depth 28 method 1 took 1.448345223s
 * depth 28 method 2 took 1.35526231s
 *
 * Implementation 3 with 2 threads is only a little bit faster. Does hyperthreading ruin it?
 *
 * Implementation 5 with 8 threads per blueprint should prove one way or another
 *
 * Implementation 7 involved creating a thread for each 3-deep subtree of a blueprint evaluation.
 * 15 threads * 3 blueprints = 45 threads.
 *
 * However many of these subtrees didn't contribute much to the total time, so the time ended up
 * being dominated by a few of the heavier subtrees. I ran this on a 4-core machine and a 16-core
 * machine and received relatively similar total times.
 *
 * 4-core:   Intel(R) Core(TM) i7-4770K CPU @ 3.50GHz
 *   real    5m7.207s
 *   user    20m58.596s
 *   sys     0m0.607s
 *
 * 16-core:  Intel(R) Xeon(R) CPU E5-2698 v3 @ 2.30GHz
 *   real    4m46.273s
 *   user    19m10.929s
 *   sys     0m0.040s
 *
 * The 16 core machine was slightly quicker but there wasn't a lot in it!
 *
 */

use clap::Parser;
use log::debug;
use std::fs::File;
use std::io::Read;

/// Day 19 of Advent of Code 2022
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
  /// Name of the person to greet
  #[arg(short, long, default_value_t = false)]
  benchmark: bool,
}

#[derive(Debug, PartialEq, Clone)]
struct Materials {
  ore: u16,
  clay: u16,
  obsidian: u16,
}

#[derive(Debug, PartialEq, Clone)]
struct Blueprint {
  orebot: Materials,
  claybot: Materials,
  obsidianbot: Materials,
  geodebot: Materials,
}

fn parse_line(line: &str) -> Blueprint {
  let r = sscanf::sscanf_unescaped!(line, "Blueprint {u16}: Each ore robot costs {u16} ore. Each clay robot costs {u16} ore. Each obsidian robot costs {u16} ore and {u16} clay. Each geode robot costs {u16} ore and {u16} obsidian.").unwrap();
  Blueprint {
    orebot: Materials {
      ore: r.1,
      clay: 0,
      obsidian: 0,
    },
    claybot: Materials {
      ore: r.2,
      clay: 0,
      obsidian: 0,
    },
    obsidianbot: Materials {
      ore: r.3,
      clay: r.4,
      obsidian: 0,
    },
    geodebot: Materials {
      ore: r.5,
      clay: 0,
      obsidian: r.6,
    },
  }
}

fn load_blueprints(filename: &str) -> Vec<Blueprint> {
  let mut file = File::open(filename).unwrap();
  let mut contents = String::new();
  file.read_to_string(&mut contents).unwrap();
  let mut blueprints = Vec::new();
  for line in contents.lines() {
    blueprints.push(parse_line(line));
  }

  blueprints
}

#[derive(Debug, Default, Clone, Copy)]
struct BpState {
  ore: u16,
  clay: u16,
  obsidian: u16,
  geodes: u16,

  orebots: u16,
  claybots: u16,
  obsidianbots: u16,
  geodebots: u16,

  time: u16,
  available_time: u16,

  action: BotBuildAction, //only used in 2nd implementation
}

#[derive(Debug, Default, Clone, Copy)]
enum BotBuildAction {
  #[default] // this default doesn't make sense but whatevs
  Ore,
  Clay,
  Obsidian,
  Geode,
}

fn build_robot(bp: &Blueprint, s: &mut BpState, most_geodes: &mut u16, action: BotBuildAction) {
  //apply the effects
  // * calc X where X = how many ticks until sufficient materials present
  // * extract for X + 1 ticks
  // * reduce inventory
  // * increment bot count and elapsed time

  let materials: &Materials = match &action {
    BotBuildAction::Ore => &bp.orebot,
    BotBuildAction::Clay => &bp.claybot,
    BotBuildAction::Obsidian => &bp.obsidianbot,
    BotBuildAction::Geode => &bp.geodebot,
  };

  let mut ticks_to_start = 0;
  if materials.ore > 0 && s.ore < materials.ore {
    ticks_to_start = std::cmp::max(
      ticks_to_start,
      num::Integer::div_ceil(&(materials.ore - s.ore), &s.orebots),
    );
  }
  if materials.clay > 0 && s.clay < materials.clay {
    ticks_to_start = std::cmp::max(
      ticks_to_start,
      num::Integer::div_ceil(&(materials.clay - s.clay), &s.claybots),
    );
  }
  if materials.obsidian > 0 && s.obsidian < materials.obsidian {
    ticks_to_start = std::cmp::max(
      ticks_to_start,
      num::Integer::div_ceil(&(materials.obsidian - s.obsidian), &s.obsidianbots),
    );
  }

  /* 1 tick to build, 1 tick to produce. if it hasn't built and produced before we run out of time
   *   then we terminate
   */
  if ticks_to_start + 2 + s.time > s.available_time {
    //println!("time limit reached! ticks_to_start={}, time={}", ticks_to_start, s.time);
    let geodes = s.geodes + (s.available_time - s.time) * s.geodebots;
    if geodes > *most_geodes {
      *most_geodes = geodes;
      debug!("New max geodes: {}", *most_geodes);
    }
    return;
  }

  s.ore += (ticks_to_start + 1) * s.orebots;
  s.clay += (ticks_to_start + 1) * s.claybots;
  s.obsidian += (ticks_to_start + 1) * s.obsidianbots;
  s.geodes += (ticks_to_start + 1) * s.geodebots;

  s.ore -= materials.ore;
  s.clay -= materials.clay;
  s.obsidian -= materials.obsidian;

  s.time += ticks_to_start + 1;

  match &action {
    BotBuildAction::Ore => s.orebots += 1,
    BotBuildAction::Clay => s.claybots += 1,
    BotBuildAction::Obsidian => s.obsidianbots += 1,
    BotBuildAction::Geode => s.geodebots += 1,
  }

  //build something else
  if s.orebots > 0 {
    build_robot(bp, s, most_geodes, BotBuildAction::Ore);
  }
  if s.orebots > 0 {
    build_robot(bp, s, most_geodes, BotBuildAction::Clay);
  }
  if s.orebots > 0 && s.claybots > 0 {
    build_robot(bp, s, most_geodes, BotBuildAction::Obsidian);
  }
  if s.orebots > 0 && s.obsidianbots > 0 {
    build_robot(bp, s, most_geodes, BotBuildAction::Geode);
  }

  /*
    let ore_ok = s.orebots > 0;
    let clay_ok = s.claybots > 0;
    let obsidian_ok = s.obsidianbots > 0;
    if ore_ok {
      build_robot( bp, s, most_geodes, BotBuildAction::Ore );
    }
    if ore_ok {
      build_robot( bp, s, most_geodes, BotBuildAction::Clay );
    }
    if ore_ok && clay_ok {
      build_robot( bp, s, most_geodes, BotBuildAction::Obsidian );
    }
    if ore_ok && obsidian_ok {
      build_robot( bp, s, most_geodes, BotBuildAction::Geode);
    }
  */

  //disapply the effects
  match &action {
    BotBuildAction::Ore => s.orebots -= 1,
    BotBuildAction::Clay => s.claybots -= 1,
    BotBuildAction::Obsidian => s.obsidianbots -= 1,
    BotBuildAction::Geode => s.geodebots -= 1,
  }

  s.time -= ticks_to_start + 1;

  s.ore += materials.ore;
  s.clay += materials.clay;
  s.obsidian += materials.obsidian;

  s.ore -= (ticks_to_start + 1) * s.orebots;
  s.clay -= (ticks_to_start + 1) * s.claybots;
  s.obsidian -= (ticks_to_start + 1) * s.obsidianbots;
  s.geodes -= (ticks_to_start + 1) * s.geodebots;
}

/* A version of build_robot that updates the state but doesn't unapply it or recurse, for setting
 * up threading
 */
fn build_robot_prep_state(bp: &Blueprint, s: &mut BpState, _most_geodes: &mut u16, action: BotBuildAction) {
  //apply the effects
  // * calc X where X = how many ticks until sufficient materials present
  // * extract for X + 1 ticks
  // * reduce inventory
  // * increment bot count and elapsed time

  let materials: &Materials = match &action {
    BotBuildAction::Ore => &bp.orebot,
    BotBuildAction::Clay => &bp.claybot,
    BotBuildAction::Obsidian => &bp.obsidianbot,
    BotBuildAction::Geode => &bp.geodebot,
  };

  let mut ticks_to_start = 0;
  if materials.ore > 0 && s.ore < materials.ore {
    ticks_to_start = std::cmp::max(
      ticks_to_start,
      num::Integer::div_ceil(&(materials.ore - s.ore), &s.orebots),
    );
  }
  if materials.clay > 0 && s.clay < materials.clay {
    ticks_to_start = std::cmp::max(
      ticks_to_start,
      num::Integer::div_ceil(&(materials.clay - s.clay), &s.claybots),
    );
  }
  if materials.obsidian > 0 && s.obsidian < materials.obsidian {
    ticks_to_start = std::cmp::max(
      ticks_to_start,
      num::Integer::div_ceil(&(materials.obsidian - s.obsidian), &s.obsidianbots),
    );
  }

  /* 1 tick to build, 1 tick to produce. if it hasn't built and produced before we run out of time
   *   then we terminate
   */
  if ticks_to_start + 2 + s.time > s.available_time {
    panic!("not supposed to reach endstate from build_robot_prep_state");
  }

  s.ore += (ticks_to_start + 1) * s.orebots;
  s.clay += (ticks_to_start + 1) * s.claybots;
  s.obsidian += (ticks_to_start + 1) * s.obsidianbots;
  s.geodes += (ticks_to_start + 1) * s.geodebots;

  s.ore -= materials.ore;
  s.clay -= materials.clay;
  s.obsidian -= materials.obsidian;

  s.time += ticks_to_start + 1;

  match &action {
    BotBuildAction::Ore => s.orebots += 1,
    BotBuildAction::Clay => s.claybots += 1,
    BotBuildAction::Obsidian => s.obsidianbots += 1,
    BotBuildAction::Geode => s.geodebots += 1,
  }
}

/* version of build_robot using a stack of states instead of pushing/popping changes */
fn build_robot2(bp: &Blueprint, stack: &mut Vec<BpState>, most_geodes: &mut u16, action: BotBuildAction) {
  let s = stack.last().unwrap();
  let mut new_state = *s;
  new_state.action = action;
  //apply the effects
  // * calc X where X = how many ticks until sufficient materials present
  // * extract for X + 1 ticks
  // * reduce inventory
  // * increment bot count and elapsed time

  let materials: &Materials = match &action {
    BotBuildAction::Ore => &bp.orebot,
    BotBuildAction::Clay => &bp.claybot,
    BotBuildAction::Obsidian => &bp.obsidianbot,
    BotBuildAction::Geode => &bp.geodebot,
  };

  let mut ticks_to_start = 0;
  if materials.ore > 0 && s.ore < materials.ore {
    ticks_to_start = std::cmp::max(
      ticks_to_start,
      num::Integer::div_ceil(&(materials.ore - s.ore), &s.orebots),
    );
  }
  if materials.clay > 0 && s.clay < materials.clay {
    ticks_to_start = std::cmp::max(
      ticks_to_start,
      num::Integer::div_ceil(&(materials.clay - s.clay), &s.claybots),
    );
  }
  if materials.obsidian > 0 && s.obsidian < materials.obsidian {
    ticks_to_start = std::cmp::max(
      ticks_to_start,
      num::Integer::div_ceil(&(materials.obsidian - s.obsidian), &s.obsidianbots),
    );
  }

  /* 1 tick to build, 1 tick to produce. if it hasn't built and produced before we run out of time
   *   then we terminate
   */
  if ticks_to_start + 2 + s.time > s.available_time {
    //println!("time limit reached! ticks_to_start={}, time={}", ticks_to_start, s.time);
    let geodes = s.geodes + (s.available_time - s.time) * s.geodebots;
    if geodes > *most_geodes {
      *most_geodes = geodes;
      debug!(
        "New max geodes: {} {:?} ",
        *most_geodes,
        stack
          .iter()
          .map(|s| format!("{:?} ", s.action))
          .collect::<Vec<String>>()
      );
    }
    return;
  }

  new_state.ore += (ticks_to_start + 1) * new_state.orebots;
  new_state.clay += (ticks_to_start + 1) * new_state.claybots;
  new_state.obsidian += (ticks_to_start + 1) * new_state.obsidianbots;
  new_state.geodes += (ticks_to_start + 1) * new_state.geodebots;

  new_state.ore -= materials.ore;
  new_state.clay -= materials.clay;
  new_state.obsidian -= materials.obsidian;

  new_state.time += ticks_to_start + 1;

  match &action {
    BotBuildAction::Ore => new_state.orebots += 1,
    BotBuildAction::Clay => new_state.claybots += 1,
    BotBuildAction::Obsidian => new_state.obsidianbots += 1,
    BotBuildAction::Geode => new_state.geodebots += 1,
  }

  /* reference rules means we can't check the value after adding to the stack */
  let ore_ok = new_state.orebots > 0;
  let clay_ok = new_state.claybots > 0;
  let obsidian_ok = new_state.obsidianbots > 0;

  stack.push(new_state);

  //build something else
  if ore_ok {
    build_robot2(bp, stack, most_geodes, BotBuildAction::Ore);
  }
  if ore_ok {
    build_robot2(bp, stack, most_geodes, BotBuildAction::Clay);
  }
  if ore_ok && clay_ok {
    build_robot2(bp, stack, most_geodes, BotBuildAction::Obsidian);
  }
  if ore_ok && obsidian_ok {
    build_robot2(bp, stack, most_geodes, BotBuildAction::Geode);
  }

  //disapply the effects
  stack.pop();
}

/* in order to get more parallelism, the first level of recursion spawns threads */
fn build_robot5_outer(bp: &Blueprint, stack: &mut Vec<BpState>, most_geodes: &mut u16, action: BotBuildAction) {
  let s = stack.last().unwrap();
  let mut new_state = *s;
  new_state.action = action;
  //apply the effects
  // * calc X where X = how many ticks until sufficient materials present
  // * extract for X + 1 ticks
  // * reduce inventory
  // * increment bot count and elapsed time

  let materials: &Materials = match &action {
    BotBuildAction::Ore => &bp.orebot,
    BotBuildAction::Clay => &bp.claybot,
    BotBuildAction::Obsidian => &bp.obsidianbot,
    BotBuildAction::Geode => &bp.geodebot,
  };

  let mut ticks_to_start = 0;
  if materials.ore > 0 && s.ore < materials.ore {
    ticks_to_start = std::cmp::max(
      ticks_to_start,
      num::Integer::div_ceil(&(materials.ore - s.ore), &s.orebots),
    );
  }
  if materials.clay > 0 && s.clay < materials.clay {
    ticks_to_start = std::cmp::max(
      ticks_to_start,
      num::Integer::div_ceil(&(materials.clay - s.clay), &s.claybots),
    );
  }
  if materials.obsidian > 0 && s.obsidian < materials.obsidian {
    ticks_to_start = std::cmp::max(
      ticks_to_start,
      num::Integer::div_ceil(&(materials.obsidian - s.obsidian), &s.obsidianbots),
    );
  }

  /* 1 tick to build, 1 tick to produce. if it hasn't built and produced before we run out of time
   *   then we terminate
   */
  if ticks_to_start + 2 + s.time > s.available_time {
    //println!("time limit reached! ticks_to_start={}, time={}", ticks_to_start, s.time);
    let geodes = s.geodes + (s.available_time - s.time) * s.geodebots;
    if geodes > *most_geodes {
      *most_geodes = geodes;
      debug!(
        "New max geodes: {} {:?} ",
        *most_geodes,
        stack
          .iter()
          .map(|s| format!("{:?} ", s.action))
          .collect::<Vec<String>>()
      );
    }
    return;
  }

  new_state.ore += (ticks_to_start + 1) * new_state.orebots;
  new_state.clay += (ticks_to_start + 1) * new_state.claybots;
  new_state.obsidian += (ticks_to_start + 1) * new_state.obsidianbots;
  new_state.geodes += (ticks_to_start + 1) * new_state.geodebots;

  new_state.ore -= materials.ore;
  new_state.clay -= materials.clay;
  new_state.obsidian -= materials.obsidian;

  new_state.time += ticks_to_start + 1;

  match &action {
    BotBuildAction::Ore => new_state.orebots += 1,
    BotBuildAction::Clay => new_state.claybots += 1,
    BotBuildAction::Obsidian => new_state.obsidianbots += 1,
    BotBuildAction::Geode => new_state.geodebots += 1,
  }

  /* reference rules means we can't check the value after adding to the stack */
  let ore_ok = new_state.orebots > 0;
  let clay_ok = new_state.claybots > 0;
  let obsidian_ok = new_state.obsidianbots > 0;

  stack.push(new_state);

  //build something else
  if ore_ok {
    //TODO - these calls to build_robot2 are given to threads
    build_robot2(bp, stack, most_geodes, BotBuildAction::Ore);
  }
  if ore_ok {
    build_robot2(bp, stack, most_geodes, BotBuildAction::Clay);
  }
  if ore_ok && clay_ok {
    build_robot2(bp, stack, most_geodes, BotBuildAction::Obsidian);
  }
  if ore_ok && obsidian_ok {
    build_robot2(bp, stack, most_geodes, BotBuildAction::Geode);
  }

  //disapply the effects
  stack.pop();
}

fn evaluate_blueprint(bp: &Blueprint, available_time: u16) -> u16 {
  let mut state = BpState {
    orebots: 1,
    available_time,
    ..Default::default()
  };
  let mut most_geodes = 0;

  debug!("Evaluate blueprint with ore as first action");
  build_robot(bp, &mut state, &mut most_geodes, BotBuildAction::Ore);
  debug!("Evaluate blueprint with clay as first action");
  build_robot(bp, &mut state, &mut most_geodes, BotBuildAction::Clay);

  most_geodes
}

fn evaluate_blueprint2(bp: &Blueprint, available_time: u16) -> u16 {
  let mut stack = Vec::new();
  stack.push(BpState {
    orebots: 1,
    available_time,
    ..Default::default()
  });
  let mut most_geodes = 0;

  debug!("Evaluate blueprint with ore as first action");
  build_robot2(bp, &mut stack, &mut most_geodes, BotBuildAction::Ore);
  debug!("Evaluate blueprint with clay as first action");
  build_robot2(bp, &mut stack, &mut most_geodes, BotBuildAction::Clay);

  most_geodes
}

fn evaluate_blueprint3(bp: &Blueprint, available_time: u16) -> u16 {
  let bp1 = (*bp).clone();
  let a1 = available_time;
  let h1 = std::thread::spawn(move || {
    let mut state = BpState {
      orebots: 1,
      available_time: a1,
      ..Default::default()
    };
    let mut most_geodes = 0;
    build_robot(&bp1, &mut state, &mut most_geodes, BotBuildAction::Ore);
    most_geodes
  });

  let bp2 = (*bp).clone();
  let a2 = available_time;
  let h2 = std::thread::spawn(move || {
    let mut state = BpState {
      orebots: 1,
      available_time: a2,
      ..Default::default()
    };
    let mut most_geodes = 0;
    build_robot(&bp2, &mut state, &mut most_geodes, BotBuildAction::Clay);
    most_geodes
  });

  std::cmp::max(h1.join().unwrap(), h2.join().unwrap())
}

fn evaluate_blueprint4(bp: &Blueprint, available_time: u16) -> u16 {
  let bp1 = (*bp).clone();
  let a1 = available_time;
  let h1 = std::thread::spawn(move || {
    let mut stack = Vec::new();
    stack.push(BpState {
      orebots: 1,
      available_time: a1,
      ..Default::default()
    });
    let mut most_geodes = 0;
    build_robot2(&bp1, &mut stack, &mut most_geodes, BotBuildAction::Ore);
    most_geodes
  });
  let bp2 = (*bp).clone();
  let a2 = available_time;
  let h2 = std::thread::spawn(move || {
    let mut stack = Vec::new();
    stack.push(BpState {
      orebots: 1,
      available_time: a2,
      ..Default::default()
    });
    let mut most_geodes = 0;
    build_robot2(&bp2, &mut stack, &mut most_geodes, BotBuildAction::Clay);
    most_geodes
  });

  std::cmp::max(h1.join().unwrap(), h2.join().unwrap())
}

fn evaluate_blueprint5(bp: &Blueprint, available_time: u16) -> u16 {
  let bp1 = (*bp).clone();
  let a1 = available_time;
  let h1 = std::thread::spawn(move || {
    let mut stack = Vec::new();
    stack.push(BpState {
      orebots: 1,
      available_time: a1,
      ..Default::default()
    });
    let mut most_geodes = 0;
    build_robot5_outer(&bp1, &mut stack, &mut most_geodes, BotBuildAction::Ore);
    most_geodes
  });
  let bp2 = (*bp).clone();
  let a2 = available_time;
  let h2 = std::thread::spawn(move || {
    let mut stack = Vec::new();
    stack.push(BpState {
      orebots: 1,
      available_time: a2,
      ..Default::default()
    });
    let mut most_geodes = 0;
    build_robot5_outer(&bp2, &mut stack, &mut most_geodes, BotBuildAction::Clay);
    most_geodes
  });

  std::cmp::max(h1.join().unwrap(), h2.join().unwrap())
}

/* set up extra threads */
fn evaluate_blueprint6(bp: &Blueprint, available_time: u16) -> u16 {
  let bp1 = (*bp).clone();
  let a1 = available_time;
  let h1 = std::thread::spawn(move || {
    let mut state = BpState {
      orebots: 1,
      available_time: a1,
      ..Default::default()
    };
    let mut most_geodes = 0;
    build_robot_prep_state(&bp1, &mut state, &mut most_geodes, BotBuildAction::Ore);
    build_robot(&bp1, &mut state, &mut most_geodes, BotBuildAction::Ore);
    most_geodes
  });

  let bp1 = (*bp).clone();
  let a1 = available_time;
  let h2 = std::thread::spawn(move || {
    let mut state = BpState {
      orebots: 1,
      available_time: a1,
      ..Default::default()
    };
    let mut most_geodes = 0;
    build_robot_prep_state(&bp1, &mut state, &mut most_geodes, BotBuildAction::Ore);
    build_robot(&bp1, &mut state, &mut most_geodes, BotBuildAction::Clay);
    most_geodes
  });

  let bp1 = (*bp).clone();
  let a1 = available_time;
  let h3 = std::thread::spawn(move || {
    let mut state = BpState {
      orebots: 1,
      available_time: a1,
      ..Default::default()
    };
    let mut most_geodes = 0;
    build_robot_prep_state(&bp1, &mut state, &mut most_geodes, BotBuildAction::Clay);
    build_robot(&bp1, &mut state, &mut most_geodes, BotBuildAction::Ore);
    most_geodes
  });

  let bp1 = (*bp).clone();
  let a1 = available_time;
  let h4 = std::thread::spawn(move || {
    let mut state = BpState {
      orebots: 1,
      available_time: a1,
      ..Default::default()
    };
    let mut most_geodes = 0;
    build_robot_prep_state(&bp1, &mut state, &mut most_geodes, BotBuildAction::Clay);
    build_robot(&bp1, &mut state, &mut most_geodes, BotBuildAction::Clay);
    most_geodes
  });

  let bp1 = (*bp).clone();
  let a1 = available_time;
  let h5 = std::thread::spawn(move || {
    let mut state = BpState {
      orebots: 1,
      available_time: a1,
      ..Default::default()
    };
    let mut most_geodes = 0;
    build_robot_prep_state(&bp1, &mut state, &mut most_geodes, BotBuildAction::Clay);
    build_robot(&bp1, &mut state, &mut most_geodes, BotBuildAction::Obsidian);
    most_geodes
  });

  std::cmp::max(
    std::cmp::max(
      std::cmp::max(h1.join().unwrap(), h2.join().unwrap()),
      std::cmp::max(h3.join().unwrap(), h4.join().unwrap()),
    ),
    h5.join().unwrap(),
  )
}

/* set up extra threads with less boilerplate */
fn evaluate_blueprint7(bp: &Blueprint, available_time: u16) -> u16 {
  use BotBuildAction::*;

  let mut handles = Vec::new();

  //let seqs = vec![ vec![ Ore, Ore], vec![ Ore, Clay ], vec![ Clay, Ore ], vec![ Clay, Clay ], vec![ Clay, Obsidian ] ];
  let seqs = vec![
    vec![Ore, Ore, Ore],
    vec![Ore, Ore, Clay],
    vec![Ore, Clay, Ore],
    vec![Ore, Clay, Clay],
    vec![Ore, Clay, Obsidian],
    vec![Clay, Ore, Ore],
    vec![Clay, Ore, Clay],
    vec![Clay, Ore, Obsidian],
    vec![Clay, Clay, Ore],
    vec![Clay, Clay, Clay],
    vec![Clay, Clay, Obsidian],
    vec![Clay, Obsidian, Ore],
    vec![Clay, Obsidian, Clay],
    vec![Clay, Obsidian, Obsidian],
    vec![Clay, Obsidian, Geode],
  ];

  for seq in seqs {
    let bp1 = (*bp).clone();
    let a1 = available_time;
    let h1 = std::thread::spawn(move || {
      let mut state = BpState {
        orebots: 1,
        available_time: a1,
        ..Default::default()
      };
      let mut most_geodes = 0;
      for s in seq.iter().take(seq.len() - 1) {
        build_robot_prep_state(&bp1, &mut state, &mut most_geodes, *s);
      }
      build_robot(&bp1, &mut state, &mut most_geodes, seq[seq.len() - 1]);
      most_geodes
    });
    handles.push(h1);
  }

  let mut most_geodes = 0;
  for h in handles {
    let mg = h.join().unwrap();
    if mg > most_geodes {
      most_geodes = mg
    }
  }

  most_geodes
}

fn evaluate_all_blueprints(blueprints: &[Blueprint], available_time: u16) -> Vec<u16> {
  let mut effectiveness = Vec::new();
  let mut i = 1;
  for b in blueprints {
    let num_geodes = evaluate_blueprint(b, available_time);
    effectiveness.push(num_geodes);
    println!("Geodes for blueprint {i}: {num_geodes}");
    i += 1;
  }

  effectiveness
}

/* evaluate each blueprint in a separate thread */
fn evaluate_all_blueprints2(blueprints: &[Blueprint], available_time: u16) -> Vec<u16> {
  let mut effectiveness = Vec::new();
  let mut handles = Vec::new();
  for b in blueprints {
    let bp1 = (*b).clone();
    let a1 = available_time;
    let h = std::thread::spawn(move || evaluate_blueprint7(&bp1, a1));
    handles.push(h);
  }

  let mut i = 1;
  for h in handles {
    let num_geodes = h.join().unwrap();
    effectiveness.push(num_geodes);
    println!("Geodes for blueprint {i}: {num_geodes}");
    i += 1;
  }

  effectiveness
}

fn calc_quality_level(effectiveness: &Vec<u16>) -> usize {
  let mut quality_level = 0;
  let mut i = 1;
  for e in effectiveness {
    quality_level += i * (*e) as usize;
    i += 1;
  }
  quality_level
}

fn calc_mult(effectiveness: &Vec<u16>) -> usize {
  assert_eq!(3, effectiveness.len());
  let mut mult = 1;
  for e in effectiveness {
    mult *= (*e) as usize;
  }
  mult
}

fn bm_evalbp(eval_func: fn(&Blueprint, u16) -> u16, i: u16, method: u16, bp: &Blueprint) {
  use std::time::Instant;
  let start = Instant::now();
  let e = eval_func(bp, i);

  let duration = start.elapsed();
  println!("depth {i} method {method} took {duration:?}: {e}");
}

fn benchmark_depth() {
  //  use std::time::{Instant};

  let blueprints = load_blueprints("input19.txt");
  for i in 24..=29 {
    /*
    let start = Instant::now();
    evaluate_blueprint( &blueprints[0], i);
    let duration = start.elapsed();
    println!("depth {i} method 1 took {duration:?}");
      */
    /*
        let start = Instant::now();
        evaluate_blueprint2( &blueprints[0], i);
        let duration = start.elapsed();
        println!("depth {i} method 2 took {duration:?}");

        let start = Instant::now();
        evaluate_blueprint3( &blueprints[0], i);
        let duration = start.elapsed();
        println!("depth {i} method 3 took {duration:?}");

        let start = Instant::now();
        evaluate_blueprint4( &blueprints[0], i);
        let duration = start.elapsed();
        println!("depth {i} method 4 took {duration:?}");
    */
    let bp = &blueprints[0];
    bm_evalbp(evaluate_blueprint2, i, 2, bp);
    bm_evalbp(evaluate_blueprint3, i, 3, bp);
    bm_evalbp(evaluate_blueprint4, i, 4, bp);
    bm_evalbp(evaluate_blueprint5, i, 5, bp);
    bm_evalbp(evaluate_blueprint6, i, 6, bp);
    bm_evalbp(evaluate_blueprint7, i, 7, bp);
  }
}

fn main() {
  env_logger::init();

  let args = Args::parse();
  if args.benchmark {
    benchmark_depth();
    return;
  }

  let blueprints = load_blueprints("input19.txt");
  let e = evaluate_all_blueprints(&blueprints, 24);
  let q = calc_quality_level(&e);
  println!("Quality level: {q}");
  let e = evaluate_all_blueprints2(&blueprints[0..3], 32);
  let m = calc_mult(&e);
  println!("mult: {m}");
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_parse_line() {
    let l = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.";
    let bp = parse_line(l);
    assert_eq!(
      bp,
      Blueprint {
        orebot: Materials {
          ore: 4,
          clay: 0,
          obsidian: 0
        },
        claybot: Materials {
          ore: 2,
          clay: 0,
          obsidian: 0
        },
        obsidianbot: Materials {
          ore: 3,
          clay: 14,
          obsidian: 0
        },
        geodebot: Materials {
          ore: 2,
          clay: 0,
          obsidian: 7
        },
      }
    );
  }

  #[test]
  fn test_evaluate_blueprint_1() {
    let bps = load_blueprints("testinput.txt");
    assert_eq!(evaluate_blueprint(&bps[0], 24), 9);
  }

  #[test]
  fn test_evaluate_blueprint_2() {
    let bps = load_blueprints("testinput.txt");
    assert_eq!(evaluate_blueprint2(&bps[0], 24), 9);
  }

  #[test]
  fn test_evaluate_blueprints() {
    let bps = load_blueprints("testinput.txt");
    let e = evaluate_all_blueprints(&bps[0..], 24);
    assert_eq!(e[0], 9);
    assert_eq!(e[1], 12);
    let q = calc_quality_level(&e);
    assert_eq!(q, 33);
  }
  #[test]
  #[ignore]
  fn test_evaluate_blueprints2() {
    let bps = load_blueprints("testinput.txt");
    let e = evaluate_all_blueprints(&bps, 32);
    assert_eq!(e[1], 62);
  }
}

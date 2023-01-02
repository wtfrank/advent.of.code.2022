use std::fs::File;
use std::io::prelude::*;
//use std::collections::HashSet;
//use std::collections::HashMap;
use rustc_hash::FxHashSet;
use rustc_hash::FxHashMap;
use priority_queue::PriorityQueue;
use std::cmp::Reverse;

type HashSet<T> = FxHashSet<T>;
type HashMap<T,U> = FxHashMap<T,U>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compare() {
      let graph = load_graph("testinput.txt");
      let room_distances = simplify_graph(&graph);
      //check crap rooms aren't present
      assert!(room_distances.get("AA").unwrap().get("FF").is_none());

      assert_eq!(*room_distances.get("AA").unwrap().get("DD").unwrap(), 2);
      assert_eq!(*room_distances.get("AA").unwrap().get("CC").unwrap(), 3);
      assert_eq!(*room_distances.get("CC").unwrap().get("AA").unwrap(), 3);
      assert_eq!(*room_distances.get("DD").unwrap().get("AA").unwrap(), 2);

      let max_pressure = calc_releasable_pressure(&graph, &room_distances, 30);
      assert_eq!(max_pressure, 1651);
      let max_pressure = calc_releasable_pressure_v2(&graph, &room_distances, 26);
      assert_eq!(max_pressure, 1707);
    }
}


fn load_graph(filename: &str) -> HashMap<String,Room>
{
  let mut file = File::open(filename).unwrap();
  let mut contents = String::new();
  file.read_to_string(&mut contents).unwrap();

  let mut graph = HashMap::<String,Room>::default();
  for line in contents.lines() {
    let r = sscanf::sscanf_unescaped!(line, "Valve {String} has flow rate={usize}; tunnels? leads? to valves? {String}");
    match r {
      Err(_) => {panic!("didn't match: {line}.")},
      Ok( (room,flow,tunnels_str) ) => {
      let tunnels:Vec<String> = tunnels_str.split(|c:char| c.is_whitespace() || c == ',').step_by(2).map(|s| String::from(s)).collect();
      println!("tunnels: {room}->{tunnels:?}");
      graph.insert(room, Room{flow,tunnels});
      },
    }
  }

  return graph;
}

struct Room {
  flow:usize,
  tunnels:Vec<String>,
}

#[derive(Debug)]
struct PathStep<'a> {
  room: &'a str,
  open_valves: usize,
  cum_pressure: usize,
  dist: usize,
}



/* all we care about are the starting room and valves with positive flow.
 * So we make a structure, containing the time to reach each room and turn on the valve from
 * each other room.
 * We can then search every possible traversal that takes fewer than 30 minutes with depth
 * first search.
 */
fn simplify_graph(graph:&HashMap<String,Room>) -> HashMap<String, HashMap<String,usize>> {
  let mut useful_rooms = HashSet::<String>::default();
  let mut distances = HashMap::default();

  for (name,room) in graph {
    if name == SOURCE || room.flow > 0 {
      useful_rooms.insert(name.clone());
      distances.insert(name.clone(), HashMap::default());
    }
  }

  println!("{} of {} rooms were useful: {useful_rooms:?}", useful_rooms.len(), graph.len());

  //now we use djikstra's algo to find the distance between each useful room. we could do this
  //more efficiently by exploiting that the forward/reverse directions are of the same length.
  for r1 in &useful_rooms {
    let mut dist = HashMap::<String,usize>::default();
    let mut queue = PriorityQueue::<String,Reverse<usize>>::default();
    let mut visited = HashSet::<String>::default();
    let mut prev = HashMap::<String,String>::default();

    for r2 in graph {
      dist.insert( r2.0.clone(), usize::MAX);
      queue.push( r2.0.clone(), Reverse(usize::MAX));
    }

    dist.insert( String::from(r1), 0 );
    queue.change_priority( r1, Reverse(0) );

    while queue.len() > 0 {
      let u = queue.pop().unwrap().0;
      //println!("visiting {u}");
      visited.insert(u.clone());

      for neighbour in &graph.get(&u).unwrap().tunnels {
        if visited.contains(neighbour) { continue; }
        let alt = dist.get(&u).unwrap() + 1; //same length tunnels;
        if alt < *dist.get(neighbour).unwrap() {
          dist.insert(neighbour.clone(), alt);
          queue.change_priority( neighbour, Reverse(alt) );
          prev.insert(neighbour.clone(), u.clone());
        }
      }
    }

    for r2 in &useful_rooms {
      if r1 == r2 { continue }
      distances.get_mut(r1).unwrap().insert(r2.clone(), *dist.get(r2).unwrap() + 1); //also include the time taken to switch on the valve (otherwise why visit)
    }
  }

  return distances;
}

fn do_calc_pressure<'a>( graph: &HashMap<String,Room>, room_distances: &'a HashMap<String, HashMap<String,usize>>, max_time: usize, path: &mut Vec::<PathStep<'a>>, discovered: &mut HashSet::<String>, highest_pressure: &mut usize, room:&'a str)
{
  //println!("Entering {room} with path length {}", path.len());
  discovered.insert(String::from(room));
  if path.len() <= 0 {
    assert!(room == SOURCE);
    path.push( PathStep{room: room, open_valves:0, cum_pressure: 0, dist: 0} );
  }
  else {
    let prev = path.get(path.len()-1).unwrap();
    let dist = room_distances.get(prev.room).unwrap().get(room).unwrap();
    //println!("distance from {} to {}={}, open_valves={},cum_pressure={},cum dist={}",
//        &prev.room, &room, dist, prev.open_valves, prev.cum_pressure, prev.dist);
    if prev.dist + dist > max_time {
      let total_pressure = prev.cum_pressure + (max_time - prev.dist) * prev.open_valves;
      if total_pressure > *highest_pressure {
        *highest_pressure = total_pressure;
        println!("New best path1: {highest_pressure} via {path:?}");
      }
      discovered.remove(room);
      //println!("Leaving {room} (path len {})", path.len());
      return;
    }
    else {
      path.push( PathStep{
          room: room,
          open_valves: prev.open_valves + graph.get(room).unwrap().flow,
          cum_pressure: prev.cum_pressure + dist * prev.open_valves,
          dist: prev.dist + dist,
      });
    }
  }

  let mut avail = 0;
  for (r2,_) in room_distances.get(room).unwrap() {
    if !discovered.contains(r2)  {
      avail += 1;
      do_calc_pressure( &graph, &room_distances, max_time, path, discovered, highest_pressure, r2 );
    }
  }

  if avail == 0 {
    let prev = path.get(path.len()-1).unwrap();
    let total_pressure = prev.cum_pressure + (max_time - prev.dist) * prev.open_valves;
    if total_pressure > *highest_pressure {
      *highest_pressure = total_pressure;
      println!("New best path2: {highest_pressure} via {path:?}");
    }
  }

  path.pop();
  discovered.remove(room);
  //println!("Leaving {room} (path len {})", path.len());
}


fn do_calc_pressure_v2<'a>( graph: &HashMap<String,Room>, room_distances: &'a HashMap<String, HashMap<String,usize>>, max_time: usize, path: &mut Vec::<PathStep<'a>>, path2: &mut Vec::<PathStep<'a>>, discovered: &mut HashSet::<String>, highest_pressure: &mut usize, room:&'a str)
{
  //println!("Entering {room} with path length {}", path.len());
  discovered.insert(String::from(room));
  if path.len() <= 0 {
    assert!(room == SOURCE);
    path.push( PathStep{room: room, open_valves:0, cum_pressure: 0, dist: 0} );
    path2.push( PathStep{room: room, open_valves:0, cum_pressure: 0, dist: 0} );
  }
  else {
    let prev = path.get(path.len()-1).unwrap();
    let dist = room_distances.get(prev.room).unwrap().get(room).unwrap();
    //println!("distance from {} to {}={}, open_valves={},cum_pressure={},cum dist={}",
//        &prev.room, &room, dist, prev.open_valves, prev.cum_pressure, prev.dist);
    if prev.dist + dist > max_time {
      let prev2 = path2.get(path2.len()-1).unwrap();
      let total_pressure = prev.cum_pressure + (max_time - prev.dist) * prev.open_valves +
                           prev2.cum_pressure+ (max_time - prev2.dist)* prev2.open_valves;
      if total_pressure > *highest_pressure {
        *highest_pressure = total_pressure;
        println!("New best path1: {highest_pressure} via {path:?}_{path2:?}");
      }
      discovered.remove(room);
      //println!("Leaving {room} (path len {})", path.len());
      return;
    }
    else {
      path.push( PathStep{
          room: room,
          open_valves: prev.open_valves + graph.get(room).unwrap().flow,
          cum_pressure: prev.cum_pressure + dist * prev.open_valves,
          dist: prev.dist + dist,
      });
    }
  }


  /* now see which route will next completed, and advance the other route if necessary */
  let next_path1;
  let next_path2;
  let next_room;
  let swapped;
  if path.get(path.len()-1).unwrap().dist > path2.get(path2.len()-1).unwrap().dist {
    next_path1 = path2;
    next_path2 = path;
    next_room = next_path2.get(next_path2.len()-1).unwrap().room;
    swapped = true;
  }
  else {
    next_path1 = path;
    next_path2 = path2;
    next_room = room;
    swapped = false;
  }
  let mut avail = 0;
  for (r2,_) in room_distances.get(next_room).unwrap() {
    if !discovered.contains(r2)  {
      avail += 1;

      do_calc_pressure_v2( &graph, &room_distances, max_time, next_path1, next_path2, discovered, highest_pressure, r2 );
    }
  }

  if avail == 0 {
    let prev = next_path1.get(next_path1.len()-1).unwrap();
    let prev2 = next_path2.get(next_path2.len()-1).unwrap();
    let total_pressure = prev.cum_pressure + (max_time - prev.dist) * prev.open_valves +
                         prev2.cum_pressure+ (max_time - prev2.dist)* prev2.open_valves;
    if total_pressure > *highest_pressure {
      *highest_pressure = total_pressure;
      println!("New best path2: {highest_pressure} via {next_path1:?}!{next_path2:?}");
    }
  }

  if swapped {
    next_path2.pop();
  }
  else {
    next_path1.pop();
  }
  discovered.remove(room);
  //println!("Leaving {room} (path len {})", path.len());
}

fn calc_releasable_pressure( graph: &HashMap<String,Room>, room_distances: &HashMap<String, HashMap<String,usize>>, max_time: usize) -> usize {
  //depth_first_search to explore all possible visit sequences starting from AA
  let mut highest_pressure = 0;

  //each recursion we store the current path, cumulative pressure, time on a stack
  let mut path = Vec::<PathStep>::new();

  let mut discovered = HashSet::<String>::default();

  do_calc_pressure( &graph, &room_distances, max_time, &mut path, &mut discovered, &mut highest_pressure, SOURCE );

  return highest_pressure;
}

fn calc_releasable_pressure_v2( graph: &HashMap<String,Room>, room_distances: &HashMap<String, HashMap<String,usize>>, max_time: usize) -> usize {
  //depth_first_search to explore all possible visit sequences starting from AA
  let mut highest_pressure = 0;

  //each recursion we store the current path, cumulative pressure, time on a stack
  let mut path1 = Vec::<PathStep>::new();
  let mut path2 = Vec::<PathStep>::new();

  let mut discovered = HashSet::<String>::default();

  do_calc_pressure_v2( &graph, &room_distances, max_time, &mut path1, &mut path2, &mut discovered, &mut highest_pressure, SOURCE );

  return highest_pressure;
}


const SOURCE: &str = "AA";

fn main() -> std::io::Result<()> {
  let graph = load_graph("input16.txt");
  let room_distances = simplify_graph(&graph);
  let max_pressure = calc_releasable_pressure(&graph, &room_distances, 30);

  println!("max pressure: {max_pressure}");

  let max_pressure = calc_releasable_pressure_v2(&graph, &room_distances, 26);
  println!("max pressure: {max_pressure}");

  Ok(())
}
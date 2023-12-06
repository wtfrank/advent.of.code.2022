
use std::fs::File;
use std::io::Read;
use clap::Parser;

use advent::{Interval, Overlap};

//use std::collections::HashSet;
use std::cmp::Ordering;

//use rustc_hash::FxHashMap;
//type HashMap<T,U> = FxHashMap<T,U>;

/// Day 3 of Advent of Code 2023
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
   /// Name of the person to greet
   #[arg(short, long, default_value_t=false)]
   benchmark: bool,
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_parse_line() {
  }
  #[test]
  fn test_load1() {
    let almanac = load_data( "testinput.txt" );
    let score1 = find_lowest(&almanac);
    assert_eq!(score1, 35);
  }
  #[test]
  fn test_load2() {
    let almanac = load_data( "testinput.txt" );
    let score2 = find_lowest2(&almanac);
    assert_eq!(score2, 46);
  }
}

struct RangeComponent {
  interval: Interval,
  dest: usize,
}

struct Range {
  _ranges: Vec<RangeComponent>,
}

impl Range {
  fn default() -> Range {
    Range{_ranges: Vec::new() }
  }
  fn get( &self, val: usize ) -> usize {
    for r in self._ranges.iter() {
      if val >= r.interval.start as usize && val < r.interval.end() as usize {
        return val - r.interval.start as usize + r.dest;
      }
    }
    
    val
  }
  fn get_ranges( &self, ranges_in: &[Interval] ) -> Vec<Interval> {
    let mut output = Vec::new();
    let mut ranges = Vec::<Interval>::new();
    for r in ranges_in.iter() {
      ranges.push(*r);
    }

    let mut extra_ranges = Vec::new();
    loop {
      for range in ranges.drain(..) {
        //split up this range as necessary based on internal ranges which are sorted
        let mut range_consumed = false;
        for r2 in &self._ranges {
          let overlap = range.cmp_overlap(&r2.interval);
          //range ends before lowest entry
          if overlap == Overlap::Less {
            output.push(range);
            range_consumed = true;
            break;
          }
          //range starts before but ends within entry
          else if overlap == Overlap::Left {
            let bef = Interval{start: range.start, length: r2.interval.start as usize - range.start as usize};
            let mid = Interval{start: r2.dest as isize, length: range.length - bef.length };
            output.push(bef);
            output.push(mid);
            range_consumed = true;
            break;
          }
          else if overlap == Overlap::Equal {
            let mid = Interval{start: r2.dest as isize, length: r2.interval.length };
            output.push(mid);
            range_consumed = true;
            break;
          }
          //range starts before but ends after entry
          else if overlap == Overlap::Outside {
            let bef = Interval{start: range.start, length: r2.interval.start as usize - range.start as usize};
            let mid = Interval{start: r2.dest as isize, length: r2.interval.length };
            let after = Interval{start: r2.interval.end(), length: range.length - r2.interval.length - bef.length};
            if bef.length > 0 {
              output.push(bef);
            }
            output.push(mid);
            if after.length > 0 {
              extra_ranges.push(after);
            }
            range_consumed = true;
            break;
          }
          //range starts and ends within entry
          else if overlap == Overlap::Inside {
            let mid = Interval{ start: r2.dest as isize + range.start-r2.interval.start, length: range.length };
            output.push(mid);
            range_consumed = true;
            break;
          }
          //range starts within and ends after entry
          else if overlap == Overlap::Right {
            let mid = Interval{ start: r2.dest as isize + range.start-r2.interval.start, length: r2.interval.length - (range.start as usize - r2.interval.start as usize) };
            //println!("range length {}, r2 length {}, rstart {}, r2start {}",
            //  range.length, r2.length, range.start, r2.source);
            let after = Interval{start: r2.interval.end(), length: range.length + (range.start as usize -r2.interval.start as usize ) - r2.interval.length };
            output.push(mid);
            extra_ranges.push(after);
            range_consumed = true;
            break;
          }
          else {
            if range.start < r2.interval.end() { panic!("not all cases covered"); }
            if range.end() < r2.interval.end() { panic!("not all cases covered"); }
            //range starts after entry so we should check against other ranges
          }
        }
        //we've checked all entries and didn't find this range
        if !range_consumed {
          output.push(range);
        }

      }

      if !extra_ranges.is_empty() {
        ranges.append(&mut extra_ranges);
      }
      else {
        break;
      }
    }
    output
  }

  fn is_empty( &self ) -> bool {
    self._ranges.is_empty()
  }
}


struct Almanac {
  seeds: Vec<usize>,
  seed_soil: Range,
  soil_fertiliser: Range,
  fertiliser_water: Range,
  water_light: Range,
  light_temperature: Range,
  temperature_humidity: Range,
  humidity_location: Range,
}

impl Almanac {
  fn new() -> Almanac {
    Almanac {
      seeds: Vec::<usize>::new(),
      seed_soil: Range::default(),
      soil_fertiliser: Range::default(),
      fertiliser_water: Range::default(),
      water_light: Range::default(),
      light_temperature: Range::default(),
      temperature_humidity: Range::default(),
      humidity_location: Range::default(),
    }
  }
}

fn seed_loc( almanac: &Almanac, seed: usize ) -> usize {
  let soil = almanac.seed_soil.get(seed);
  let fertiliser = almanac.soil_fertiliser.get(soil);
  let water = almanac.fertiliser_water.get(fertiliser);
  let light = almanac.water_light.get(water);
  let temperature = almanac.light_temperature.get(light);
  let humidity = almanac.temperature_humidity.get(temperature);
  //println!("seed {seed}, soil {soil}, fert {fertiliser}, water {water}, light {light}, temp {temperature}, humidity {humidity}, loc {location}");

  almanac.humidity_location.get(humidity)
}

fn seed_loc_range( almanac: &Almanac, seedranges: &[Interval] ) -> Vec<Interval> {
  let soil = almanac.seed_soil.get_ranges(seedranges);
  let fertiliser = almanac.soil_fertiliser.get_ranges(&soil);
  let water = almanac.fertiliser_water.get_ranges(&fertiliser);
  let light = almanac.water_light.get_ranges(&water);
  let temperature = almanac.light_temperature.get_ranges(&light);
  let humidity = almanac.temperature_humidity.get_ranges(&temperature);
  
  almanac.humidity_location.get_ranges(&humidity)
}

fn find_lowest2( almanac: &Almanac ) -> isize {

  if almanac.seeds.len() %2 != 0 { panic!("odd number of seed ranges") }

  let mut lowest = isize::MAX;
  let mut it = almanac.seeds.iter();
  let mut start = it.next();
  let mut length = it.next();
  while start.is_some() {
    let s = *start.unwrap();
    let l = *length.unwrap();

    let seedranges = vec![Interval{ start: s as isize, length: l}];

    let locranges = seed_loc_range(almanac, &seedranges);

    for lr in locranges {
      if lr.start < lowest { lowest = lr.start; }
    }

    start = it.next();
    length = it.next();
  }

  lowest
}

fn find_lowest( almanac: &Almanac ) -> usize {
  let mut lowest = usize::MAX;
  for seed in &almanac.seeds {
    let location = seed_loc(almanac, *seed);
    if location < lowest {
      lowest = location;
    }
  }
  lowest
}

enum ParseFields {
  Seeds,
  SeedSoil,
  SoilFertiliser,
  FertiliserWater,
  WaterLight,
  LightTemperature,
  TemperatureHumidity,
  HumidityLocation
}

fn parse_range( line: &str, ranges: &mut Range ) {
  let r = sscanf::sscanf_unescaped!(line, "{usize} {usize} {usize}").unwrap();
  let dest = r.0;
  let source = r.1 as isize;
  let length = r.2;
  ranges._ranges.push( RangeComponent{ interval: Interval{ start: source, length}, dest } );
}

fn sort_range( ranges: &mut Range ) {
  ranges._ranges.sort_by(|a,b| a.interval.start.partial_cmp(&b.interval.start).unwrap());
}

fn load_data( filename: &str) -> Almanac
{
  let mut file = File::open(filename).unwrap();
  let mut contents = String::new();
  file.read_to_string(&mut contents).unwrap();

  let mut parse_progress = ParseFields::Seeds;
  let mut almanac = Almanac::new();

  let mut heading_found = false;

  for line in contents.lines() {
    match parse_progress {
      ParseFields::Seeds => { 
        if line.is_empty() { 
          parse_progress = ParseFields::SeedSoil;
          heading_found = false;
          if almanac.seeds.is_empty() {panic!("not loaded any seeds");}
          }
          else {
            let r = sscanf::sscanf_unescaped!(line, "seeds: {String}").unwrap();
            for s in r.split(' ') {
              let seed = s.parse::<usize>().unwrap();
              almanac.seeds.push(seed);
            }
          }
        },
      ParseFields::SeedSoil => {
        if !heading_found {
          if line.cmp("seed-to-soil map:") == Ordering::Equal {
            heading_found = true;
          }
          else {
            panic!("not found seed soil heading");
          }
        }
        else if !line.is_empty() {
          parse_range(line, &mut almanac.seed_soil);
        }
        else {
          parse_progress = ParseFields::SoilFertiliser;
          heading_found = false;
          if almanac.seed_soil.is_empty() {panic!("not loaded any seed soil maps");}
          sort_range( &mut almanac.seed_soil );
        }
      },
      ParseFields::SoilFertiliser => {
        if !heading_found {
          if line.cmp("soil-to-fertilizer map:") == Ordering::Equal {
            heading_found = true;
          }
          else {
            panic!("not found soil fert heading");
          }
        }
        else if !line.is_empty() {
          parse_range(line, &mut almanac.soil_fertiliser);
        }
        else {
          parse_progress = ParseFields::FertiliserWater;
          heading_found = false;
          if almanac.soil_fertiliser.is_empty() {panic!("not loaded any fertiliser_water maps");}
          sort_range( &mut almanac.soil_fertiliser );
        }
      },
      ParseFields::FertiliserWater=> {
        if !heading_found {
          if line.cmp("fertilizer-to-water map:") == Ordering::Equal {
            heading_found = true;
          }
          else {
            panic!("not found fert water heading");
          }
        }
        else if !line.is_empty() {
          parse_range(line, &mut almanac.fertiliser_water);
        }
        else {
          parse_progress = ParseFields::WaterLight;
          heading_found = false;
          if almanac.fertiliser_water.is_empty() {panic!("not loaded any water light maps");}
          sort_range( &mut almanac.fertiliser_water );
        }
      },
      ParseFields::WaterLight => {
        if !heading_found {
          if line.cmp("water-to-light map:") == Ordering::Equal {
            heading_found = true;
          }
          else {
            panic!("not found water light heading");
          }
        }
        else if !line.is_empty() {
          parse_range(line, &mut almanac.water_light);
        }
        else {
          parse_progress = ParseFields::LightTemperature;
          heading_found = false;
          if almanac.water_light.is_empty() {panic!("not loaded any water light maps");}
          sort_range(  &mut almanac.water_light );
        }
      },
      ParseFields::LightTemperature => {
        if !heading_found {
          if line.cmp("light-to-temperature map:") == Ordering::Equal {
            heading_found = true;
          }
          else {
            panic!("not found light temp heading");
          }
        }
        else if !line.is_empty() {
          parse_range(line, &mut almanac.light_temperature);
        }
        else {
          parse_progress = ParseFields::TemperatureHumidity;
          heading_found = false;
          if almanac.light_temperature.is_empty() {panic!("not loaded any light temp maps");}
          sort_range( &mut almanac.light_temperature );
        }
      },
      ParseFields::TemperatureHumidity => {
        if !heading_found {
          if line.cmp("temperature-to-humidity map:") == Ordering::Equal {
            heading_found = true;
          }
          else {
            panic!("not found temp humid heading");
          }
        }
        else if !line.is_empty() {
          parse_range(line, &mut almanac.temperature_humidity);
        }
        else {
          parse_progress = ParseFields::HumidityLocation;
          heading_found = false;
          if almanac.temperature_humidity.is_empty() {panic!("not loaded any temp humid maps");}
          sort_range( &mut almanac.temperature_humidity );
        }
      },
      ParseFields::HumidityLocation => {
        if !heading_found {
          if line.cmp("humidity-to-location map:") == Ordering::Equal {
            heading_found = true;
          }
          else {
            panic!("not found humid loc heading");
          }
        }
        else if !line.is_empty() {
          parse_range(line, &mut almanac.humidity_location);
        }
        else {
          if almanac.humidity_location.is_empty() {panic!("not loaded any humid loc maps");}
          panic!("shouldn't reach here");
          //sorted after
        }
      },

    }
  }

  sort_range( &mut almanac.humidity_location );

  almanac
}


fn main() {
    env_logger::init();

    let args = Args::parse();
    if args.benchmark {
      return;
    }

    let almanac = load_data( "input5.txt" );
    let score1 = find_lowest(&almanac);
    let score2 = find_lowest2(&almanac);
    println!("score: {score1}, {score2} ");

}

use std::fs;
use std::str::FromStr;

#[derive(Debug)]
struct MapItem(u64, u64, u64);

impl MapItem {
    fn build(source: u64, dest: u64, count: u64) -> MapItem {
        MapItem(source, dest, count)
    }
}

#[derive(Debug)]
#[allow(dead_code)]
struct AgMap {
    source: String,
    destination: String,
    map: Vec<MapItem>,
}

impl AgMap {
    fn build(source: &str, destination: &str) -> Self {
        AgMap {
            source: String::from(source),
            destination: String::from(destination),
            map: vec![],
        }
    }

    fn add_range(&mut self, source_start: u64, dest_start: u64, count: u64) {
        self.map
            .push(MapItem::build(source_start, dest_start, count));
    }

    fn map_value(&self, value: u64) -> u64 {
        match self
            .map
            .iter()
            .find(|MapItem(start, _, count)| value >= *start && value < (start + count))
        {
            Some(MapItem(source, dest, _)) => dest + (value - source),
            None => value,
        }
    }
}

impl FromStr for AgMap {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, &'static str> {
        let Some((source_to_destination_str, rest)) = s.split_once(":") else { return Err("Failed to parse map source and destination: split :" ); };
        let Some((source_to_destination_str, _)) = source_to_destination_str.split_once(" ") else { return Err("Failed to parse map source and destination: split ' '"); };
        let strs: Vec<&str> = source_to_destination_str
            .split('-')
            .filter(|&s| s != "to")
            .collect();

        let mut ag_map = AgMap::build(strs[0], strs[1]);

        for s in rest
            .split("\n")
            .filter(|&s| !s.is_empty())
            .collect::<Vec<&str>>()
        {
            let range_strs: Vec<&str> = s.split(" ").collect();

            if !range_strs.len() == 3 {
                return Err("Failed to parse map line: unexpected length");
            }

            let dest_start = range_strs[0]
                .parse::<u64>()
                .map_err(|_| "Failed to parse source start")?;
            let source_start = range_strs[1]
                .parse::<u64>()
                .map_err(|_| "Failed to parse destination start")?;
            let count = range_strs[2]
                .parse::<u64>()
                .map_err(|_| "Failed to parse count")?;

            ag_map.add_range(source_start, dest_start, count);
        }

        return Ok(ag_map);
    }
}

fn main() -> Result<(), &'static str> {
    let input = fs::read_to_string("input.txt").expect("failed to open input file");

    let Some((default_seed_str, rest)) = input.split_once("\n\n")
        else { return Err("Failed to parse seed list: split \\n\\n") };
    let Some((_, default_seed_str)) = default_seed_str.split_once(":")
        else { return Err("Failed to parse seed list: split :") };
    let seeds = default_seed_str
        .split(" ")
        .filter(|&s| !s.is_empty())
        .map(|s| s.parse::<u64>())
        .collect::<Result<Vec<u64>, std::num::ParseIntError>>()
        .map_err(|_| "Failed to parse seeds")?;

    let mut maps: Vec<AgMap> = vec![];
    for s in rest.split("\n\n").filter(|&s| !s.is_empty()) {
        let map = s
            .parse::<AgMap>()
            .map_err(|_| "Failed to parse map section into AgMap")?;
        maps.push(map);
    }

    // Part 1
    let mut min = u64::MAX;
    for seed in seeds.iter() {
        let mut mapping = *seed;
        for map in &maps {
            mapping = map.map_value(mapping);
        }

        if mapping < min {
            min = mapping;
        }
    }
    println!("{}", min);

    // Part 2
    let seed_ranges = seeds.chunks(2).map(|chunk| (chunk[0], chunk[1])).collect::<Vec<(u64, u64)>>();

    for seed in seed_ranges.iter() {
    }

    return Ok(());
}

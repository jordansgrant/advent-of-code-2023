use std::fs;
use std::ops::Range;
use std::str::FromStr;
use rayon::prelude::*;


struct RangeMapping {
    mapped_range: Option<Range<u64>>,
    // Portions of the source range don't fit this mapping
    unmapped_sections: Vec<Range<u64>>,
}

impl RangeMapping {
    fn build(mapped_range: Option<Range<u64>>, unmapped_sections: Vec<Range<u64>>) -> Self {
        Self {
            mapped_range,
            unmapped_sections,
        }
    }
}

#[derive(Debug)]
struct MapItem(Range<u64>, Range<u64>);

impl MapItem {
    fn build(source: u64, dest: u64, count: u64) -> MapItem {
        MapItem(source..(source + count), dest..(dest + count))
    }

    fn map_value(&self, value: u64) -> bool {
        self.0.contains(&value)
    }

    fn map_range(&self, range: &Range<u64>) -> RangeMapping {
        let MapItem(source, dest) = self;

        if range.start >= source.start && range.end <= source.end {
            let mapped_range = (dest.start + (range.start - source.start))..(dest.end - (source.end - range.end));
            return RangeMapping::build(Some(mapped_range), vec![]);
        } else if range.start < source.start && range.end <= source.end && range.end > source.start {
            let mapped_range = dest.start..(dest.end - (source.end - range.end));
            let unmapped_section = range.start..source.start;
            return RangeMapping::build(Some(mapped_range), vec![unmapped_section]);
        } else if range.start >= source.start && range.end > source.end && range.start < source.end {
            let mapped_range = (dest.start + (range.start - source.start))..dest.end;
            let unmapped_section = source.end..range.end;
            return RangeMapping::build(Some(mapped_range), vec![unmapped_section]);
        } else if range.start < source.start && range.end > source.end {
            let mapped_range = dest.start..dest.end;
            let unmapped_left = range.start..source.start;
            let unmapped_right = source.end..range.end;
            return RangeMapping::build(Some(mapped_range), vec![unmapped_left, unmapped_right]);
        }

        return RangeMapping::build(None, vec![range.clone()]);
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
            .find(|map_item| map_item.map_value(value))
        {
            Some(MapItem(source, dest)) => dest.start + (value - source.start),
            None => value,
        }
    }

    fn map_range(&self, range: &Range<u64>) -> Vec<Range<u64>> {
        let mut mapped_ranges: Vec<Range<u64>> = vec![];
        let mut unmapped_ranges: Vec<Range<u64>> = vec![range.clone()];

        for j in 0..self.map.len() {
            for i in 0..unmapped_ranges.len() {
                let mapping: RangeMapping = self.map[j].map_range(&unmapped_ranges[i]);
                if let Some(mapped) = mapping.mapped_range {
                    unmapped_ranges.remove(i);
                    mapped_ranges.push(mapped);
                    if !mapping.unmapped_sections.is_empty() {
                        unmapped_ranges.extend(mapping.unmapped_sections.into_iter());
                    }
                    break;
                }
            }
        }

        if mapped_ranges.is_empty() {
            return vec![range.clone()];
        }
        mapped_ranges.extend(unmapped_ranges);
        return mapped_ranges;
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
    let seed_ranges: Vec<Range<u64>> = seeds.chunks(2).map(|chunk| chunk[0]..(chunk[0] + chunk[1])).collect::<Vec<Range<u64>>>();

    let map = seed_ranges.par_iter().map(|range| {
        let mut ranges_to_map: Vec<Range<u64>> = vec![range.clone()];

        for map in maps.iter() {
            let mut mapped_ranges: Vec<Range<u64>> = vec![];
            for range in ranges_to_map.iter() {
                mapped_ranges.extend(map.map_range(&range));
            }
            ranges_to_map = mapped_ranges; 
        }

        return ranges_to_map;
    });

    let min = map.reduce(|| Vec::<Range<u64>>::new(), |a: Vec<Range<u64>>, b: Vec<Range<u64>>| {
        let max = u64::MAX..u64::MAX;
        let min_a = match a.iter().min_by(|&a, &b| a.start.cmp(&b.start)) {
            Some(v) => v,
            None => &max,
        };
        let min_b = match b.iter().min_by(|&a, &b| a.start.cmp(&b.start)) {
            Some(v) => v,
            None => &max,
        };
        if min_a.start < min_b.start {
            return vec![min_a.clone()];
        }
        return vec![min_b.clone()];
    });

    println!("{:?}", min[0].start);
    return Ok(());
}

use std::collections::HashMap;
use std::fs;
use num::Integer;

fn follow_directions(
    directions: &str,
    map: &HashMap<&str, (&str, &str)>,
    start_location: &str,
    is_end_location: fn(&str) -> bool,
) -> u32 {
    let mut count = 0;
    let mut location = start_location;
    let mut dir_iter = directions.chars();

    while !is_end_location(location) {
        let next = match dir_iter.next() {
            Some(dir) => dir,
            None => {
                dir_iter = directions.chars();
                dir_iter.next().unwrap()
            }
        };
        let (left, right) = match map.get(location) {
            Some(val) => val,
            None => panic!("This maps leads nowhere!"),
        };

        location = match next {
            'L' => left,
            _ => right,
        };

        count += 1
    }

    count
}

fn last_char(s: &str) -> char {
    s.chars().last().unwrap()
}

fn lcm_directions(directions: &str, map: &HashMap<&str, (&str, &str)>) -> u64 {
    let locations: Vec<&str> = map.keys().filter(|k| last_char(k) == 'A').map(|k| *k).collect();
    let end_distances: Vec<u32> = locations.iter().map(|l| follow_directions(directions, map, l, |v| last_char(v) == 'Z')).collect();
    end_distances.into_iter().fold(1, |acc, dist| acc.lcm(&(dist as u64)))
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("failed to open input file");
    let (directions, maps_str) = input.split_once("\n\n").unwrap();
    let map: HashMap<&str, (&str, &str)> = HashMap::new();
    let mut map = maps_str
        .split("\n")
        .filter(|l| !l.is_empty())
        .fold(map, |mut acc, l| {
            let (key, vals) = l.split_once(" = ").unwrap();
            let (left, right) = vals
                .strip_prefix('(')
                .and_then(|s| s.strip_suffix(')'))
                .unwrap()
                .split_once(", ")
                .unwrap();

            acc.insert(key, (left, right));
            acc
        });
    map.shrink_to_fit();
    let map = map;

    // Part 1
    println!("{}", follow_directions(directions, &map, "AAA", |l| l == "ZZZ"));

    // Part 2
    println!("{}", lcm_directions(directions, &map));
}

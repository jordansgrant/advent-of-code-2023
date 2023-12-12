use std::collections::HashMap;
use std::fs;

fn follow_directions(directions: &str, map: HashMap<&str, (&str, &str)>) -> u32 {
    let mut count = 0;
    let mut location = "AAA";
    let mut dir_iter = directions.chars();

    while location != "ZZZ" {
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

    println!("{}", follow_directions(directions, map));
}

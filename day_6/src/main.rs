use std::fs;

#[derive(Debug)]
struct Race {
    time: u32,
    record_distance: u32,
}

impl Race {
    fn build(time: u32, record_distance: u32) -> Self {
        Self {
            time,
            record_distance,
        }
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("failed to open input file");
    let (line1, line2) = input.split_once("\n").unwrap();

    let line1_vals = line1.strip_prefix("Time: ").and_then(|l| Some(l.split(' '))).unwrap().filter(|s| !s.is_empty()).collect::<Vec<&str>>();
    let line2_vals = line2.strip_prefix("Distance: ").and_then(|l| Some(l.split(' '))).unwrap().filter(|s| !s.is_empty()).collect::<Vec<&str>>();

    let mut races: Vec<Race> = vec![];
    for (i, str) in line1_vals.iter().enumerate() {
        println!("{} {}", str.trim(), line2_vals[i].trim());
        let time = str.trim().parse::<u32>().unwrap();
        let distance = line2_vals[i].trim().parse::<u32>().unwrap();

        races.push(Race::build(time, distance));
    }

    println!("{:?}", races);
}

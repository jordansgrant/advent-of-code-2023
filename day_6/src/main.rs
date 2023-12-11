use std::fs;
use std::ops::Range;

#[derive(Debug)]
struct Race {
    time: u32,
    record_distance: u128,
}

impl Race {
    fn build(time: u32, record_distance: u128) -> Self {
        Self {
            time,
            record_distance,
        }
    }
}

fn has_won_race(race: &Race, time: u32) -> bool {
    u128::from(time) * u128::from(race.time - time) > race.record_distance
}

fn add_unsigned_negative(val: u32, direction: i32) -> u32 {
    if direction < 0 {
        if val == 0 {
            return 0;
        }
        return val - (direction * -1) as u32;
    }

    val + direction as u32
}

fn scan_for_range_bound(race: &Race, time: u32, direction: i32) -> u32 {
    let mut curr = add_unsigned_negative(time, direction);
    while curr > 0 || curr <= race.time {
        if !has_won_race(race, curr) {
            break;
        }

        curr = add_unsigned_negative(curr, direction);
    }

    return curr;
}

fn find_winning_value(race: &Race, low: u32, high: u32) -> Option<u32> {
    let mid = (low as f64 + (high as f64 - low as f64) / 2.0) as u32;
    if has_won_race(race, mid) {
        return Some(mid);
    }
    if low >= high {
        return None;
    }

    let maybe_left = find_winning_value(race, low, mid);
    let maybe_right = find_winning_value(race, mid + 1, high);

    if maybe_left.is_some() {
        return maybe_left;
    }
    if maybe_right.is_some() {
        return maybe_right;
    }

    return None;
}

fn find_winning_range(race: &Race) -> Option<Range<u32>> {
    let Some(winning_value) = find_winning_value(race, 0, race.time) else {
        return None;
    };
    let first_bound = scan_for_range_bound(race, winning_value, -1) + 1;
    let second_bound = scan_for_range_bound(race, winning_value, 1);

    return Some(first_bound..second_bound);
}

fn range_difference(range: &Range<u32>) -> u32 {
    let diff = range.end - range.start;

    if diff == 0 {
        return 1;
    }

    diff
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("failed to open input file");
    let (line1, line2) = input.split_once("\n").unwrap();

    let line1_vals = line1
        .strip_prefix("Time: ")
        .and_then(|l| Some(l.split(' ')))
        .unwrap()
        .filter(|s| !s.is_empty())
        .collect::<Vec<&str>>();
    let line2_vals = line2
        .strip_prefix("Distance: ")
        .and_then(|l| Some(l.split(' ')))
        .unwrap()
        .filter(|s| !s.is_empty())
        .collect::<Vec<&str>>();

    let mut races: Vec<Race> = vec![];
    for (i, str) in line1_vals.iter().enumerate() {
        let time = str.trim().parse::<u32>().unwrap();
        let distance = line2_vals[i].trim().parse::<u128>().unwrap();

        races.push(Race::build(time, distance));
    }

    // Part 1
    let mut winning_ranges: Vec<Range<u32>> = vec![];
    for race in races.iter() {
        let Some(winning_range) = find_winning_range(race) else {
            continue;
        };
        winning_ranges.push(winning_range);
    }

    let product = winning_ranges
        .iter()
        .fold(1, |acc, range| acc * range_difference(range));
    println!("{}", product);

    // Part 2
    let (long_time, long_record) = races.iter().fold(("".to_owned(), "".to_owned()), |acc, r| {
        (
            format!("{}{}", acc.0, r.time),
            format!("{}{}", acc.1, r.record_distance),
        )
    });
    let time = long_time.trim().parse::<u32>().unwrap();
    let record_distance = long_record.trim().parse::<u128>().unwrap();
    let race = Race::build(time, record_distance);
    let winning_range = find_winning_range(&race).unwrap();
    println!("{}", range_difference(&winning_range));
}

use itertools::Itertools;
use std::fs;

fn parse_sequence(line: &str) -> Vec<i32> {
    line.split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect()
}

fn find_next_val(seq: &Vec<i32>, acc: i32) -> i32 {
    if seq.iter().all(|&v| v == 0) {
        return acc;
    }

    let diffs: Vec<i32> = seq.iter().tuple_windows().map(|(l, r)| r - l).collect();
    return find_next_val(&diffs, seq.iter().last().unwrap() + acc);
}

fn find_prev_val(seq: &Vec<i32>) -> i32 {
    if seq.iter().all(|&v| v == 0) {
        return 0;
    }

    let diffs: Vec<i32> = seq.iter().tuple_windows().map(|(l, r)| r - l).collect();
    return seq.iter().nth(0).unwrap() - find_prev_val(&diffs);
}

fn part_1(seq: &Vec<Vec<i32>>) -> i32 {
    seq.iter()
        .map(|s| find_next_val(s, 0))
        .fold(0, |acc, v| acc + v)
}

fn part_2(seq: &Vec<Vec<i32>>) -> i32 {
    seq.iter()
        .map(|s| find_prev_val(s))
        .fold(0, |acc, v| acc + v)
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Failed to open file");
    let sequences: Vec<Vec<i32>> = input.lines().map(|line| parse_sequence(line)).collect();

    println!("{}", part_1(&sequences));
    println!("{}", part_2(&sequences));
}

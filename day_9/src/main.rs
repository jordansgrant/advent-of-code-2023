use std::fs;

fn has_zero_diff(diff_seqs: &Vec<Vec<u32>>) -> bool {
    match diff_seqs.iter().last() {
        Some(v) => v.iter().all(|v| *v == 0),
        None => false,
    }
}

fn find_next_value(seq: &Vec<u32>) -> u32 {
    let mut reverse = seq.iter().rev();
    let mut val1 = *reverse.next().unwrap();
    let mut val2 = *reverse.next().unwrap();
    println!("{} {}", val1, val2);
    let mut diff_seqs: Vec<Vec<u32>> = vec![vec![val1, val2], vec![val1 - val2]];

    while !has_zero_diff(&diff_seqs) {
        diff_seqs.push(vec![]);
        let mut next = match reverse.next() {
            Some(v) => *v,
            None => panic!("Shouldn't be here"),
        };

        for i in 1..diff_seqs.len() {
            val1 = *diff_seqs[i - 1].iter().last().unwrap();
            val2 = next;
            diff_seqs[i - 1].push(next);
            println!("{} {}", val1, val2);
            next = val1 - val2;
            diff_seqs[i].push(next);
        }
    }

    diff_seqs.iter().rev().fold(0, |acc, v| {
        let val = v.iter().nth(0).unwrap();
        println!("{} {}", acc, val);
        acc + val
    })
}

fn main() {
    let input = fs::read_to_string("sample_input.txt").expect("Failed to open file");
    let sequences: Vec<Vec<u32>> = input
        .split("\n")
        .filter(|l| !l.is_empty())
        .map(|l| {
            l.split(" ")
                .map(|v| v.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .collect();

    println!("{:?}", sequences);

    let next_seq_values: Vec<u32> = sequences.iter().map(|s| find_next_value(s)).collect();

    println!("{:?}", next_seq_values);
    println!("{}", next_seq_values.iter().fold(0, |acc, v| acc + v));
}

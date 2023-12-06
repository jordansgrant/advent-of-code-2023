use std::fs;
use std::ops::RangeInclusive;

const ADJACENT_OFFSETS: &[(i32, i32)] = &[
    (-1, -1),
    (0, -1),
    (1, -1),
    (1, 0),
    (-1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

fn is_symbol(val: u8) -> bool {
    (33..=45).contains(&val)
        || val == 47
        || (58..=64).contains(&val)
        || (91..=96).contains(&val)
        || (123..=126).contains(&val)
}

fn is_digit(val: u8) -> bool {
    (48..=57).contains(&val)
}

fn is_star(val: u8) -> bool {
    val == 42
}

fn add_offset(val: usize, offset: i32) -> Option<usize> {
    if offset < 0 && val == 0 {
        return None;
    }

    if offset < 1 {
        Some(val - offset.abs() as usize)
    } else {
        Some(val + offset as usize)
    }
}

fn check_adjacency_part_number(
    engine: &Vec<Vec<u8>>,
    bounds: (usize, usize),
    location: (usize, usize),
) -> bool {
    ADJACENT_OFFSETS.iter().fold(false, |acc, &offset| {
        let Some(x_offset) = add_offset(location.0, offset.0) else { return acc || false };
        let Some(y_offset) = add_offset(location.1, offset.1) else { return acc || false };

        if x_offset > bounds.0 || y_offset > bounds.1 {
            return acc || false;
        }

        return acc || is_symbol(engine[y_offset][x_offset]);
    })
}

fn create_range(index1: usize, index2: usize) -> RangeInclusive<usize> {
    if index1 > index2 {
        return index2..=index1;
    }

    index1..=index2
}

fn new_gear_value(
    engine: &Vec<Vec<u8>>,
    acc: (u32, Option<Vec<usize>>),
    location: (usize, usize),
) -> (u32, Option<Vec<usize>>) {
    if acc.0 == 0 {
        return (1, Some(vec![location.0, location.1]));
    }
    if acc.0 == 2 {
        return acc;
    }

    let positions: &Vec<usize> = &acc.1.as_ref().unwrap();
    if positions[1] == location.1
        && engine[location.1][create_range(positions[0], location.0)]
            .iter()
            .all(|&b| is_digit(b))
    {
        return acc;
    }

    return (
        2,
        Some(vec![positions[0], positions[1], location.0, location.1]),
    );
}

fn adjacency_gear_ratio(
    engine: &Vec<Vec<u8>>,
    bounds: (usize, usize),
    location: (usize, usize),
) -> (bool, Option<Vec<usize>>) {
    let (count, maybe_vec) = ADJACENT_OFFSETS.iter().fold((0, None), |acc, &offset| {
        let Some(x_offset) = add_offset(location.0, offset.0) else { return acc };
        let Some(y_offset) = add_offset(location.1, offset.1) else { return acc };

        if x_offset > bounds.0 || y_offset > bounds.1 {
            return acc;
        }

        if is_digit(engine[y_offset][x_offset]) {
            return new_gear_value(engine, acc, (x_offset, y_offset));
        }

        return acc;
    });

    if count == 2 {
        return (true, maybe_vec);
    }

    return (false, None);
}

// returns a tuple of the parsed value and the index of the last digit
fn parse_number(engine_row: &Vec<u8>, len: usize, index: usize) -> (u32, usize) {
    let mut first = index;
    let mut last = index;
    while first > 0 && is_digit(engine_row[first - 1]) {
        first -= 1;
    }
    while last < len && is_digit(engine_row[last + 1]) {
        last += 1;
    }

    let string = std::str::from_utf8(&engine_row[first..=last]).unwrap();
    let number = string.parse::<u32>().unwrap();
    return (number, last);
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("failed to open input file");

    let engine: Vec<Vec<u8>> = input
        .split("\n")
        .filter(|&s| !s.is_empty())
        .map(|s| s.as_bytes().to_owned().into_iter().collect())
        .collect();
    let width = engine[0].len() - 1;
    let height = engine.len() - 1;

    // Part 1
    let mut count = 0;
    for j in 0..=height {
        let mut width_iter = 0..=width;
        while let Some(i) = width_iter.next() {
            if is_digit(engine[j][i])
                && check_adjacency_part_number(&engine, (width, height), (i, j))
            {
                let (number, last_digit) = parse_number(&engine[j], width, i);
                count += number;
                width_iter.nth(last_digit - i);
            }
        }
    }
    println!("{}", count);

    // Part 2
    let mut count = 0;
    for j in 0..=height {
        let mut width_iter = 0..=width;
        while let Some(i) = width_iter.next() {
            if is_star(engine[j][i]) {
                let (is_gear_ratio, maybe_positions) =
                    adjacency_gear_ratio(&engine, (width, height), (i, j));
                if is_gear_ratio {
                    let positions = maybe_positions.unwrap();
                    let (number1, _) = parse_number(&engine[positions[1]], width, positions[0]);
                    let (number2, _) = parse_number(&engine[positions[3]], width, positions[2]);
                    count += number1 * number2;
                }
            }
        }
    }
    println!("{}", count);
}

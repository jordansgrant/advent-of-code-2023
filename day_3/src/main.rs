use std::fs;

const ADJACENT_OFFSETS: &[(i32, i32)] = &[(-1, -1), (0, -1), (1, -1), (1, 0), (1, 1), (0, 1), (-1, 1), (-1, 0)];

fn is_symbol(val: u8) -> bool {
    (33..=47).contains(&val) || (58..=64).contains(&val) || (91..=96).contains(&val) || (123..=126).contains(&val)
}

fn is_digit(val: u8) -> bool {
    (48..=57).contains(&val)
}

fn add_offset(val: usize, offset: i32) -> Option<usize> {
    if offset < 0 && val == 0 {
        return None
    }

    Some(val + offset as usize)
}

fn check_adjacency(engine: &Vec<Vec<u8>>, bounds: (usize, usize), location: (usize, usize)) -> bool {
    ADJACENT_OFFSETS.iter().fold(false, |acc, &offset| {
        let Some(x_offset) = add_offset(location.0, offset.0) else { return acc || false };
        let Some(y_offset) = add_offset(location.1, offset.1) else { return acc || false };

        if x_offset > bounds.0 || y_offset > bounds.1 {
            return acc || false;
        }

        return acc || is_symbol(engine[y_offset][x_offset]);
    })
}

// returns a tuple of the parsed value and the index of the last digit
fn parse_number(engine_row: &Vec<u8>, width: usize, index: usize) -> (u32, usize) {
    let mut first = index;
    let mut last = index;
    while first > 0 && is_digit(engine_row[first]) {
        first -= 1;
    }
    while (last < width && is_digit(engine_row[last])) {
        last = index + 1;
    }

    (engine_row[(first + 1)..last].parse::<u32>().unwrap(), last - 1)
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

    let mut count = 0;

    for j in 0..=height {
        let mut width_iter = 0..=width;
        while let Some(i) = width_iter.next()  {
            if is_digit(engine[j][i]) && check_adjacency(&engine, (width, height), (i, j)) {
                let (number, last_digit) = parse_number(&engine[j], width, i);
                count += number;
                width_iter.nth(last_digit);
            }
        }
    }

    println!("{:?}", count);
}

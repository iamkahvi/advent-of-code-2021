use std::fs;
const BASE: i32 = 2;

fn main() {
    let temp = fs::read_to_string("input.txt").unwrap();
    let input: Vec<Vec<i32>> = temp
        .lines()
        .collect::<Vec<&str>>()
        .iter()
        .map(|line| {
            let parsed_line: Vec<i32> = line
                .chars()
                .map(|c| {
                    return c.to_digit(10).unwrap() as i32;
                })
                .collect();

            return parsed_line;
        })
        .collect();

    println!("part 1: {}", part_1(&input));
    println!("part 2: {}", part_2(&input));
}

fn flip_bits(num: &Vec<i32>) -> Vec<i32> {
    return num
        .iter()
        .map(|b| {
            return if b == &1 { 0 } else { 1 };
        })
        .collect();
}

fn bits_to_number(num: &Vec<i32>) -> i32 {
    return num
        .iter()
        .enumerate()
        .map(|(i, x)| (x * BASE.pow((num.len() - 1 - i) as u32)))
        .sum();
}
fn part_1(input: &Vec<Vec<i32>>) -> i32 {
    println!("{:?}", input);

    let count = generate_count(input);
    println!("count: {:?}", count);
    let gamma = bits_to_number(&count);
    let epsilon = bits_to_number(&flip_bits(&count));
    println!("gamma: {}", gamma);
    return epsilon * gamma;
}

fn part_2(input: &Vec<Vec<i32>>) -> i32 {
    let mut count;
    let mut i = 0;
    let mut filtered = input.clone();
    while filtered.len() > 1 {
        count = generate_count(&filtered);
        filtered = filter_by_bit_pos(filtered, i, count[i]);
        i += 1;
    }
    println!(
        "oxygen generator rating: {:?}",
        bits_to_number(&filtered[0])
    );
    return 0;
}

// Filters vector by a bit: value at a certain position: pos
fn filter_by_bit_pos(input: Vec<Vec<i32>>, pos: usize, value: i32) -> Vec<Vec<i32>> {
    return input.into_iter().filter(|x| x[pos] == value).collect();
}

// Returns an array repesenting the most frequent bit by position
fn generate_count(input: &Vec<Vec<i32>>) -> Vec<i32> {
    let mut count: Vec<i32> = vec![0; input[0].len()];
    for num in input {
        for (i, d) in num.iter().enumerate() {
            count[i] += if d == &1 { 1 } else { -1 };
        }
    }
    return count.iter().map(|x| if x >= &0 { 1 } else { 0 }).collect();
}

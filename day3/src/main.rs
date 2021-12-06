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

fn part_1(input: &Vec<Vec<i32>>) -> i32 {
    let count = generate_count(input, &(0..input.len()).collect());
    let gamma = bits_to_number(&count);
    let epsilon = bits_to_number(&flip_bits(&count));
    return epsilon * gamma;
}

fn part_2(input: &Vec<Vec<i32>>) -> i32 {
    let mut ox_index: Vec<usize> = (0..input.len()).collect();
    let mut co2_index: Vec<usize> = (0..input.len()).collect();
    let mut i = 0;
    let mut count;
    while ox_index.len() > 1 {
        count = generate_count(input, &ox_index)[i];
        ox_index = filter_by_bit_pos(input, ox_index, i, count);
        i += 1;
    }
    i = 0;
    while co2_index.len() > 1 {
        count = flip_bits(&generate_count(input, &co2_index))[i];
        co2_index = filter_by_bit_pos(input, co2_index, i, count);
        i += 1;
    }
    let ox_rating = bits_to_number(&input[ox_index[0]]);
    let co2_rating = bits_to_number(&input[co2_index[0]]);

    return ox_rating * co2_rating;
}

// Filters vector by a bit: value at a certain position: pos
fn filter_by_bit_pos(
    input: &Vec<Vec<i32>>,
    index: Vec<usize>,
    pos: usize,
    value: i32,
) -> Vec<usize> {
    return index
        .into_iter()
        .filter(|i| input[*i][pos] == value)
        .collect();
}

// Returns an array repesenting the most frequent bit by position
fn generate_count(input: &Vec<Vec<i32>>, index: &Vec<usize>) -> Vec<i32> {
    let mut count: Vec<i32> = vec![0; input[0].len()];
    for (i, num) in input.iter().enumerate() {
        if index.contains(&i) {
            for (i, d) in num.iter().enumerate() {
                count[i] += if d == &1 { 1 } else { -1 };
            }
        }
    }
    return count.iter().map(|x| if x >= &0 { 1 } else { 0 }).collect();
}

// Flips the bits of a vector
fn flip_bits(num: &Vec<i32>) -> Vec<i32> {
    return num
        .iter()
        .map(|b| {
            return if b == &1 { 0 } else { 1 };
        })
        .collect();
}

// Returns the decimal number from a vector
fn bits_to_number(num: &Vec<i32>) -> i32 {
    return num
        .iter()
        .enumerate()
        .map(|(i, x)| (x * BASE.pow((num.len() - 1 - i) as u32)))
        .sum();
}

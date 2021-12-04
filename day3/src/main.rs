use std::fs;

fn main() {
    let temp = fs::read_to_string("input.txt").unwrap();
    let input = temp.lines().collect::<Vec<&str>>();
    println!("part 1: {}", part_1(&input));
}

fn part_1(input: &Vec<&str>) -> i32 {
    let mut count: Vec<i32> = vec![0; input[0].len()];
    for line in input {
        for (i, c) in line.chars().enumerate() {
            count[i] += if c == '0' { -1 } else { 1 };
        }
    }
    let gamma_str = count
        .iter()
        .map(|x| if x > &0 { '1' } else { '0' })
        .collect::<Vec<char>>()
        .iter()
        .collect::<String>();
    let epsilon_str = count
        .iter()
        .map(|x| if x > &0 { '0' } else { '1' })
        .collect::<Vec<char>>()
        .iter()
        .collect::<String>();
    let epsilon = i32::from_str_radix(&epsilon_str, 2).unwrap();
    let gamma = i32::from_str_radix(&gamma_str, 2).unwrap();
    return epsilon * gamma;
}

use std::fs;
const BASE: i32 = 2;

fn main() {
    let temp = fs::read_to_string("input.txt").unwrap();
    let input = temp.lines().collect::<Vec<&str>>();
    println!("part 1: {}", part_1(&input));
}

fn ones(num: i32) -> i32 {
    let mut res = 0;
    for i in 0..num {
        res += BASE.pow(i as u32);
    }
    return res;
}

fn part_1(input: &Vec<&str>) -> i32 {
    let mut count: Vec<i32> = vec![0; input[0].len()];
    for line in input {
        for (i, c) in line.chars().enumerate() {
            count[i] += if c == '0' { -1 } else { 1 };
        }
    }
    let gamma: i32 = count
        .iter()
        .enumerate()
        .map(|(i, x)| {
            if x > &0 {
                BASE.pow((count.len() - i - 1) as u32)
            } else {
                0
            }
        })
        .collect::<Vec<i32>>()
        .iter()
        .sum();
    let epsilon = !gamma & ones(count.len() as i32);
    println!("epsilon: {}", epsilon);
    println!("gamma: {}", gamma);
    return epsilon * gamma;
}

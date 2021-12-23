use std::fs;

fn main() {
    let line: String = fs::read_to_string("input.txt").unwrap();
    let mut crabs: Vec<i32> = line
        .split(',')
        .map(|n| {
            return n.parse().unwrap();
        })
        .collect();

    crabs.sort();
    println!("part 1: {}", part_1(&crabs));
    println!("part 2: {}", part_2(&crabs));
}

fn part_1(crabs: &Vec<i32>) -> i32 {
    let med = crabs[crabs.len() / 2];
    println!("{}", med);
    let mut fuel = 0;
    for c in crabs {
        fuel += (c - med).abs();
    }
    return fuel;
}

fn part_2(crabs: &Vec<i32>) -> i32 {
    let mean = (crabs.iter().sum::<i32>() as f32 / crabs.len() as f32).ceil() as i32;
    let mut fuel = 0;
    for c in crabs {
        let n = (c - mean).abs() as f32;
        let sum = (n.powf(2.0) + n) / 2.0;
        fuel += sum as i32;
        // println!("{} to {} -> {}", c, med, sum as f32);
    }
    return fuel;
}

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
    println!("part 1: {}", part_1(crabs));
}

fn part_1(crabs: Vec<i32>) -> i32 {
    let med = crabs[crabs.len() / 2];
    println!("{}", med);
    let mut fuel = 0;
    for c in crabs {
        fuel += (c - med).abs();
    }
    return fuel;
}

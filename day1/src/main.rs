use std::fs;

fn main() {
    let name = "kahvi";
    println!("my name is {}", name);
    let input = fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|x| x.parse().unwrap())
        .collect::<Vec<i32>>();

    let ans = part_1(&input);
    println!("answer pt1: {}", ans);
    let ans2 = part_2(&input);
    println!("answer pt2: {}", ans2);
}

fn part_1(input: &[i32]) -> i32 {
    let res = input
        .windows(2)
        .map(|el| {
            return if el[1] > el[0] { 1 } else { 0 };
        })
        .sum();
    return res;
}

fn part_2(input: &[i32]) -> i32 {
    let res = input
        .windows(4)
        .map(|el| {
            let first: i32 = el[0..3].iter().sum();
            let second: i32 = el[1..4].iter().sum();

            return if second > first { 1 } else { 0 };
        })
        .sum();

    return res;
}

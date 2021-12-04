use std::fs;

fn main() {
    let temp = fs::read_to_string("input.txt").unwrap();
    let input = temp
        .lines()
        .map(|x| {
            let pair = x.split(" ").collect::<Vec<&str>>();
            return (pair[0], pair[1].parse().unwrap());
        })
        .collect::<Vec<(&str, i32)>>();

    println!("part 1: {:?}", part_1(&input));
    println!("part 2: {:?}", part_2(&input));
}

fn part_1(input: &Vec<(&str, i32)>) -> i32 {
    let mut depth = 0;
    let mut pos = 0;
    for el in input {
        let (cmd, amount) = el;
        match cmd.as_ref() {
            "up" => depth -= amount,
            "down" => depth += amount,
            "forward" => pos += amount,
            _ => println!("Invalid entry"),
        }
    }
    return depth * pos;
}

fn part_2(input: &Vec<(&str, i32)>) -> i32 {
    let mut depth = 0;
    let mut pos = 0;
    let mut aim = 0;
    for el in input {
        let (cmd, amount) = el;
        match cmd.as_ref() {
            "up" => {
                aim -= amount;
            }
            "down" => {
                aim += amount;
            }
            "forward" => {
                pos += amount;
                depth += aim * amount;
            }
            _ => println!("Invalid entry"),
        }
    }
    return depth * pos;
}

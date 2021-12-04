use std::fs;

fn main() {
    let temp = fs::read_to_string("input.txt").unwrap();
    
    let input = temp
        .lines()
        .map(|x| x.split(" ").collect())
        .collect::<Vec<Vec<&str>>>();

    println!("{:?}", part_1(&input));
}

fn part_1(input: &Vec<Vec<&str>>) -> i32 {
    let mut depth = 0;
    let mut pos = 0;
    
    for el in input {
        let num: i32 = el[1].parse().unwrap();
        
        match el[0] {
            "up" => depth -= num,
            "down" => depth += num,
            "forward" => pos += num,
            _ => println!("Invalid entry")
        }
    }
    
    return depth * pos;
}

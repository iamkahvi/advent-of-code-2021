use std::fs;

fn main() {
    let line: String = fs::read_to_string("input.txt").unwrap();
    let fish: Vec<i32> = line
        .split(',')
        .map(|n| {
            return n.parse().unwrap();
        })
        .collect();
    println!("part 1: {}", part_1(&fish));
    let days = 256;
    let mut sum = 0;
    for f in fish {
        sum += 1 + calc_fish(f, days);
    }
    println!("{}", sum);
    // println!("{}", calc_fish(3, 18));
    // println!("{}", calc_fish(4, 18));
}

fn part_1(fish: &Vec<i32>) -> i128 {
    let mut nfish = fish.clone();
    for _ in 0..18 {
        // println!("{:?}", nfish);
        // println!("{:?}", nfish.iter().sum::<i32>());
        println!("{:?}", nfish.len());
        let mut to_add: Vec<i32> = Vec::new();
        nfish = nfish
            .into_iter()
            .map(|f| {
                if f == 0 {
                    to_add.push(8);
                    return 6;
                }
                return f - 1;
            })
            .collect();
        // println!("{:?}\n", to_add.len());
        nfish.append(&mut to_add);
    }
    return nfish.len() as i128;
}

// (3,18) -> 3
// (18 + 1) - (3 + 2) = 14 -> 14 - 7 = 7 -> 7 - 7 = 0
//
fn calc_fish(start: i32, days: i32) -> i32 {
    let mut x = start + 2;
    let mut spent = days + 1;
    let mut added = 0;
    loop {
        spent -= x;
        if spent < 0 {
            break;
        }
        // println!("calc_fish({},{})", 8, spent);
        added += 1 + calc_fish(8, spent);
        x = 7;
    }
    return added;
}

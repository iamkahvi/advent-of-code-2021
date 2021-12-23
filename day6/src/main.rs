use std::collections::HashMap;
use std::fs;

const DAYS: i32 = 256;

fn main() {
    let line: String = fs::read_to_string("input.txt").unwrap();
    let fish: Vec<i32> = line
        .split(',')
        .map(|n| {
            return n.parse().unwrap();
        })
        .collect();
    println!("part 1: {}", part_1(&fish, DAYS));
    println!("part 2: {}", part_2(&fish, DAYS));
}

fn part_1(fish: &Vec<i32>, days: i32) -> i32 {
    let mut nfish = fish.clone();
    for _ in 0..days {
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
        nfish.append(&mut to_add);
    }
    return nfish.len() as i32;
}

fn part_2(fish: &Vec<i32>, days: i32) -> i128 {
    let mut sum = 0;
    let mut map: HashMap<(i32, i128), i128> = HashMap::new();
    for f in fish {
        if map.contains_key(&(*f, days as i128)) {
            sum += 1 + map.get(&(*f, days as i128)).unwrap();
        } else {
            let res = calc_fish(*f, days as i128, &mut map);
            map.insert((*f, days as i128), res);
            sum += 1 + res;
        }
    }
    return sum;
}

fn calc_fish(start: i32, days: i128, map: &mut HashMap<(i32, i128), i128>) -> i128 {
    let mut x = start + 2;
    let mut spent = days + 1;
    let mut added: i128 = 0;
    loop {
        spent -= x as i128;
        if spent < 0 {
            break;
        }
        if map.contains_key(&(8, spent)) {
            added += 1 + map.get(&(8, spent)).unwrap();
        } else {
            let res = calc_fish(8, spent, map);
            map.insert((8, spent), res);
            added += 1 + res;
        }
        x = 7;
    }
    return added;
}

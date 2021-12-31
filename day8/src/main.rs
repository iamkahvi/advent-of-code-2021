use std::fmt;
use std::fs;

struct Entry {
    input: Vec<String>,
    output: Vec<String>,
}

impl fmt::Debug for Entry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "{:?} | {:?}\n", self.input, self.output);
    }
}

//  6666
// 5    1
// 5    1
//  7777
// 4    2
// 4    2
//  3333

fn main() {
    let temp: String = fs::read_to_string("input.txt").unwrap();

    let entries: Vec<Entry> = temp
        .lines()
        .map(|l| {
            let two: Vec<String> = l.splitn(2, '|').map(|l| l.to_string()).collect();
            let input: Vec<String> = two[0].split_whitespace().map(|l| l.to_string()).collect();
            let output: Vec<String> = two[1].split_whitespace().map(|l| l.to_string()).collect();

            return Entry { input, output };
        })
        .collect();
    // println!("{:?}", entries);
    println!("part 1: {:?}", part_1(&entries));
}

fn part_1(entries: &Vec<Entry>) -> i32 {
    let mut count = 0;
    for e in entries {
        for s in e.output.iter() {
            let chars: Vec<char> = s.chars().collect();
            match solve_digit(&chars) {
                Some(_) => count += 1,
                None => continue,
            }
        }
    }
    return count;
}

fn part_2(entries: &Vec<Entry>) -> i32 {
    return 0;
}

fn solve_digit(d: &Vec<char>) -> Option<i32> {
    match d.len() {
        2 => return Some(1),
        3 => return Some(7),
        4 => return Some(4),
        7 => return Some(8),
        _ => return None,
    }
}

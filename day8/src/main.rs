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

fn main() {
    let temp: String = fs::read_to_string("input.txt").unwrap();

    let entries: Vec<Entry> = temp
        .lines()
        .map(|l| {
            let two: Vec<String> = l.split('|').map(|l| l.to_string()).collect();
            let input: Vec<String> = two[0].split_whitespace().map(|l| l.to_string()).collect();
            let output: Vec<String> = two[1].split_whitespace().map(|l| l.to_string()).collect();

            return Entry { input, output };
        })
        .collect();
    println!("{:?}", entries);
    println!("part 1: {:?}", part_1(&entries));
}

fn part_1(entries: &Vec<Entry>) -> i32 {
    let mut count = 0;
    let lengths: Vec<usize> = vec![2, 3, 4, 7];
    for e in entries {
        for s in e.output.iter() {
            if lengths.contains(&s.len()) {
                count += 1;
            }
        }
    }
    return count;
}

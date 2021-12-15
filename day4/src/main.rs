use std::fs;

type Board = Vec<Vec<i32>>;

fn main() {
    let temp: String = fs::read_to_string("input.txt").unwrap();
    let lines: Vec<&str> = temp.lines().collect();
    let numbers: Vec<i32> = lines[0].split(',').map(|x| x.parse().unwrap()).collect();
    let boards: Vec<Board> = lines[2..]
        .split(|l| l == &"")
        .map(|line_group| {
            return line_group
                .into_iter()
                .map(|line| {
                    return line
                        .split_whitespace()
                        .map(|x| x.parse().unwrap())
                        .collect();
                })
                .collect();
        })
        .collect();
    let boards2 = boards.clone();
    println!("part 1: {}", part_1(&numbers, boards));
    println!("part 2: {}", part_2(&numbers, boards2));
}

fn part_1(numbers: &Vec<i32>, mut boards: Vec<Board>) -> i32 {
    for n in numbers {
        for b in &mut boards {
            *b = mark_board(b, *n);
            if check_board(b) {
                return b
                    .iter()
                    .map(|row| {
                        return row
                            .iter()
                            .filter(|x| x >= &&0)
                            .collect::<Vec<&i32>>()
                            .into_iter()
                            .sum();
                    })
                    .collect::<Vec<i32>>()
                    .iter()
                    .sum::<i32>()
                    * n;
            }
        }
    }
    return 0;
}

fn part_2(numbers: &Vec<i32>, mut boards: Vec<Board>) -> i32 {
    let mut solved: Vec<usize> = Vec::new();
    let len = boards.len();
    for n in numbers {
        for (i, b) in boards.iter_mut().enumerate() {
            if solved.contains(&i) {
                continue;
            };
            *b = mark_board(b, *n);
            if check_board(b) {
                solved.push(i);
                if solved.len() == len {
                    return b
                        .iter()
                        .map(|row| {
                            return row
                                .iter()
                                .filter(|x| x >= &&0)
                                .collect::<Vec<&i32>>()
                                .into_iter()
                                .sum();
                        })
                        .collect::<Vec<i32>>()
                        .iter()
                        .sum::<i32>()
                        * n;
                }
            }
        }
    }
    return 0;
}

fn check_board(board: &Board) -> bool {
    // Check columns
    for i in 0..board.len() {
        let col = board.iter().map(|x| x[i]).collect::<Vec<i32>>();
        if col.iter().all(|x| x < &0) {
            return true;
        }
    }
    // Check rows
    for row in board {
        if row.iter().all(|x| x < &0) {
            return true;
        }
    }
    return false;
}

fn mark_board(board: &Board, number: i32) -> Board {
    return board
        .iter()
        .map(|row| {
            return row
                .iter()
                .map(|x| {
                    if x == &number {
                        // Uh oh
                        if x == &0 {
                            return -100;
                        }
                        return x * -1;
                    }
                    return *x;
                })
                .collect();
        })
        .collect::<Board>()
        .clone();
}

use std::fmt;
use std::fs;

struct Point {
    x: i32,
    y: i32,
}

type Line = (Point, Point);

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "({}, {})", self.x, self.y);
    }
}

impl Clone for Point {
    fn clone(&self) -> Self {
        return Point {
            x: self.x,
            y: self.y,
        };
    }
}

fn main() {
    let temp: String = fs::read_to_string("input.txt").unwrap();
    let points: Vec<Line> = temp
        .lines()
        .map(|l| {
            let points: Vec<Point> = l
                .split(" -> ")
                .map(|p| {
                    println!("{}", p);
                    let nums: Vec<i32> = p.split(',').map(|n| n.parse().unwrap()).collect();
                    return Point {
                        x: nums[0],
                        y: nums[1],
                    };
                })
                .collect();
            return (points[0].clone(), points[1].clone());
        })
        .collect();

    println!("{:?}", points);
}

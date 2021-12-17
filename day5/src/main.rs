use std::fmt;
use std::fs;

struct Point {
    x: i32,
    y: i32,
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "({},{})", self.x, self.y);
    }
}

struct LineSegment {
    start: Point,
    end: Point,
}

impl LineSegment {
    fn new(start: Point, end: Point) -> LineSegment {
        return LineSegment { start, end };
    }
    fn overlap(&self, l: &LineSegment) -> Vec<Point> {
        return Vec::new();
    }
    fn points_covered(&self) -> Option<Vec<Point>> {
        let Point { x: x1, y: y1 } = self.start;
        let Point { x: x2, y: y2 } = self.end;
        if x1 == x2 {
            let mut v = Vec::new();
            match (y1, y2) {
                (y1, y2) if y1 < y2 => {
                    for i in y1..(y2 + 1) {
                        v.push(Point { x: x1, y: i });
                    }
                }
                (y1, y2) if y1 > y2 => {
                    for i in (y1..(y2 + 1)).rev() {
                        v.push(Point { x: x1, y: i });
                    }
                }
                (y1, _y2) => {
                    v.push(Point { x: x1, y: y1 });
                }
            }
            return Some(v);
        } else if y1 == y2 {
            let mut v = Vec::new();
            match (x1, x2) {
                (x1, x2) if x1 < x2 => {
                    for i in x1..(x2 + 1) {
                        v.push(Point { x: i, y: y1 });
                    }
                }
                (x1, x2) if x1 > x2 => {
                    for i in (x2..(x1 + 1)).rev() {
                        v.push(Point { x: i, y: y1 });
                    }
                }
                (x1, _x2) => {
                    v.push(Point { x: x1, y: y1 });
                }
            }
            return Some(v);
        } else {
            return None;
        }
    }
}

impl fmt::Debug for LineSegment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "{:?} -> {:?}\n", self.start, self.end);
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
    let lines: Vec<LineSegment> = temp
        .lines()
        .map(|l| {
            let points: Vec<Point> = l
                .split(" -> ")
                .map(|p| {
                    let nums: Vec<i32> = p.split(',').map(|n| n.parse().unwrap()).collect();
                    return Point {
                        x: nums[0],
                        y: nums[1],
                    };
                })
                .collect();
            return LineSegment::new(points[0].clone(), points[1].clone());
        })
        .collect();

    println!("{:?}", lines);

    println!(
        "points covered for {:?}: {:?}",
        lines[2],
        lines[2].points_covered()
    )
}

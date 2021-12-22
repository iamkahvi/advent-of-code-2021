use std::collections::HashMap;
use std::fmt;
use std::fs;

#[derive(Eq, PartialEq, Hash)]
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
        let Point { x: x1, y: y1 } = start;
        let Point { x: x2, y: y2 } = end;
        if x1 == x2 {
            return LineSegment {
                start: Point {
                    x: x1,
                    y: y1.min(y2),
                },
                end: Point {
                    x: x1,
                    y: y1.max(y2),
                },
            };
        }
        if y1 == y2 {
            return LineSegment {
                start: Point {
                    x: x1.min(x2),
                    y: y1,
                },
                end: Point {
                    x: x1.max(x2),
                    y: y1,
                },
            };
        }
        if x1 < x2 {
            return LineSegment {
                start: Point { x: x1, y: y1 },
                end: Point { x: x2, y: y2 },
            };
        }
        return LineSegment {
            start: Point { x: x2, y: y2 },
            end: Point { x: x1, y: y1 },
        };
    }
    fn points_covered(&self) -> Vec<Point> {
        let Point { x: x1, y: y1 } = self.start;
        let Point { x: x2, y: y2 } = self.end;
        let mut points: Vec<Point> = Vec::new();
        if x1 == x2 {
            for i in y1..(y2 + 1) {
                points.push(Point { x: x1, y: i })
            }
        } else if y1 == y2 {
            for i in x1..(x2 + 1) {
                points.push(Point { x: i, y: y1 })
            }
        } else {
            // should always be positive according to the constructor
            let dx = x2 - x1;
            let dy = y2 - y1;
            if dx == dy.abs() {
                let mut yi = y1;
                for xi in x1..(x2 + 1) {
                    if dy > 0 {
                        points.push(Point { x: xi, y: yi });
                        yi += 1;
                    } else {
                        points.push(Point { x: xi, y: yi });
                        yi -= 1;
                    }
                }
            }
        }
        return points;
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

    println!(
        "points covered for {:?}: \n{:?}",
        lines[9],
        lines[9].points_covered()
    );

    println!("part 1: {}", part_1(lines));
}

fn part_1(lines: Vec<LineSegment>) -> i32 {
    let mut map: HashMap<Point, i32> = HashMap::new();
    for l in lines {
        let covered = l.points_covered();
        for point in covered.iter() {
            if map.contains_key(point) {
                *map.get_mut(point).unwrap() += 1;
            } else {
                map.insert(point.clone(), 1);
            }
        }
    }

    let mut overlaps = 0;
    for p in map.values() {
        if p > &1 {
            overlaps += 1;
        }
    }
    return overlaps;
}

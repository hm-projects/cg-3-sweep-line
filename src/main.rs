use std::cmp::{max, min};
use std::collections::BTreeMap;
use std::{env, fs};
use std::{num::ParseFloatError, str::FromStr};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
struct Point {
    x: f64,
    y: f64,
}

fn ccw(p: &Point, q: &Point, r: &Point) -> f64 {
    return (p.x * q.y - p.y * q.x) + (q.x * r.y - q.y * r.x) + (p.y * r.x - p.x * r.y);
}

impl Eq for Point {}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let x = f64::total_cmp(&self.x, &other.x);
        match x {
            std::cmp::Ordering::Equal => f64::total_cmp(&self.y, &other.y),
            _ => x,
        }
    }
}

impl Point {
    fn from_str(x: &str, y: &str) -> Result<Point, ParseFloatError> {
        let p = Point {
            x: x.parse()?,
            y: y.parse()?,
        };

        return Ok(p);
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
struct Line {
    p: Point,
    q: Point,
}

impl Eq for Line {}

#[derive(Debug)]
enum ParseLineError {
    ParseFloat(ParseFloatError),
    NotFourElements,
}

impl FromStr for Line {
    type Err = ParseLineError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let splits: Vec<_> = s.split(" ").collect();

        if splits.len() != 4 {
            return Err(ParseLineError::NotFourElements);
        }

        let p =
            Point::from_str(&splits[0], &splits[1]).map_err(|e| ParseLineError::ParseFloat(e))?;
        let q =
            Point::from_str(&splits[2], &splits[3]).map_err(|e| ParseLineError::ParseFloat(e))?;

        let line = Line { p, q };

        return Ok(line);
    }
}

fn overlap_for_colinear(p1: &Point, p2: &Point, q1: &Point, q2: &Point) -> bool {
    // x
    let p_smallest_x = p1.x.min(p2.x);
    let p_largest_x = p1.x.max(p2.x);
    let q_smallest_x = q1.x.min(q2.x);
    let q_largest_x = q1.x.max(q2.x);

    let qx_not_in_px = q_smallest_x > p_largest_x || q_largest_x < p_smallest_x;

    if qx_not_in_px {
        // early return
        return false;
    }

    // y
    let p_smallest_y = p1.y.min(p2.y);
    let p_largest_y = p1.y.max(p2.y);
    let q_smallest_y = q1.y.min(q2.y);
    let q_largest_y = q1.y.max(q2.y);

    let qy_not_in_py = q_smallest_y > p_largest_y || q_largest_y < p_smallest_y;

    return !qy_not_in_py;
}

fn intersect(p1: &Point, p2: &Point, q1: &Point, q2: &Point) -> bool {
    // let overlap = overlap_for_colinear(p1, p2, q1, q2);
    // if !overlap {
    //     return false;
    // }

    let ccwq1 = ccw(p1, p2, q1);
    let ccwq2 = ccw(p1, p2, q2);
    if ccwq1 * ccwq2 > 0.0 {
        return false;
    }

    let ccwp1 = ccw(q1, q2, p1);
    let ccwp2 = ccw(q1, q2, p2);
    if ccwp1 * ccwp2 > 0.0 {
        return false;
    }

    if ccwq1 == 0.0 && ccwq2 == 0.0 && ccwp1 == 0.0 && ccwp2 == 0.0 {
        // lines are colinear --> check for overlap
        return overlap_for_colinear(p1, p2, q1, q2);
    }

    return true;
}

impl Line {
    fn intersection(&self, other: &Line) -> Option<Point> {
        let intersect = intersect(&self.p, &self.q, &other.p, &other.q);

        todo!()
    }
}

#[derive(Debug, PartialEq)]
enum Event<'a> {
    Begin {
        point: &'a Point,
        line: &'a Line,
    },
    End {
        point: &'a Point,
        line: &'a Line,
    },
    Intersection {
        point: &'a Point,
        line: &'a Line,
        other_line: &'a Line,
    },
}

impl Event<'_> {
    fn point(&self) -> &Point {
        match self {
            Event::Begin { point, .. } => point,
            Event::End { point, .. } => point,
            Event::Intersection { point, .. } => point,
        }
    }
}

impl PartialOrd for Event<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        return self.point().partial_cmp(other.point());
    }
}

fn initialize(lines: &Vec<Line>) -> BTreeMap<&Point, Event> {
    let mut queue = BTreeMap::new();

    for line in lines {
        let smaller = min(&line.p, &line.q);
        let larger = max(&line.p, &line.q);

        let start = Event::Begin {
            point: smaller,
            line: &line,
        };
        queue.insert(smaller, start);

        let end = Event::End {
            point: larger,
            line: &line,
        };
        queue.insert(larger, end);
    }

    return queue;
}

#[derive(Debug, Clone)]
struct SweepLineElement {
    y: f64,
    line: Line,
}

impl PartialEq for SweepLineElement {
    fn eq(&self, other: &Self) -> bool {
        self.y == other.y
    }
}
impl Eq for SweepLineElement {}

impl PartialOrd for SweepLineElement {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.y.partial_cmp(&other.y)
    }
}

impl Ord for SweepLineElement {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        f64::total_cmp(&self.y, &other.y)
    }
}

fn sweep_line_intersections(mut queue: BTreeMap<&Point, Event>) -> i64 {
    let mut segments: Vec<SweepLineElement> = Vec::new();
    let mut intersections = 0;

    while let Some((_, event)) = queue.pop_first() {
        match event {
            Event::Begin { point, line } => {
                segments.push(SweepLineElement {
                    y: point.y,
                    line: line.clone(),
                });
                segments.sort();

                let index_line = segments.iter().position(|e| &e.line == line).unwrap();
                let line = segments[index_line].clone();
                let line_above = segments[index_line + 1].clone();
                //let line_below = segments[index_line - 1].clone();

                if let Some(inter) = line.line.intersection(&line_above.line) {
                    let key = inter.clone();
                    queue.insert(
                        &key,
                        Event::Intersection {
                            point: &inter,
                            line: &line.line,
                            other_line: &line_above.line,
                        },
                    );
                }

                // if let Some(intersection_above) = line.line.intersection(&line_above.line) {
                //     let key = intersection_above.clone();
                //     queue.insert(
                //         &key,
                //         Event::Intersection {
                //             point: &intersection_above.clone(),
                //             line: &line.line,
                //             other_line: &line_above.line,
                //         },
                //     );
                // };
                // if let Some(intersection_below) = line.line.intersection(&line_below.line) {
                //     queue.insert(
                //         &intersection_below,
                //         Event::Intersection {
                //             point: &intersection_below,
                //             line: &line.line,
                //             other_line: &line_above.line,
                //         },
                //     );
                // };

                // TODO: detect change in order of line segments
                // TODO: if changed calc xy of intersection and add to queue
            }
            Event::End { point: _, line } => {
                let index = segments.iter().position(|e| &e.line == line).unwrap();
                segments.remove(index);
            }
            Event::Intersection {
                point: _,
                line,
                other_line,
            } => {
                let index_line = segments.iter().position(|e| &e.line == line).unwrap();
                let index_other_line = segments.iter().position(|e| &e.line == other_line).unwrap();

                // TODO: this is probably incorrect
                let y = segments[index_line].y;
                let other_y = segments[index_line].y;
                segments[index_line].y = other_y;
                segments[index_other_line].y = y;

                //segments.swap(index_line, index_other_line);
                segments.sort();

                intersections += 1;
            }
        }
    }

    intersections
}

fn read_file(file: &str) -> Vec<Line> {
    let contents = fs::read_to_string(file).expect("Should have been able to read the file");

    contents
        .lines()
        .map(|l| l.parse().expect("Failed to parse a line"))
        .collect()
}

fn main() {
    let params = env::args().collect::<Vec<_>>();

    for param in params.iter().skip(1) {
        let lines = read_file(param);

        let mut queue = initialize(&lines);
        println!("{:#?}", queue);
        let intersections = sweep_line_intersections(queue);
        println!("intersects: {}", intersections);

        //println!("{:#?}", queue);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        let s = "0 1 2 3";
        let line = Line::from_str(s);
        assert!(line.is_ok());
        let line = line.unwrap();
        assert_eq!(line.p, Point { x: 0.0, y: 1.0 });
        assert_eq!(line.q, Point { x: 2.0, y: 3.0 });
    }

    #[test]
    fn test_point_order() {
        let p = Point { x: 0.0, y: 1.0 };
        let q = Point { x: 2.0, y: 3.0 };

        assert!(p < q);
        assert!(!(q < p));
        assert!(p == p);
        assert!(q == q);

        let q2 = Point { x: 0.0, y: 0.5 };
        assert!(q2 < p);
        assert!(p > q2);
    }
}

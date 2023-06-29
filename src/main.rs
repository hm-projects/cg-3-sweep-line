use std::cmp::{max, min};
use std::collections::BTreeSet;
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
        let p1 = &self.p;
        let p2 = &self.q;
        let q1 = &other.p;
        let q2 = &other.q;

        let ccwq1 = ccw(p1, p2, q1);
        let ccwq2 = ccw(p1, p2, q2);
        if ccwq1 * ccwq2 > 0.0 {
            return None;
        }

        let ccwp1 = ccw(q1, q2, p1);
        let ccwp2 = ccw(q1, q2, p2);
        if ccwp1 * ccwp2 > 0.0 {
            return None;
        }

        if ccwq1 == 0.0 && ccwq2 == 0.0 && ccwp1 == 0.0 && ccwp2 == 0.0 {
            panic!("Two colinear lines were detected: {:?}, {:?}", self, other);
            // lines are colinear --> check for overlap
            // let overlap = overlap_for_colinear(p1, p2, q1, q2);
            // if overlap {
            //     return Some(Point { x: 0., y: 0. });
            // } else {
            //     return None;
            // }
        }

        // Determine intersection point
        let r_ab = (ccwq2 / ccwq1).abs();
        let a = r_ab / (r_ab + 1.0);
        let i_x = q2.x + a * (q1.x - q2.x);
        let i_y = q2.y + a * (q1.y - q2.y);

        Some(Point { x: i_x, y: i_y })
    }
}

#[derive(Debug, PartialEq)]
enum Event {
    Begin {
        point: Point,
        line: Line,
    },
    End {
        point: Point,
        line: Line,
    },
    Intersection {
        point: Point,
        line: Line,
        other_line: Line,
    },
}

impl Event {
    fn point(&self) -> &Point {
        match self {
            Event::Begin { point, .. } => point,
            Event::End { point, .. } => point,
            Event::Intersection { point, .. } => point,
        }
    }
}

impl Eq for Event {}

impl PartialOrd for Event {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        return self.point().partial_cmp(other.point());
    }
}

impl Ord for Event {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.point().cmp(other.point())
    }
}

fn initialize(lines: Vec<Line>) -> BTreeSet<Event> {
    let mut queue = BTreeSet::new();

    for line in lines {
        let smaller = min(&line.p, &line.q);
        let larger = max(&line.p, &line.q);

        let start = Event::Begin {
            point: smaller.to_owned(),
            line: line.clone(),
        };
        queue.insert(start);

        let end = Event::End {
            point: larger.to_owned(),
            line,
        };
        queue.insert(end);
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

fn sweep_line_intersections(mut queue: BTreeSet<Event>) -> i64 {
    let mut segments: Vec<SweepLineElement> = Vec::new();
    let mut intersections = 0;

    while let Some(event) = queue.pop_first() {
        match event {
            Event::Begin { point, line } => {
                segments.push(SweepLineElement {
                    y: point.y,
                    line: line.clone(),
                });
                segments.sort();

                let index_line = segments.iter().position(|e| &e.line == &line).unwrap();
                let line = segments[index_line].clone();

                if let Some(line_above) = segments.get(index_line + 1) {
                    if let Some(inter) = line.line.intersection(&line_above.line) {
                        queue.insert(Event::Intersection {
                            point: inter,
                            line: line.line.clone(),
                            other_line: line_above.line.clone(),
                        });
                    };
                };

                if index_line > 0 {
                    if let Some(line_below) = segments.get(index_line - 1).clone() {
                        if let Some(inter) = line.line.intersection(&line_below.line) {
                            queue.insert(Event::Intersection {
                                point: inter,
                                line: line.line,
                                other_line: line_below.line.clone(),
                            });
                        };
                    };
                }
            }
            Event::End { point: _, line } => {
                let index = segments.iter().position(|e| &e.line == &line).unwrap();

                let index_line = segments.iter().position(|e| &e.line == &line).unwrap();

                if index_line > 0 {
                    if let Some(line_below) = segments.get(index_line - 1).clone() {
                        if let Some(line_above) = segments.get(index_line + 1) {
                            if let Some(inter) = line_below.line.intersection(&line_above.line) {
                                queue.insert(Event::Intersection {
                                    point: inter,
                                    line: line_below.line.clone(),
                                    other_line: line_above.line.clone(),
                                });
                            };
                        };
                    };
                }

                segments.remove(index);
            }
            Event::Intersection {
                point: _,
                line,
                other_line,
            } => {
                let index_line = segments.iter().position(|e| &e.line == &line).unwrap();
                let index_other_line = segments
                    .iter()
                    .position(|e| &e.line == &other_line)
                    .unwrap();

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

        let queue = initialize(lines);
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

    #[test]
    fn test_intersect() {
        let line1 = Line::from_str("0 0 1 1").expect("Failed to parse first line");
        let line2_s = Line::from_str("0 1 1 0").expect("Failed to parse second line");

        assert!(line1.intersection(&line2_s).is_some());
        assert_eq!(
            line1.intersection(&line2_s).unwrap(),
            Point { x: 0.5, y: 0.5 }
        );
    }

    #[test]
    fn test_intersect_cross() {
        let line1 = Line::from_str("0 1 2 1").expect("Failed to parse first line");
        let line2_s = Line::from_str("1 2 1 0").expect("Failed to parse second line");

        assert!(line1.intersection(&line2_s).is_some());
        assert_eq!(
            line1.intersection(&line2_s).unwrap(),
            Point { x: 1.0, y: 1.0 }
        );
    }
}

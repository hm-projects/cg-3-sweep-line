use std::cmp::{max, min};
use std::collections::BTreeSet;
use std::fmt::{self, Display};
use std::{env, fs};
use std::{num::ParseFloatError, str::FromStr};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
struct Point {
    x: f64,
    y: f64,
}

impl Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.x, self.y)
    }
}

fn ccw(p: &Point, q: &Point, r: &Point) -> f64 {
    (p.x * q.y - p.y * q.x) + (q.x * r.y - q.y * r.x) + (p.y * r.x - p.x * r.y)
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

        Ok(p)
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
        let splits: Vec<_> = s.split(' ').collect();

        if splits.len() != 4 {
            return Err(ParseLineError::NotFourElements);
        }

        let p = Point::from_str(splits[0], splits[1]).map_err(ParseLineError::ParseFloat)?;
        let q = Point::from_str(splits[2], splits[3]).map_err(ParseLineError::ParseFloat)?;

        let line = Line { p, q };

        Ok(line)
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

    !qy_not_in_py
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

    true
}

impl Line {
    fn len(&self) -> f64 {
        let dx = self.p.x - self.q.x;
        let dy = self.p.y - self.q.y;
        f64::sqrt(dx * dx + dy * dy)
    }

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

#[derive(Debug)]
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

impl PartialEq for Event {
    fn eq(&self, other: &Self) -> bool {
        self.point() == other.point()
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
        if line.p.x == line.q.x {
            panic!("Vertical line detected: {:?}", line)
        }

        if line.len() < 0.0 {
            panic!("Line segment with 0 length detected: {:?}", line)
        }

        let smaller = min(&line.p, &line.q);
        let larger = max(&line.p, &line.q);

        let start = Event::Begin {
            point: smaller.to_owned(),
            line: line.clone(),
        };
        if queue.contains(&start) {
            panic!("Duplicate point detected: {:?}", start)
        }
        queue.insert(start);

        let end = Event::End {
            point: larger.to_owned(),
            line,
        };
        if queue.contains(&end) {
            panic!("Duplicate point detected: {:?}", end)
        }
        queue.insert(end);
    }

    queue
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
        // Reverse order
        other.y.partial_cmp(&self.y)
    }
}

impl Ord for SweepLineElement {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Reverse order
        f64::total_cmp(&other.y, &self.y)
    }
}

fn sweep_line_intersections(mut queue: BTreeSet<Event>) -> Vec<Point> {
    let mut segments: Vec<SweepLineElement> = Vec::new();
    let mut intersections_set: BTreeSet<Point> = BTreeSet::new();

    let mut last_x = 0.0;

    while let Some(event) = queue.pop_first() {
        if event.point().x < last_x {
            println!("Sweep line went backwards: {} {:?}", last_x, event)
        }
        last_x = event.point().x;
        //println!("{:?}", &event);
        //println!("{:?}", &segments);
        //println!("{:?}", &queue);
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
                        if !intersections_set.contains(&inter) {
                            intersections_set.insert(inter.clone());
                            queue.insert(Event::Intersection {
                                point: inter,
                                line: line.line.clone(),
                                other_line: line_above.line.clone(),
                            });
                        }
                    };
                };

                if index_line > 0 {
                    if let Some(line_below) = segments.get(index_line - 1) {
                        if let Some(inter) = line.line.intersection(&line_below.line) {
                            if !intersections_set.contains(&inter) {
                                intersections_set.insert(inter.clone());
                                queue.insert(Event::Intersection {
                                    point: inter,
                                    line: line.line,
                                    other_line: line_below.line.clone(),
                                });
                            }
                        };
                    };
                }
            }
            Event::End { point: _, line } => {
                let index_line = segments.iter().position(|e| &e.line == &line).unwrap();

                if index_line > 0 {
                    if let Some(line_below) = segments.get(index_line - 1) {
                        if let Some(line_above) = segments.get(index_line + 1) {
                            if let Some(inter) = line_below.line.intersection(&line_above.line) {
                                if !intersections_set.contains(&inter) {
                                    intersections_set.insert(inter.clone());
                                    queue.insert(Event::Intersection {
                                        point: inter,
                                        line: line_below.line.clone(),
                                        other_line: line_above.line.clone(),
                                    });
                                };
                            };
                        };
                    };
                }

                segments.remove(index_line);
            }
            Event::Intersection {
                point: intersection_point,
                line,
                other_line,
            } => {
                let index_line = segments.iter().position(|e| &e.line == &line).unwrap();
                let index_other_line = segments
                    .iter()
                    .position(|e| &e.line == &other_line)
                    .unwrap();

                if index_line.abs_diff(index_other_line) != 1 {
                    println!(
                        "Two lines with indices too far apart: {}, {}. \nSegments are: {:?}",
                        index_line, index_other_line, segments
                    )
                }

                if index_line < index_other_line {
                    segments[index_line].y = intersection_point.y;
                    segments[index_other_line].y = intersection_point.y + 0.000000001;
                } else {
                    segments[index_line].y = intersection_point.y + 0.000000001;
                    segments[index_other_line].y = intersection_point.y;
                }

                //segments.swap(index_line, index_other_line);
                segments.sort();

                let smaller = index_line.min(index_other_line);
                let bigger = index_line.max(index_other_line);

                if let Some(line_above) = segments.get(bigger + 1) {
                    if let Some(line) = segments.get(bigger) {
                        if let Some(inter) = line.line.intersection(&line_above.line) {
                            if !intersections_set.contains(&inter) {
                                intersections_set.insert(inter.clone());
                                queue.insert(Event::Intersection {
                                    point: inter,
                                    line: line.line.clone(),
                                    other_line: line_above.line.clone(),
                                });
                            }
                        };
                    };
                };

                if smaller > 0 {
                    if let Some(line_below) = segments.get(smaller - 1) {
                        if let Some(line) = segments.get(smaller) {
                            if let Some(inter) = line.line.intersection(&line_below.line) {
                                if !intersections_set.contains(&inter) {
                                    intersections_set.insert(inter.clone());
                                    queue.insert(Event::Intersection {
                                        point: inter,
                                        line: line.line.clone(),
                                        other_line: line_below.line.clone(),
                                    });
                                }
                            };
                        };
                    };
                }
                intersections_set.insert(intersection_point.clone());
            }
        }
    }

    intersections_set.into_iter().collect()
}

fn read_file(file: &str) -> Vec<Line> {
    let contents = fs::read_to_string(file).expect("Should have been able to read the file");

    contents
        .lines()
        .map(|l| l.parse().expect("Failed to parse a line"))
        .collect()
}

fn main() {
    //let params = env::args().collect::<Vec<_>>();
    let params = vec!["", "./data/s_1000_10.dat"];

    for param in params.iter().skip(1) {
        let lines = read_file(param);

        let queue = initialize(lines);
        let intersections = sweep_line_intersections(queue);
        intersections
            .iter()
            .map(|p| format!("{}", p))
            .for_each(|p| println!("{}", p));
        println!("intersections: {}", intersections.len());

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
        assert!(q >= p);
        assert!(p == p);
        assert!(q == q);

        let q2 = Point { x: 0.0, y: 0.5 };
        assert!(q2 < p);
        assert!(p > q2);
    }

    #[test]
    fn test_init() {
        let p = Point { x: 1.0, y: 1.0 };
        let q = Point { x: 0.0, y: 0.0 };
        let line = Line {
            p: p.clone(),
            q: q.clone(),
        };
        let mut queue = initialize(vec![line]);

        assert_eq!(queue.len(), 2);

        let first = queue.pop_first().unwrap();
        assert_eq!(first.point(), &q);

        let second = queue.pop_first().unwrap();
        assert_eq!(second.point(), &p);
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

    #[test]
    fn test_trivial_sweep() {
        let p = Point { x: 0.0, y: 1.0 };
        let q = Point { x: 5.0, y: 1.0 };
        let line = Line { p, q };
        let p2 = Point { x: 1.0, y: 0.0 };
        let q2 = Point { x: 4.0, y: 2.0 };
        let line2 = Line { p: p2, q: q2 };
        let queue = initialize(vec![line, line2]);
        let intersections = sweep_line_intersections(queue);

        assert_eq!(intersections.len(), 1);
        assert_eq!(intersections[0].x, 2.5);
        assert_eq!(intersections[0].y, 1.0);
    }

    #[test]
    fn test_three_lines() {
        let l1 = Line::from_str("0 1 5 1").unwrap();
        let l2 = Line::from_str("1.5 2.5 4 0.5").unwrap();
        let l3 = Line::from_str("0.5 1.5 4 2.5").unwrap();

        let queue = initialize(vec![l1, l2, l3]);
        let intersections = sweep_line_intersections(queue);

        assert_eq!(intersections.len(), 2);
        assert_eq!(intersections[0].x, 2.1578947368421053);
        assert_eq!(intersections[0].y, 1.973684210526316);

        assert_eq!(intersections[1].x, 3.375);
        assert_eq!(intersections[1].y, 1.0);
    }

    #[test]
    fn test_three_lines_different_order() {
        let l1 = Line::from_str("0 1 5 1").unwrap();
        let l2 = Line::from_str("1 2.5 4 0.5").unwrap();
        let l3 = Line::from_str("1.5 1.5 4 2.5").unwrap();

        let queue = initialize(vec![l1, l2, l3]);
        let intersections = sweep_line_intersections(queue);

        assert_eq!(intersections.len(), 2);
        assert_eq!(intersections[0].x, 2.125);
        assert_eq!(intersections[0].y, 1.75);

        assert_eq!(intersections[1].x, 3.25);
        assert_eq!(intersections[1].y, 1.0);
    }

    #[test]
    fn test_three_lines_same_end_x() {
        let l1 = Line::from_str("0 1 5 1").unwrap();
        let l2 = Line::from_str("1.5 2 2.5 1.5").unwrap();
        let l3 = Line::from_str("0.5 0.5 2.5 2").unwrap();

        let queue = initialize(vec![l1, l2, l3]);
        let intersections = sweep_line_intersections(queue);

        assert_eq!(intersections.len(), 2);
        assert_eq!(intersections[0].x, 1.166666666666667);
        assert_eq!(intersections[0].y, 1.0);

        assert_eq!(intersections[1].x, 2.1);
        assert_eq!(intersections[1].y, 1.7);
    }

    #[test]
    fn test_four_lines() {
        let l1 = Line::from_str("0 1 5 1").unwrap();
        let l2 = Line::from_str("1 1.5 2 0.5").unwrap();
        let l3 = Line::from_str("1.5 0.5 3 2").unwrap();
        let l4 = Line::from_str("2 2 3.5 0.5").unwrap();

        let queue = initialize(vec![l1, l2, l3, l4]);
        let intersections = sweep_line_intersections(queue);

        assert_eq!(intersections.len(), 5);

        assert_eq!(intersections[0].x, 1.4999999999999996); // floating point shenanigans
        assert_eq!(intersections[0].y, 1.0);

        assert_eq!(intersections[1].x, 1.75);
        assert_eq!(intersections[1].y, 0.75);

        assert_eq!(intersections[2].x, 2.0);
        assert_eq!(intersections[2].y, 1.0);

        assert_eq!(intersections[3].x, 2.5);
        assert_eq!(intersections[3].y, 1.5);

        assert_eq!(intersections[4].x, 3.0);
        assert_eq!(intersections[4].y, 1.0);
    }
}

use std::cmp::{max, min};
use std::collections::BTreeMap;
use std::{env, fs};
use std::{num::ParseFloatError, str::FromStr};

#[derive(Debug, PartialEq, PartialOrd)]
struct Point {
    x: f64,
    y: f64,
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

#[derive(Debug, PartialEq)]
struct Line {
    p: Point,
    q: Point,
}

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

fn sweep_line_intersections(queue: &mut BTreeMap<&Point, Event>) -> i64 {
    let mut segments = Vec::new();
    let mut intersections = 0;

    while let Some((_, event)) = queue.pop_first() {
        match event {
            Event::Begin { point: _, line } => {
                segments.push(line);
            }
            Event::End { point: _, line } => {
                let index = segments.iter().position(|l| l == &line).expect(
                    format!(
                        "could not find line to remove in segments, {:?} not in {:?}",
                        line, segments,
                    )
                    .as_str(),
                );
                segments.remove(index);
            }
            Event::Intersection {
                point: _,
                line: _,
                other_line: _,
            } => {
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
        let intersections = sweep_line_intersections(&mut queue);
        println!("intersects: {}", intersections);

        println!("{:#?}", queue);
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

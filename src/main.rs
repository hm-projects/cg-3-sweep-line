use std::{num::ParseFloatError, str::FromStr};

#[derive(Debug, PartialEq, PartialOrd)]
struct Point {
    x: f64,
    y: f64,
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

#[derive(Debug)]
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

fn main() {
    println!("Hello, world!");
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

use std::{str::FromStr, num::ParseFloatError};

#[derive(Debug)]
struct Point {
    x: f64,
    y: f64
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
    q: Point
}

enum ParseLineError {
    ParseFloat(ParseFloatError),
    NotFourElements
}

impl FromStr for Line {
    type Err = ParseLineError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let splits: Vec<_> = s.split(" ").collect();

        if splits.len() != 4 {
            return Err(ParseLineError::NotFourElements)
        }

        let p = Point::from_str(&splits[0], &splits[1]).map_err(|e| ParseLineError::ParseFloat(e))?;
        let q = Point::from_str(&splits[2], &splits[3]).map_err(|e| ParseLineError::ParseFloat(e))?;

        let line = Line {
            p,
            q,
        };

        return Ok(line);
    }
}

fn main() {
    println!("Hello, world!");
}

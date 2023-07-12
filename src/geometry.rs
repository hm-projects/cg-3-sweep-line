use std::{
    fmt::{self, Display},
    num::ParseFloatError,
    str::FromStr,
};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Point {
    pub x: f64,
    pub y: f64,
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
pub struct Line {
    pub p: Point,
    pub q: Point,
}

impl Eq for Line {}

#[derive(Debug)]
pub enum ParseLineError {
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

impl Line {
    pub fn len(&self) -> f64 {
        let dx = self.p.x - self.q.x;
        let dy = self.p.y - self.q.y;
        f64::sqrt(dx * dx + dy * dy)
    }

    pub fn intersection(&self, other: &Line) -> Option<Point> {
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

    pub fn y(&self, x: f64) -> f64 {
        // calculate the lines y value at a certain x value
        let m = (self.p.y - self.q.y) / (self.p.x - self.q.x);

        let y = m * (x - self.p.x) + self.p.y;
        y
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_y() {
        let line = Line {
            p: Point { x: 0., y: 0. },
            q: Point { x: 1., y: 1. },
        };

        assert_eq!(line.y(0.5), 0.5);
        assert_eq!(line.y(0.), 0.);
        assert_eq!(line.y(1.), 1.);
    }

    // test for line with no m
    #[test]
    fn test_y_no_m() {
        let line = Line {
            p: Point { x: 0., y: 0. },
            q: Point { x: 1., y: 0. },
        };

        assert_eq!(line.y(0.5), 0.);
        assert_eq!(line.y(0.), 0.);
        assert_eq!(line.y(1.), 0.);
    }
}

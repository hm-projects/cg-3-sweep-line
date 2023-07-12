use std::{
    cmp::{max, min},
    collections::BTreeSet,
};

use crate::geometry::{Line, Point};

#[derive(Debug)]
pub enum Event {
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
    pub fn point(&self) -> &Point {
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
        // compare events first by their point, if point is equal then prefer Intersection over End
        let point_cmp = self.point().partial_cmp(other.point());
        match point_cmp {
            Some(std::cmp::Ordering::Equal) => match (self, other) {
                (Event::Intersection { .. }, Event::End { .. }) => Some(std::cmp::Ordering::Less),
                (Event::End { .. }, Event::Intersection { .. }) => {
                    Some(std::cmp::Ordering::Greater)
                }
                _ => Some(std::cmp::Ordering::Equal),
            },
            _ => point_cmp,
        }
    }
}

impl Ord for Event {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // compare events first by their point, if point is equal then prefer Intersection over End
        let point_cmp = self.point().cmp(other.point());
        match point_cmp {
            std::cmp::Ordering::Equal => match (self, other) {
                (Event::Intersection { .. }, Event::End { .. }) => std::cmp::Ordering::Less,
                (Event::End { .. }, Event::Intersection { .. }) => std::cmp::Ordering::Greater,
                _ => std::cmp::Ordering::Equal,
            },
            _ => point_cmp,
        }
    }
}

pub fn initialize(lines: Vec<Line>) -> BTreeSet<Event> {
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

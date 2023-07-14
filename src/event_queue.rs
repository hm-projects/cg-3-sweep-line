use std::{
    cmp::{max, min},
    collections::BTreeSet,
};

use crate::{
    geometry::{Line, Point},
    sweep_line::SweepLine,
};

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
        Some(self.cmp(other))
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

pub struct EventQueue {
    last_x: f64,
    queue: BTreeSet<Event>,
    pub intersection_points: BTreeSet<Point>,
}

impl EventQueue {
    pub fn new(lines: Vec<Line>) -> Self {
        let mut events: EventQueue = Self {
            last_x: 0.0,
            queue: BTreeSet::new(),
            intersection_points: BTreeSet::new(),
        };

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
            if events.queue.contains(&start) {
                panic!("Duplicate point detected: {:?}", start)
            }
            events.queue.insert(start);

            let end = Event::End {
                point: larger.to_owned(),
                line,
            };
            if events.queue.contains(&end) {
                panic!("Duplicate point detected: {:?}", end)
            }
            events.queue.insert(end);
        }
        events
    }

    pub fn pop_first(&mut self) -> Option<Event> {
        let event = self.queue.pop_first();
        if let Some(event) = &event {
            if event.point().x < self.last_x {
                panic!("Sweep line went backwards!");
            }
            self.last_x = event.point().x;
        };
        event
    }

    pub fn add_intersection_event(
        &mut self,
        intersection_point: Point,
        line: &Line,
        other_line: &Line,
    ) {
        let intersection_point = intersection_point.round(9);
        if intersection_point.x > self.last_x
            && !self.intersection_points.contains(&intersection_point)
        {
            self.intersection_points.insert(intersection_point.clone());
            self.queue.insert(Event::Intersection {
                point: intersection_point,
                line: line.clone(),
                other_line: other_line.clone(),
            });
        }
    }

    pub fn sweep(mut self) -> BTreeSet<Point> {
        let mut sweep_line = SweepLine::new();

        while let Some(event) = self.pop_first() {
            sweep_line.update(event.point().x);
            match event {
                Event::Begin { point, line } => {
                    sweep_line.insert(point.y, line.clone());

                    let neighbors = sweep_line.get_neighbors(&line);
                    let Some(neighbors) = neighbors else {
                        panic!("Line not found in sweep line, but was just inserted: {:?}", line);
                    };

                    if let Some(line_above) = neighbors.above {
                        if let Some(inter) = line.intersection(&line_above.line) {
                            self.add_intersection_event(inter, &line, &line_above.line);
                        };
                    };

                    if let Some(line_below) = neighbors.below {
                        if let Some(inter) = line.intersection(&line_below.line) {
                            self.add_intersection_event(inter, &line, &line_below.line);
                        };
                    };
                }
                Event::End { point: _, line } => {
                    let neighbors = sweep_line.get_neighbors(&line);

                    let Some(neighbors) = neighbors else {
                        panic!("Line not found in sweep line, should be removed now: {:?}", line);
                    };

                    if let (Some(line_below), Some(line_above)) = (neighbors.below, neighbors.above)
                    {
                        if let Some(inter) = line_below.line.intersection(&line_above.line) {
                            self.add_intersection_event(inter, &line_below.line, &line_above.line);
                        };
                    };

                    sweep_line.remove(&line);
                }
                Event::Intersection {
                    point: intersection_point,
                    line,
                    other_line,
                } => {
                    let swapped = sweep_line.swap_and_get_new_neighbors(
                        &line,
                        &other_line,
                        &intersection_point,
                    );

                    if let (line, Some(line_above)) = (swapped.bigger, swapped.above) {
                        if let Some(inter) = line.line.intersection(&line_above.line) {
                            self.add_intersection_event(inter, &line.line, &line_above.line);
                        };
                    };

                    if let (line, Some(line_below)) = (swapped.smaller, swapped.below) {
                        if let Some(inter) = line.line.intersection(&line_below.line) {
                            self.add_intersection_event(inter, &line.line, &line_below.line);
                        };
                    };
                }
            };
        }

        return self.intersection_points;
    }
}

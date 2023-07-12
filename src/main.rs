mod geometry;
mod sweep_line;

use std::cmp::{max, min};
use std::collections::BTreeSet;
use std::fs;

use geometry::{Line, Point};
use sweep_line::SweepLineElement;

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
                        if inter.x > last_x && !intersections_set.contains(&inter) {
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
                            if inter.x > last_x && !intersections_set.contains(&inter) {
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
                                if inter.x > last_x && !intersections_set.contains(&inter) {
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
                            if inter.x > last_x && !intersections_set.contains(&inter) {
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
                                if inter.x > last_x && !intersections_set.contains(&inter) {
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
    use std::str::FromStr;

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

    #[test]
    fn test_three_lines_close_and_reorder() {
        // TODO: this test provokes a "Two lines with indices too far apart: 0, 2." warning
        let l1 = Line::from_str("0 0.5 3 0.5").unwrap();
        let l2 = Line::from_str("0.5 1 2 0.2").unwrap();
        let l3 = Line::from_str("1 0.8 1.8 0.8").unwrap();

        let queue = initialize(vec![l1, l2, l3]);
        let intersections = sweep_line_intersections(queue);

        assert_eq!(intersections.len(), 1);

        assert_eq!(intersections[0].x, 1.4375);
        assert_eq!(intersections[0].y, 0.5);
    }
}

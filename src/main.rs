mod event_queue;
mod geometry;
mod sweep_line;

use std::fs;
use std::{collections::BTreeSet, io::Write};

use event_queue::Event;
use geometry::{Line, Point};
use sweep_line::SweepLine;

use crate::event_queue::initialize;

fn sweep_line_intersections(mut queue: BTreeSet<Event>) -> Vec<Point> {
    let mut sweep_line = SweepLine::new();
    let mut intersections_set: BTreeSet<Point> = BTreeSet::new();

    let mut last_x = 0.0;

    while let Some(event) = queue.pop_first() {
        if event.point().x < last_x {
            panic!("Sweep line went backwards!");
        }
        last_x = event.point().x;
        sweep_line.update(event.point().x);
        //println!("{:?}", &event);
        //println!("{:?}", &segments);
        //println!("{:?}", &queue);
        match event {
            Event::Begin { point, line } => {
                sweep_line.insert(point.y, line.clone());

                let neighbors = sweep_line.get_neighbors(line.clone());
                let Some(neighbors) = neighbors else {
                    panic!("Line not found in sweep line, but was just inserted: {:?}", line);
                };

                if let Some(line_above) = neighbors.above {
                    if let Some(inter) = line.intersection(&line_above.line) {
                        if inter.x >= last_x && !intersections_set.contains(&inter) {
                            let inter = inter.round(9);
                            intersections_set.insert(inter.clone());
                            queue.insert(Event::Intersection {
                                point: inter,
                                line: line.clone(),
                                other_line: line_above.line.clone(),
                            });
                        }
                    };
                };

                if let Some(line_below) = neighbors.below {
                    if let Some(inter) = line.intersection(&line_below.line) {
                        if inter.x >= last_x && !intersections_set.contains(&inter) {
                            let inter = inter.round(9);
                            intersections_set.insert(inter.clone());
                            queue.insert(Event::Intersection {
                                point: inter,
                                line: line.clone(),
                                other_line: line_below.line.clone(),
                            });
                        }
                    };
                };
            }
            Event::End { point: _, line } => {
                let neighbors = sweep_line.get_neighbors(line.clone());

                let Some(neighbors) = neighbors else {
                    panic!("Line not found in sweep line, should be removed now: {:?}", line);
                };

                if let (Some(line_below), Some(line_above)) = (neighbors.below, neighbors.above) {
                    if let Some(inter) = line_below.line.intersection(&line_above.line) {
                        if inter.x >= last_x && !intersections_set.contains(&inter) {
                            let inter = inter.round(9);
                            intersections_set.insert(inter.clone());
                            queue.insert(Event::Intersection {
                                point: inter,
                                line: line_below.line.clone(),
                                other_line: line_above.line.clone(),
                            });
                        }
                    };
                };

                sweep_line.remove(line.clone());
            }
            Event::Intersection {
                point: intersection_point,
                line,
                other_line,
            } => {
                let swapped = sweep_line.swap_and_get_new_neighbors(
                    line.clone(),
                    other_line.clone(),
                    &intersection_point,
                );

                if let (line, Some(line_above)) = (swapped.bigger, swapped.above) {
                    if let Some(inter) = line.line.intersection(&line_above.line) {
                        if inter.x >= last_x && !intersections_set.contains(&inter) {
                            let inter = inter.round(9);
                            intersections_set.insert(inter.clone());
                            queue.insert(Event::Intersection {
                                point: inter,
                                line: line.line.clone(),
                                other_line: line_above.line.clone(),
                            });
                        }
                    };
                };

                if let (line, Some(line_below)) = (swapped.smaller, swapped.below) {
                    if let Some(inter) = line.line.intersection(&line_below.line) {
                        if inter.x >= last_x && !intersections_set.contains(&inter) {
                            let inter = inter.round(9);
                            intersections_set.insert(inter.clone());
                            queue.insert(Event::Intersection {
                                point: inter,
                                line: line.line.clone(),
                                other_line: line_below.line.clone(),
                            });
                        }
                    };
                };

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
        println!("intersections: {}", intersections.len());

        // create a new file "i_<filename>" with the intersections
        let filename = format!("{}.i", param);
        // delete file if it exists
        if fs::metadata(&filename).is_ok() {
            fs::remove_file(&filename).expect("Failed to delete file");
        }
        println!("Writing intersections to file {}", filename);
        let mut file = fs::File::create(filename).expect("Failed to create file");
        intersections
            .iter()
            .map(|p| format!("{}", p))
            .for_each(|p| writeln!(file, "{}", p).expect("Failed to write to file"));

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

    #[test]
    fn test_dunno() {
        let l1 = Line::from_str("0 0.5 3 0.5").unwrap();
        let l2 = Line::from_str("0.5 1 2 0.2").unwrap();
        let l3 = Line::from_str("1 0.8 1.8 0.8").unwrap();
        let l4 = Line::from_str("1.1 0.6 1.4 1").unwrap();

        let queue = initialize(vec![l1, l2, l3, l4]);
        let intersections = sweep_line_intersections(queue);

        println!("{:#?}", intersections);
        assert_eq!(intersections.len(), 3);

        assert_eq!(intersections[0].x, 1.142857142857143);
        assert_eq!(intersections[0].y, 0.657142857142857);

        assert_eq!(intersections[1].x, 1.2499999999999998);
        assert_eq!(intersections[1].y, 0.8);

        assert_eq!(intersections[2].x, 1.4375);
        assert_eq!(intersections[2].y, 0.5);
    }
}

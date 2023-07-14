mod event_queue;
mod geometry;
mod sweep_line;

use std::io::Write;
use std::time::Instant;
use std::{env, fs};

use geometry::Line;
use log::info;

use crate::event_queue::EventQueue;

fn read_file(file: &str) -> Vec<Line> {
    let contents = fs::read_to_string(file).expect("Should have been able to read the file");

    contents
        .lines()
        .map(|l| l.parse().expect("Failed to parse a line"))
        .collect()
}

fn main() {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    let params = env::args().collect::<Vec<_>>();

    for param in params.iter().skip(1) {
        info!("Processing file {}", param);
        let lines = read_file(param);

        let start_init = Instant::now();
        let queue = EventQueue::new(lines);
        let init = start_init.elapsed();
        let start_sweep = Instant::now();
        let intersections = queue.sweep();
        let swept = start_sweep.elapsed();
        let total = start_init.elapsed();

        info!("Initializing events: {:.2?}", init);
        info!("Sweeping line: {:.2?}", swept);
        info!("Total elapsed: {:.2?}", total);
        info!("intersections: {}", intersections.len());

        // create a new file "i_<filename>" with the intersections
        let filename = format!("{}.i", param);
        // delete file if it exists
        if fs::metadata(&filename).is_ok() {
            fs::remove_file(&filename).expect("Failed to delete file");
        }
        let mut file = fs::File::create(&filename).expect("Failed to create file");
        intersections
            .iter()
            .map(|p| format!("{}", p))
            .for_each(|p| writeln!(file, "{}", p).expect("Failed to write to file"));
        info!("Wrote intersections to file {}", filename);
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::geometry::Point;

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
        let mut queue = EventQueue::new(vec![line]);

        let first = queue.pop_first().unwrap();
        assert_eq!(first.point(), &q);

        let second = queue.pop_first().unwrap();
        assert_eq!(second.point(), &p);

        assert_eq!(None, queue.pop_first());
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
        let queue = EventQueue::new(vec![line, line2]);
        let intersections = queue.sweep().into_iter().collect::<Vec<_>>();

        assert_eq!(intersections.len(), 1);
        assert_eq!(intersections[0].x, 2.5);
        assert_eq!(intersections[0].y, 1.0);
    }

    #[test]
    fn test_three_lines() {
        let l1 = Line::from_str("0 1 5 1").unwrap();
        let l2 = Line::from_str("1.5 2.5 4 0.5").unwrap();
        let l3 = Line::from_str("0.5 1.5 4 2.5").unwrap();

        let queue = EventQueue::new(vec![l1, l2, l3]);
        let intersections = queue.sweep().into_iter().collect::<Vec<_>>();

        assert_eq!(intersections.len(), 2);
        assert_eq!(intersections[0].x, 2.157894737);
        assert_eq!(intersections[0].y, 1.973684211);

        assert_eq!(intersections[1].x, 3.375);
        assert_eq!(intersections[1].y, 1.0);
    }

    #[test]
    fn test_three_lines_different_order() {
        let l1 = Line::from_str("0 1 5 1").unwrap();
        let l2 = Line::from_str("1 2.5 4 0.5").unwrap();
        let l3 = Line::from_str("1.5 1.5 4 2.5").unwrap();

        let queue = EventQueue::new(vec![l1, l2, l3]);
        let intersections = queue.sweep().into_iter().collect::<Vec<_>>();

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

        let queue = EventQueue::new(vec![l1, l2, l3]);
        let intersections = queue.sweep().into_iter().collect::<Vec<_>>();

        assert_eq!(intersections.len(), 2);
        assert_eq!(intersections[0].x, 1.166666667);
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

        let queue = EventQueue::new(vec![l1, l2, l3, l4]);
        let intersections = queue.sweep().into_iter().collect::<Vec<_>>();

        assert_eq!(intersections.len(), 5);

        assert_eq!(intersections[0].x, 1.5); // floating point shenanigans
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
        let l1 = Line::from_str("0 0.5 3 0.5").unwrap();
        let l2 = Line::from_str("0.5 1 2 0.2").unwrap();
        let l3 = Line::from_str("1 0.8 1.8 0.8").unwrap();

        let queue = EventQueue::new(vec![l1, l2, l3]);
        let intersections = queue.sweep().into_iter().collect::<Vec<_>>();

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

        let queue = EventQueue::new(vec![l1, l2, l3, l4]);
        let intersections = queue.sweep().into_iter().collect::<Vec<_>>();

        assert_eq!(intersections.len(), 3);

        assert_eq!(intersections[0].x, 1.142857143);
        assert_eq!(intersections[0].y, 0.657142857);

        assert_eq!(intersections[1].x, 1.25);
        assert_eq!(intersections[1].y, 0.8);

        assert_eq!(intersections[2].x, 1.4375);
        assert_eq!(intersections[2].y, 0.5);
    }
}

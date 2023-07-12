use crate::geometry::{Line, Point};

#[derive(Debug, Clone)]
pub struct SweepLineElement {
    pub y: f64,
    pub line: Line,
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

pub struct SweepLine {
    pub elements: Vec<SweepLineElement>,
}

pub struct Neighbors {
    pub below: Option<SweepLineElement>,
    pub above: Option<SweepLineElement>,
}

pub struct SwapResult {
    pub below: Option<SweepLineElement>,
    pub smaller: SweepLineElement,
    pub bigger: SweepLineElement,
    pub above: Option<SweepLineElement>,
}

impl SweepLine {
    pub fn new() -> Self {
        Self {
            elements: Vec::new(),
        }
    }

    pub fn insert(&mut self, y: f64, line: Line) {
        let element = SweepLineElement { y, line };
        self.elements.push(element);
        self.elements.sort();
    }

    pub fn remove(&mut self, line: Line) {
        let index = self.elements.iter().position(|x| x.line == line);
        let Some(index) = index else {
            // The line is not in the sweep line
            return;
        };

        self.elements.remove(index);
    }

    pub fn get_neighbors(&self, line: Line) -> Option<Neighbors> {
        let index = self.elements.iter().position(|x| x.line == line);
        let Some(index) = index else {
            // The line is not in the sweep line
            return None;
        };

        let mut neighbors = Neighbors {
            below: None,
            above: None,
        };

        if let Some(line_below) = self.elements.get(index + 1) {
            neighbors.below = Some(line_below.clone());
        }

        if index > 0 {
            if let Some(line_above) = self.elements.get(index - 1) {
                neighbors.above = Some(line_above.clone());
            }
        }

        Some(neighbors)
    }

    pub fn swap_and_get_new_neighbors(
        &mut self,
        line1: Line,
        line2: Line,
        intersection_point: &Point,
    ) -> SwapResult {
        let index_line = self.elements.iter().position(|x| x.line == line1).unwrap();
        let index_other_line = self.elements.iter().position(|x| x.line == line2).unwrap();

        if index_line.abs_diff(index_other_line) != 1 {
            println!(
                "Two lines with indices too far apart: {}, {}. \nSegments are: {:?}",
                index_line, index_other_line, self.elements
            )
        }

        if index_line < index_other_line {
            self.elements[index_line].y = intersection_point.y;
            self.elements[index_other_line].y = intersection_point.y + 0.000000001;
        } else {
            self.elements[index_line].y = intersection_point.y + 0.000000001;
            self.elements[index_other_line].y = intersection_point.y;
        };
        self.elements.sort();

        let smaller = index_line.min(index_other_line);
        let bigger = index_line.max(index_other_line);

        let mut result = SwapResult {
            below: None,
            smaller: self.elements[smaller].clone(),
            bigger: self.elements[bigger].clone(),
            above: None,
        };

        // TODO: this ordering here is not correct
        if let Some(line_above) = self.elements.get(bigger + 1) {
            result.above = Some(line_above.clone());
        };

        if smaller > 0 {
            if let Some(line_below) = self.elements.get(smaller - 1) {
                result.below = Some(line_below.clone());
            };
        }

        result
    }
}

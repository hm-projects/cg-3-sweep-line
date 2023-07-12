use crate::geometry::Line;

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

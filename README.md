# Sweep Line (Bentley-Ottmann) Algorithm

- Nicolas Bissig
- Antonino Grasso

Source code is located in GitHub: <https://github.com/hm-projects/cg-3-sweep-line>

## Problem description

Count and find all intersections between a set of line segments using the sweep line algorithm.

## Quick overview

Implementation: [Rust](https://www.rust-lang.org/) \
For visualizations: [Python](https://www.python.org/) & [Matplotlib](https://matplotlib.org/)

### Program usage

Requirements:

- Rust > 1.69,
- Python > 3.10 with `requirements.txt` installed

```sh
$ cargo run --release -q .\data\s_1000_10.dat
[2023-07-14T12:11:26Z INFO  cg_3_sweep_line] Processing file .\data\s_1000_10.dat
[2023-07-14T12:11:26Z INFO  cg_3_sweep_line] Initializing events: 356.30µs
[2023-07-14T12:11:26Z INFO  cg_3_sweep_line] Sweeping line: 1.41ms
[2023-07-14T12:11:26Z INFO  cg_3_sweep_line] Total elapsed: 1.77ms
[2023-07-14T12:11:26Z INFO  cg_3_sweep_line] intersections: 796
[2023-07-14T12:11:26Z INFO  cg_3_sweep_line] Wrote intersections to file .\data\s_1000_10.dat.i
$ # visualize results using interative matplotlib figure
$ python .\visualize.py .\data\s_1000_10.dat
```

An interactive matplotlib figure will open, showing the line segments and their intersections.
Use the magnifying glass icon to zoom in and out for closer inspection of the intersections.

### Output

In the following the output for the data set `s_1000_10.dat` is shown.

![segments and intersections for sample dataset](doc/imgs/s_1000_10.dat.svg) \
Figure 1: Segments and intersections for sample dataset, image is in SVG format, so zoom in for better resolution

We want to highlight some sections of the output, which might be of interest.
Please see the axis labels in the figures to find the location in Figure 1.
Click to expand the sections, if viewed in a browser.

<details>
<summary markdown="span">Looks like a missed intersection, but misses really close</summary>
<img src="doc/imgs/close_miss.svg">
</details>

<details>
<summary markdown="span">Seems like a multi intersection point</summary>
<img src="doc/imgs/multi_intersect.svg">
</details>

## Algorithm & Implementation

### Requirements

The following requirements must be met with the input data set, and the implementation detects if this is not the case.

- No duplicate points
- No line segments with length 0
- No vertical line segments (same x coordinate for both points)
- No colinear / overlapping points, as intersection points must be unique

### Usage

```rust
let lines: Vec<Line> = read_file(file_path);
let queue: EventQueue = EventQueue::new(lines);
let intersections: BTreeSet<Point> = queue.sweep();
```

### Data structures

- `EventQueue` is a `BTreeSet` of `Events` with point and associated line segments
  - The events are ordered by their points
- `SweepLine` is `Vec` of `LineSegments` with `y` value
  - The vector is sorted by the `y` value of the line segments

### Sweeping Pseudo code

The following pseudo code is used to implement the sweep line algorithm.
Keep in mind, this is only pseudo code, and the actual implementation might differ.

```rust
fn sweep(mut self) -> BTreeSet<Point> {
    let mut sweep_line = SweepLine::new();

    while let Some(event) = self.pop_first() {
        // popping the next event ensures that the sweep line never goes backwards

        // update all line segments to their current y value at x
        sweep_line.update(event.point().x);

        match event {
                Event::Begin { point, line } => {
                    // Inserting also ensures the ordering of the sweep line
                    sweep_line.insert(point.y, line);

                    let neighbors = sweep_line.get_neighbors(&line);

                    if let Some(intersection_point) = line.intersection(neighbors.above) {
                        // adding the intersection event makes sure that the event is to the "right" of the sweep line, and was never seen before
                        self.add_intersection_event(intersection_point, line, line_above);
                    };
                    // the same is performed for neighbor below
                },
                Event::End { point: _, line } => {
                    let neighbors = sweep_line.get_neighbors(line);

                    if let Some(intersection_point) = line_below.intersection(line_above) {
                        self.add_intersection_event(intersection_point, line_below, line_above);
                    };

                    sweep_line.remove(line);
                },
                Event::Intersection {
                    point: intersection_point,
                    line,
                    other_line,
                } => {
                    // this swaps the two lines in the sweep line, and returns the new neighbors
                    let swapped = sweep_line.swap_and_get_new_neighbors(
                        line,
                        other_line,
                        intersection_point,
                    );

                    // test for intersections after swap
                    if let Some(intersection_point) = swapped.bigger.intersection(line_above) {
                        self.add_intersection_event(intersection_point, swapped.bigger, line_above);
                    };
                    // same is done for other pair
                },
        }
    }

    return self.intersection_points;
}
```

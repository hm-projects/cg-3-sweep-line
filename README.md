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

<details>
<summary markdown="span">Looks like a missed intersection, but misses really close</summary>
![](doc/imgs/close_miss.svg)
</details>

<details>
<summary markdown="span">Seems like a multi intersection point</summary>
![](doc/imgs/multi_intersect.svg)
</details>

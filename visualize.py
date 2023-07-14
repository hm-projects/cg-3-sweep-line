import matplotlib.pyplot as plt
import sys

LINE_WIDTH = 0.25
POINT_R = 0.5

# get the filename from first argument
filename = sys.argv[1]

fig, ax = plt.subplots()

# Read the file with lines coordinates
with open(filename, "r") as file:
    lines = file.readlines()

# Extract the line coordinates
line_coordinates = []
for line in lines:
    coords = line.strip().split()
    start = (float(coords[0]), float(coords[1]))
    end = (float(coords[2]), float(coords[3]))
    line_coordinates.append((start, end))

# Plot the lines
for start, end in line_coordinates:
    x = [start[0], end[0]]
    y = [start[1], end[1]]
    ax.plot(x, y, 'b', linewidth=LINE_WIDTH)

# Read the file with additional points
points_filename = f"{filename}.i"
with open(points_filename, "r") as file:
    points = file.readlines()

# Extract the point coordinates
point_coordinates = []
for point in points:
    coords = point.strip().split()
    x = float(coords[0])
    y = float(coords[1])
    point_coordinates.append((x, y))

# Plot the points
x_points = [coord[0] for coord in point_coordinates]
y_points = [coord[1] for coord in point_coordinates]
ax.scatter(x_points, y_points, color='r', marker='o', s=POINT_R)

# Set the plot title and labels
ax.set_title("Line segment intersections (Sweep-line algorithm))")
ax.set_xlabel("x")
ax.set_ylabel("y")

fig.set_dpi(300)
fig.tight_layout()
fig.savefig(f"{filename}.svg", dpi=2000)
plt.show()

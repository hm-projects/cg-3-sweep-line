import matplotlib.pyplot as plt

fig, ax = plt.subplots()

# Read the file with lines coordinates
lines_filename = "data/s_1000_10.dat"
with open(lines_filename, "r") as file:
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
    ax.plot(x, y, 'b', linewidth = 0.1)

# Read the file with additional points
points_filename = "data/s_1000_10.dat.i"
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
ax.scatter(x_points, y_points, color='r', marker='o', s=0.05)

# Set the plot title and labels
ax.set_title("Lines and Points Plot")
ax.set_xlabel("X-axis")
ax.set_ylabel("Y-axis")

fig.set_dpi(300)
plt.show()
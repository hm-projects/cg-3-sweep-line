# generate n line segments with random coordinates and given length
# and save them to a file

import math
import random
import sys

FROM = 0
TO = 10000
LENGTH = 10

def generate_line_segments(n: int):
    with open(f"./data/gen_{n}_{LENGTH}.txt", "w") as file:
        for _ in range(n):
            length = random.uniform(0.01, LENGTH)
            angle = random.uniform(0, 2 * 3.14159)  # Random angle in radians
            x1 = random.uniform(FROM, TO)  # Random x coordinate (adjust range as needed)
            y1 = random.uniform(FROM, TO)  # Random y coordinate (adjust range as needed)
            x2 = abs(x1 + length * math.cos(angle))
            y2 = abs(y1 + length * math.sin(angle))
            file.write(f"{x1} {y1} {x2} {y2}\n")


if __name__ == "__main__":
    n = int(sys.argv[1])
    generate_line_segments(n)

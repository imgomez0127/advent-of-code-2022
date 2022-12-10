import numpy as np
from numpy import linalg


def parse():
    output = []
    with open('input.txt') as f:
        return [line.strip().split() for line in f]


def cap_values(arr):
    return np.vectorize(lambda x: min(1, max(-1, x)))(arr)


def task(directions, knot_count=2):
    force_vector = {
        'U': np.array([0, 1]),
        'D': np.array([0, -1]),
        'L': np.array([-1, 0]),
        'R': np.array([1, 0]),
    }
    knots = [np.array([0, 0]) for _ in range(knot_count)]
    seen_positions = {(0, 0)}
    for direction, amount in directions:
        for _ in range(int(amount)):
            knots[0] += force_vector[direction]
            for head, tail in zip(knots[:-1], knots[1:]):
                tail += cap_values(head - tail) * (max(np.abs(head - tail)) > 1)
            seen_positions.add(tuple(knots[-1]))
    return len(seen_positions)


print(f'task 1: {task(parse())}')
print(f'task 2: {task(parse(), knot_count=10)}')

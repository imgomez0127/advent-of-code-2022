def parse():
    with open('input.txt') as f:
        return [
            [list(map(int, nums.split('-')))
             for nums in line.strip().split(',')]
            for line in f
        ]


def overlap(pairs):
    return sum([(a <= c and b >= d) or (c <= a and d >= b)
                for ((a, b), (c, d)) in pairs])


def overlap2(pairs):
    return sum([b >= c if c > a else d >= a for ((a, b), (c, d)) in pairs])


print(overlap(parse()))
print(overlap2(parse()))

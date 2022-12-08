

def parse():
    with open('input.txt') as f:
        return [list(line.strip()) for line in f]


def task1(result):
    intersections = []
    for x in result:
        a, b = x[:len(x)//2], x[len(x)//2:]
        intersections.extend(set(a) & set(b))
    s = 0
    for c in intersections:
        if c.islower():
            s += ord(c)-97+1
        else:
            s += ord(c)-65+27
    return s




def task2(result):
    badges = []
    for i in range(0, len(result), 3):
        a, b, c = result[i:i+3]
        badges.extend(set(a) & set(b) & set(c))
    s = 0
    for c in badges:
        if c.islower():
            s += ord(c)-97+1
        else:
            s += ord(c)-65+27
    return s


result = parse()
print(f'task 1 {task1(result)}')
print(f'task 2 {task2(result)}')



def parse():
    with open('input.txt') as f:
        return f.read()

def task1(packet):
    seen = set()
    for i, c in enumerate(packet):
        seen.add(c)
        print(seen)
        if len(seen) == 4:
            return i + 1
    return -1


print(task1(parse()))

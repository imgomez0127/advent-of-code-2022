from collections import deque
import dataclasses


def parse(num_stacks):
    stacks = [deque() for _ in range(num_stacks)]
    instructions_list = []
    with open('input.txt') as f:
        crates, instructions = f.read().split('\n\n')
        for crate_lines in crates.split('\n')[:-1]:
            crate_vals = list(crate_lines)
            for i, x in enumerate(crate_lines[1::4]):
                if x != ' ':
                    stacks[i].appendleft(x)
        for instruction in instructions.split('\n'):
            instruction_val = []
            for c in list(instruction.split()):
                if c.isnumeric():
                    instruction_val.append(int(c))
            instructions_list.append(instruction_val)
    return stacks, instructions_list[:-1]

def task1(crates, instructions):
    for (amt, start, end) in instructions:
        for _ in range(amt):
            crates[end-1].append(crates[start-1].pop())
    return ''.join([crate[-1] for crate in crates])


def task1(crates, instructions):
    for (amt, start, end) in instructions:
        x = deque()
        for _ in range(amt):
            x.appendleft(crates[start-1].pop())
        crates[end-1].extend(x)
    return ''.join([crate[-1] for crate in crates])


print(parse(9))
print(task1(*parse(9)))

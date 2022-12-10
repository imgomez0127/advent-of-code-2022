import enum


class Commands(enum.IntEnum):
    noop = 1
    addx = 2


def parse():
    command_map = {
        'addx': Commands.addx,
        'noop': Commands.noop,
    }
    commands = []
    with open('input.txt') as f:
        for line in f:
            command = line.split()
            if len(command) == 1:
                commands.append((command_map[command[0]], 0))
            else:
                commands.append((command_map[command[0]], int(command[1])))
    return list(reversed(commands))


def task1(commands):
    score = 0
    special_cycle = 20
    cycle = 0
    get_command = 0
    command = Commands.noop
    amt = 0
    x = 1
    while commands:
        if cycle == special_cycle:
            score += x * cycle
            special_cycle += 40
        if cycle != 0:
            print_char = '#' if x-1 <= (cycle % 40) - 1 <= x+1 else '.'
            if (cycle % 40) - 1 == 0:
                print('')
            print(print_char, end='')
        if get_command <= 0:
            x += amt
            command, amt = commands.pop()
            get_command = int(command)
        cycle += 1
        get_command -= 1
    print_char = '#' if x-1 <= (cycle % 40) - 1 <= x+1 else '.'
    print(print_char, end='')
    return score


print(f'\ntask 1: {task1(parse())}')

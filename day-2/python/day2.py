import dataclasses
import enum


class Choice(enum.IntEnum):
    ROCK = 0
    PAPER = 1
    SCISSOR = 2


@dataclasses.dataclass()
class Strategy:
    opponent: Choice
    yours: Choice


def parse():
    strategies = []
    with open('input.txt') as f:
        for strategy in f:
            choices = []
            for choice in strategy.split():
                match choice:
                    case 'A' | 'X':
                        choices.append(Choice.ROCK)
                    case 'B' | 'Y':
                        choices.append(Choice.PAPER)
                    case 'C' | 'Z':
                        choices.append(Choice.SCISSOR)
            strategies.append(Strategy(*choices))
    return strategies


def score1(strategy):
    if strategy.opponent == (strategy.yours - 1) % 3:
        return strategy.yours + 7
    if strategy.opponent == strategy.yours:
        return strategy.yours + 4
    return strategy.yours + 1


def score2(strategy):
    match strategy.yours:
        case Choice.ROCK:
            return ((strategy.opponent - 1) % 3) + 1
        case Choice.PAPER:
            return strategy.opponent + 4
        case Choice.SCISSOR:
            return ((strategy.opponent + 1) % 3) + 7


def task(strategies, scoring_method):
    return sum([scoring_method(strategy) for strategy in strategies])


strategies = parse()
print(task(strategies, score1))
print(task(strategies, score2))

def parse_input():
  with open('input.txt') as f:
    return [[int(meal.strip()) for meal in meals.split('\n') if meal]
            for meals in f.read().split('\n\n')]

def task(meals, k):
  return sum(sorted(map(sum, meals))[-k:])

print(f'task 1: {task(parse_input(), 1)}')
print(f'task 2: {task(parse_input(), 3)}')

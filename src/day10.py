import statistics

lines = open('day10_input.txt').read().splitlines()
corrupted_points_map = {
    ')': 3,
    ']': 57,
    '}': 1197,
    '>': 25137,
}

incomplete_points_map = {
    ')': 1,
    ']': 2,
    '}': 3,
    '>': 4,
}

open_to_close = {
    '(': ')',
    '[': ']',
    '{': '}',
    '<': '>',
}


corrupted_points = 0
completions = []
for line in lines:
    char_stack = []
    for char in line:
        if char in open_to_close:
            char_stack.append(char)
        elif char in open_to_close.values():
            if char != open_to_close[char_stack.pop()]:
                corrupted_points += corrupted_points_map[char]
                break
        else:
            raise Exception('Mark!')
    else:
        completions.append(list(reversed([open_to_close[char] for char in char_stack])))

print('part1: {}'.format(corrupted_points))

incomplete_points_list = []
for completion in completions:
    incomplete_points = 0
    for char in completion:
        incomplete_points *= 5
        incomplete_points += incomplete_points_map[char]
    incomplete_points_list.append(incomplete_points)

print('part2: {}'.format(statistics.median(sorted(incomplete_points_list))))

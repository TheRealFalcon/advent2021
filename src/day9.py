from functools import reduce

heights = [[int(x) for x in line.strip()] for line in open("day9_input.txt").readlines()]


def get_basin(i, j):
    left, right, up, down = [], [], [], []
    if i > 0 and heights[i][j] < heights[i-1][j]:
        left = get_basin(i-1, j)
    if i < len(heights) - 1 and heights[i][j] < heights[i+1][j]:
        right = get_basin(i+1, j)
    if j > 0 and heights[i][j] < heights[i][j-1]:
        up = get_basin(i, j-1)
    if j < len(heights[0]) - 1 and heights[i][j] < heights[i][j+1]:
        down = get_basin(i, j+1)
    good = [(i, j)] if heights[i][j] < 9 else []
    return good + left + right + up + down


basins = []
for (i, row) in enumerate(heights):
    for (j, height) in enumerate(row):
        above = True if i == 0 else height < heights[i-1][j]
        below = True if i == len(heights) - 1 else height < heights[i+1][j]
        left = True if j == 0 else height < heights[i][j-1]
        right = True if j == len(row) - 1 else height < heights[i][j+1]
        if above and below and left and right:
            print('\033[1m{}\033[0m'.format(height), end='')
            basin = get_basin(i, j)
            basin = list(set(basin))
            basins.append(basin)
        else:
            print(height, end='')
# print(risk_sum)

basins.sort(key=len)
print(reduce(lambda x, y: x*len(y), basins[-3:], 1))

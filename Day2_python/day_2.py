raw = open("input.txt").read().split("\n")
data = []
for i in raw:
    data.append(i.split())

def is_safe(row):
    
    row_diff = [int(row[i + 1]) - int(row[i]) for i in range(len(row) - 1)]

    min_num, max_num = min(row_diff), max(row_diff)

    return -3 <= min_num <= max_num < 0 or 0 < min_num <= max_num <= 3

print(sum([is_safe(row) for row in data]))

print(sum([any([is_safe(row[:i] + row[i + 1:]) for i in range(len(row))]) for row in data]))
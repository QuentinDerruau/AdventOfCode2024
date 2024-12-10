with open('input.txt', 'r') as file:
    left_list, right_list = zip(*[(int(line.split()[0]), int(line.split()[1])) for line in file.readlines()])
    print(sum(abs(sorted(right_list)[idx] - left_element) for idx, left_element in enumerate(sorted(left_list))), sum(left_element * right_list.count(left_element) for left_element in set(left_list)))
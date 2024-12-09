file_content = File.read('input.txt').strip

array_content = file_content.chars.map(&:to_i)
formated_array = []
increase_value = 0

for i in 0..array_content.length-1
    if i % 2 == 0
        for j in 0..array_content[i] - 1
            formated_array.push(increase_value)
        end
        
        if i + 1 < array_content.length && array_content[i + 1] > 0
            for j in 0..array_content[i + 1] - 1
                formated_array.push('.')
            end
        end
        increase_value += 1
    end
end

last_number_index = formated_array.rindex { |x| x.is_a?(Integer) }
first_dot_index = formated_array.index { |x| x == '.' }
formated_array_part1 = formated_array.clone

while last_number_index > first_dot_index
    formated_array_part1[first_dot_index] = formated_array_part1[last_number_index]
    formated_array_part1[last_number_index] = "."
    last_number_index = formated_array_part1.rindex { |x| x.is_a?(Integer) }
    first_dot_index = formated_array_part1.index { |x| x == '.' }
end

int_values = formated_array_part1.select { |x| x.is_a?(Integer) }

result_part_1 = 0
for i in 0..int_values.length-1
    result_part_1 += int_values[i] * i
end

puts "result part 1 : #{result_part_1}"

result_part_2 = 0
formated_array_part2 = formated_array.clone
last_number_index = formated_array_part2.rindex { |x| x.is_a?(Integer) }
last_number_value = formated_array_part2[last_number_index]

def find_first_consecutive_dots_indices(array, length,search)
    array.each_cons(length).with_index do |sequence, index|
      return (index...(index + length)).to_a if sequence.all? { |x| x == search}
    end
    nil
end

while last_number_value > 0
    sequences_of_dots = find_first_consecutive_dots_indices(formated_array_part2, formated_array_part2.count(last_number_value), ".")
    sequences_of_value = find_first_consecutive_dots_indices(formated_array_part2, formated_array_part2.count(last_number_value), last_number_value)
    if sequences_of_value != nil && sequences_of_dots != nil && sequences_of_value.length == formated_array_part2.count(last_number_value) && sequences_of_dots[0] < sequences_of_value[0]
        for i in 0..sequences_of_dots.length-1
            formated_array_part2[sequences_of_dots[i]] = last_number_value
            formated_array_part2[sequences_of_value[i]] = "."
        end
    end
    last_number_value -= 1
end


for i in 0..formated_array_part2.length-1
    if formated_array_part2[i].is_a?(Integer)
        result_part_2 += formated_array_part2[i] * i
    end
end

puts "result part 2 : #{result_part_2}"

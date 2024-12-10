require 'set'

input = File.read("input.txt").split("\n").map { |line| line.split("").map(&:to_i) }

@grid = Hash.new
width = input.size
height = input[0].size
heads = []
tops = Set.new

width.times do |y|
  height.times do |x|
    pos = Complex(x, y)
    level = input[y][x]
    heads << pos if level == 0
    tops << pos if level == 9
    @grid[pos] = level
  end
end

part1_result = heads.sum do |start_pos|
  search_stack = [start_pos]
  visited_positions = Set.new

  until search_stack.empty?
    current_pos = search_stack.pop
    visited_positions << current_pos

    [1 + 0i, -1 + 0i, 0 + 1i, 0 - 1i].each do |direction|
      next_pos = current_pos + direction
      if @grid[next_pos] == @grid[current_pos] + 1
        search_stack << next_pos
      end
    end
  end

  tops.intersection(visited_positions).size
end

part2_result = heads.sum do |start_pos|
  paths = Set.new
  search_stack = [[start_pos]]

  until search_stack.empty?
    current_path = search_stack.pop
    if current_path.size == 10
      paths << current_path
      next
    end
    current_pos = current_path.last

    [1 + 0i, -1 + 0i, 0 + 1i, 0 - 1i].each do |direction|
      next_pos = current_pos + direction
      if @grid[next_pos] == @grid[current_pos] + 1
        new_path = current_path.dup << next_pos
        search_stack << new_path
      end
    end
  end

  paths.size
end

puts "Part 1: #{part1_result}"
puts "Part 2: #{part2_result}"
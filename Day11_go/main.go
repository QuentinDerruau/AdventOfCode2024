package main

import (
	"fmt"
	"io/ioutil"
	"strconv"
	"strings"
)

func main() {
	data, err := ioutil.ReadFile("input.txt")
	if err != nil {
		panic(err)
	}

	stones := parseInput(string(data))

	// Part 1: 25 blinks
	part1Stones := stones
	for i := 0; i < 25; i++ {
		part1Stones = blink(part1Stones)
	}
	fmt.Println("Part 1:", len(part1Stones))

	// Part 2: 75 blinks
	part2Result := blinkXTimes(75)
	fmt.Println("Part 2:", part2Result)
}

func parseInput(input string) []int {
	parts := strings.Fields(input)
	stones := make([]int, len(parts))
	for i, part := range parts {
		stones[i], _ = strconv.Atoi(part)
	}
	return stones
}

func blink(stones []int) []int {
	var newStones []int
	for _, stone := range stones {
		if stone == 0 {
			newStones = append(newStones, 1)
		} else if len(strconv.Itoa(stone))%2 == 0 {
			strStone := strconv.Itoa(stone)
			mid := len(strStone) / 2
			left, _ := strconv.Atoi(strStone[:mid])
			right, _ := strconv.Atoi(strStone[mid:])
			newStones = append(newStones, left, right)
		} else {
			newStones = append(newStones, stone*2024)
		}
	}
	return newStones
}

func blinkXTimes(iterations int) int {
	lines := readLinesToList()
	if len(lines) == 0 || len(lines[0]) == 0 {
		return 0
	}

	stones := make(map[int]int)
	for _, stone := range lines[0] {
		stones[stone]++
	}

	for i := 0; i < iterations; i++ {
		newStones := make(map[int]int)
		for rock, count := range stones {
			blinkResults := blink([]int{rock})
			for _, blinkResult := range blinkResults {
				newStones[blinkResult] += count
			}
		}
		stones = newStones
	}

	total := 0
	for _, count := range stones {
		total += count
	}

	return total
}

func readLinesToList() [][]int {
	data, err := ioutil.ReadFile("input.txt")
	if err != nil {
		panic(err)
	}

	lines := strings.Split(string(data), "\n")
	var result [][]int
	for _, line := range lines {
		if line == "" {
			continue
		}
		parts := strings.Fields(line)
		var intParts []int
		for _, part := range parts {
			num, _ := strconv.Atoi(part)
			intParts = append(intParts, num)
		}
		result = append(result, intParts)
	}
	return result
}
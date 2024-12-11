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
	blinks := 75 // Number of blinks to simulate

	for i := 0; i < blinks; i++ {
		stones = blink(stones)
	}

	fmt.Println(len(stones))
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
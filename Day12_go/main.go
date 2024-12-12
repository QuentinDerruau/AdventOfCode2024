package main

import (
	"fmt"
	"bufio"
	"os"
)

func main() {
	grid := readInput("input.txt")
	regions := findRegions(grid)

	totalPricePartOne := calculateTotalPricePartOne(regions)
	fmt.Println("Total price of fencing all regions (Part One):", totalPricePartOne)
}

func readInput(filename string) [][]rune {
	file, err := os.Open(filename)
	if err != nil {
		panic(err)
	}
	defer file.Close()

	var grid [][]rune
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		grid = append(grid, []rune(scanner.Text()))
	}
	return grid
}

func findRegions(grid [][]rune) map[rune][][][2]int {
	visited := make(map[[2]int]bool)
	regions := make(map[rune][][][2]int)

	for i := range grid {
		for j := range grid[i] {
			if !visited[[2]int{i, j}] {
				region := exploreRegion(grid, i, j, visited)
				if len(region) > 0 {
					regions[grid[i][j]] = append(regions[grid[i][j]], region)
				}
			}
		}
	}
	return regions
}

func exploreRegion(grid [][]rune, i, j int, visited map[[2]int]bool) [][2]int {
	region := [][2]int{}
	stack := [][2]int{{i, j}}
	plantType := grid[i][j]

	for len(stack) > 0 {
		x, y := stack[len(stack)-1][0], stack[len(stack)-1][1]
		stack = stack[:len(stack)-1]

		if x < 0 || y < 0 || x >= len(grid) || y >= len(grid[0]) || visited[[2]int{x, y}] || grid[x][y] != plantType {
			continue
		}

		visited[[2]int{x, y}] = true
		region = append(region, [2]int{x, y})

		stack = append(stack, [2]int{x + 1, y}, [2]int{x - 1, y}, [2]int{x, y + 1}, [2]int{x, y - 1})
	}
	return region
}

func calculateTotalPricePartOne(regions map[rune][][][2]int) int {
	totalPrice := 0
	for plantType, plantRegions := range regions {
		for _, region := range plantRegions {
			area := len(region)
			perimeter := calculatePerimeter(region)
			price := area * perimeter
			fmt.Printf("Part One - Region of %c plants with area %d and perimeter %d has price %d\n", plantType, area, perimeter, price)
			totalPrice += price
		}
	}
	return totalPrice
}

func calculatePerimeter(region [][2]int) int {
	perimeter := 0
	directions := [][2]int{{1, 0}, {-1, 0}, {0, 1}, {0, -1}}

	for _, plot := range region {
		for _, dir := range directions {
			neighbor := [2]int{plot[0] + dir[0], plot[1] + dir[1]}
			if !contains(region, neighbor) {
				perimeter++
			}
		}
	}
	return perimeter
}

func calculateSides(region [][2]int) int {
	sides := 0
	directions := [][2]int{{1, 0}, {-1, 0}, {0, 1}, {0, -1}}

	for _, plot := range region {
		for _, dir := range directions {
			neighbor := [2]int{plot[0] + dir[0], plot[1] + dir[1]}
			if !contains(region, neighbor) {
				sides++
			}
		}
	}
	return sides
}

func contains(region [][2]int, plot [2]int) bool {
	for _, p := range region {
		if p == plot {
			return true
		}
	}
	return false
}
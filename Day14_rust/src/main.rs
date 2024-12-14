use std::collections::{HashSet};
use std::fs;

type Position = (i32, i32);
type Velocity = (i32, i32);

fn simulate_robots(robots: &[(Position, Velocity)], t: i32) -> Vec<Position> {
    let max_x = 101;
    let max_y = 103;

    robots
        .iter()
        .map(|&((rx, ry), (vx, vy))| {
            (
                (rx + t * vx).rem_euclid(max_x),
                (ry + t * vy).rem_euclid(max_y),
            )
        })
        .collect()
}

fn count_in_formation(robots: &HashSet<Position>) -> usize {
    let deltas = [(1, 0), (0, 1), (-1, 0), (0, -1)];

    let mut touching = 0;
    for &(rx, ry) in robots {
        for (dx, dy) in deltas.iter() {
            if robots.contains(&(rx + dx, ry + dy)) {
                touching += 1;
                break;
            }
        }
    }
    touching
}

fn find_easter_egg(robots: &[(Position, Velocity)], _width: i32, _height: i32) -> i32 {
    let mut second = 1;
    loop {
        let positions: HashSet<Position> = simulate_robots(robots, second)
            .into_iter()
            .collect();

        // Check if a compact formation has been found
        if count_in_formation(&positions) >= positions.len() / 2 {
            break;
        }

        second += 1;
    }
    second
}

fn part_1() {
    let input = fs::read_to_string("input.txt").expect("Failed to read input file");
    let robots = parse_input(&input);
    
    // Simulate positions after 100 seconds
    let positions = simulate_robots(&robots, 100);
    
    // Part 1: Count robots in different quadrants
    let quadrant_count = count_quadrant(&positions);
    println!("{}", quadrant_count);
}

fn parse_input(input: &str) -> Vec<(Position, Velocity)> {
    input
        .lines()
        .filter_map(|line| {
            if let Some((p_part, v_part)) = line.split_once(' ') {
                let p = p_part[2..] // Remove "p=" prefix
                    .split(',')
                    .map(|n| n.parse::<i32>().unwrap())
                    .collect::<Vec<_>>();
                let v = v_part[2..] // Remove "v=" prefix
                    .split(',')
                    .map(|n| n.parse::<i32>().unwrap())
                    .collect::<Vec<_>>();
                Some(((p[0], p[1]), (v[0], v[1])))
            } else {
                None
            }
        })
        .collect()
}

fn count_quadrant(robots: &Vec<Position>) -> i32 {
    let (mut c1, mut c2, mut c3, mut c4) = (0, 0, 0, 0);
    let max_x = 101;
    let max_y = 103;

    for &(rx, ry) in robots {
        if rx < max_x / 2 && ry < max_y / 2 {
            c1 += 1;
        } else if rx > max_x / 2 && ry < max_y / 2 {
            c2 += 1;
        } else if rx < max_x / 2 && ry > max_y / 2 {
            c3 += 1;
        } else if rx > max_x / 2 && ry > max_y / 2 {
            c4 += 1;
        }
    }
    c1 * c2 * c3 * c4
}

fn part_2() {
    let input = fs::read_to_string("input.txt").expect("Failed to read input file");
    let robots = parse_input(&input);
    
    // Find the time when compact formation is reached
    let seconds = find_easter_egg(&robots, 101, 103);
    println!("{}", seconds);
}

fn main() {
    part_1();
    part_2();
}

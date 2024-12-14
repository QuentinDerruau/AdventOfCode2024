use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const WIDTH: i32 = 101;
const HEIGHT: i32 = 103;
const SECONDS: i32 = 100;

fn main() {
    let input = read_input("input_test.txt");
    let mut robots = parse_input(&input);
    simulate(&mut robots, SECONDS);
    let safety_factor = calculate_safety_factor(&robots);
    println!("Safety factor: {}", safety_factor);
}

fn read_input(filename: &str) -> Vec<String> {
    let path = Path::new(filename);
    let file = File::open(&path).expect("Could not open file");
    io::BufReader::new(file).lines().map(|line| line.unwrap()).collect()
}

fn parse_input(input: &[String]) -> Vec<((i32, i32), (i32, i32))> {
    input.iter().map(|line| {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let p = parts[0][2..].split(',').collect::<Vec<&str>>();
        let v = parts[1][2..].split(',').collect::<Vec<&str>>();
        (
            (p[0].parse().unwrap(), p[1].parse().unwrap()),
            (v[0].parse().unwrap(), v[1].parse().unwrap())
        )
    }).collect()
}

fn simulate(robots: &mut Vec<((i32, i32), (i32, i32))>, seconds: i32) {
    for _ in 0..seconds {
        for robot in robots.iter_mut() {
            let (ref mut pos, ref vel) = *robot;
            pos.0 = (pos.0 + vel.0).rem_euclid(WIDTH);
            pos.1 = (pos.1 + vel.1).rem_euclid(HEIGHT);
        }
    }
}

fn calculate_safety_factor(robots: &[((i32, i32), (i32, i32))]) -> i32 {
    let mut quadrants = [0; 4];
    for &((x, y), _) in robots {
        if x == WIDTH / 2 || y == HEIGHT / 2 {
            continue;
        }
        let quadrant = match (x < WIDTH / 2, y < HEIGHT / 2) {
            (true, true) => 0,
            (false, true) => 1,
            (true, false) => 2,
            (false, false) => 3,
        };
        quadrants[quadrant] += 1;
    }
    quadrants.iter().product()
}

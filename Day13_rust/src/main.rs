use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct Machine {
    a_x: i32,
    a_y: i32,
    b_x: i32,
    b_y: i32,
    prize_x: i64,
    prize_y: i64,
}

fn main() -> io::Result<()> {
    let path = Path::new("input.txt");
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut machines = Vec::new();

    let mut lines = reader.lines();
    while let Some(Ok(line)) = lines.next() {
        if line.starts_with("Button A:") {
            let a_coords: Vec<i32> = line.split(&['X', 'Y', '+', ','][..])
                .filter_map(|s| s.trim().parse().ok())
                .collect();
            let b_line = lines.next().unwrap().unwrap();
            let b_coords: Vec<i32> = b_line.split(&['X', 'Y', '+', ','][..])
                .filter_map(|s| s.trim().parse().ok())
                .collect();
            let prize_line = lines.next().unwrap().unwrap();
            let prize_coords: Vec<i64> = prize_line.split(&['X', 'Y', '=', ','][..])
                .filter_map(|s| s.trim().parse().ok())
                .collect();

            machines.push(Machine {
                a_x: a_coords[0],
                a_y: a_coords[1],
                b_x: b_coords[0],
                b_y: b_coords[1],
                prize_x: prize_coords[0],
                prize_y: prize_coords[1],
            });
        }
    }

    // Part 1
    let (prizes_won, total_tokens) = calculate_prizes_and_tokens(&machines);
    println!("Part 1 - Prizes won: {}", prizes_won);
    println!("Part 1 - Total tokens spent: {}", total_tokens);

    // Part 2
    for machine in &mut machines {
        machine.prize_x += 10000000000000;
        machine.prize_y += 10000000000000;
    }
    let (prizes_won, total_tokens) = calculate_prizes_and_tokens(&machines);
    println!("Part 2 - Prizes won: {}", prizes_won);
    println!("Part 2 - Total tokens spent: {}", total_tokens);

    Ok(())
}

fn calculate_prizes_and_tokens(machines: &[Machine]) -> (i32, i32) {
    let mut total_tokens = 0;
    let mut prizes_won = 0;

    for machine in machines {
        if let Some(tokens) = min_tokens_to_win(machine) {
            total_tokens += tokens;
            prizes_won += 1;
        }
    }

    (prizes_won, total_tokens)
}

fn min_tokens_to_win(machine: &Machine) -> Option<i32> {
    let (gcd_x, x_x, y_x) = extended_gcd(machine.a_x as i64, machine.b_x as i64);
    let (gcd_y, x_y, y_y) = extended_gcd(machine.a_y as i64, machine.b_y as i64);

    if machine.prize_x % gcd_x != 0 || machine.prize_y % gcd_y != 0 {
        return None;
    }

    let scale_x = machine.prize_x / gcd_x;
    let scale_y = machine.prize_y / gcd_y;

    let a_presses_x = x_x * scale_x;
    let b_presses_x = y_x * scale_x;
    let a_presses_y = x_y * scale_y;
    let b_presses_y = y_y * scale_y;

    if a_presses_x >= 0 && b_presses_x >= 0 && a_presses_y >= 0 && b_presses_y >= 0 {
        let tokens_x = a_presses_x * 3 + b_presses_x;
        let tokens_y = a_presses_y * 3 + b_presses_y;
        return Some(tokens_x.min(tokens_y) as i32);
    }

    None
}

fn extended_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    if b == 0 {
        (a, 1, 0)
    } else {
        let (g, x, y) = extended_gcd(b, a % b);
        (g, y, x - (a / b) * y)
    }
}

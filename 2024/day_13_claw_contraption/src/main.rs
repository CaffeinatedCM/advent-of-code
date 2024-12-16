use rayon::prelude::*;
use regex::Regex;

const A_COST: i64 = 3;
const B_COST: i64 = 1;

fn main() {
    let input_file = std::env::args()
        .nth(1)
        .expect("Usage: <program> <input file>");

    let input_str = std::fs::read_to_string(input_file).unwrap();
    let machines = parse_input(&input_str);
    let total_cost = calculate_total_cost(&machines);
    println!("Total cost: {}", total_cost);
    let machines_part_two = machines
        .clone()
        .into_iter()
        .map(|mut machine| {
            machine.prize = (
                machine.prize.0 + 10000000000000,
                machine.prize.1 + 10000000000000,
            );
            machine
        })
        .collect::<Vec<_>>();
    let total_cost_part_two = calculate_total_cost(&machines_part_two);
    println!("Total cost part two: {}", total_cost_part_two);
}

#[derive(Debug, Clone)]
struct ClawMachine {
    a: (i64, i64),
    b: (i64, i64),
    prize: (i64, i64),
}

fn parse_button_line(line: &str) -> (i64, i64) {
    let button_regex = Regex::new(r"(X|Y)\+(\d+)").unwrap();
    let mut a_x: i64 = -1;
    let mut a_y: i64 = -1;

    for cap in button_regex.captures_iter(line) {
        match &cap[1] {
            "X" => a_x = cap[2].parse::<i64>().unwrap(),
            "Y" => a_y = cap[2].parse::<i64>().unwrap(),
            _ => {}
        }
    }

    (a_x, a_y)
}

fn parse_prize_line(line: &str) -> (i64, i64) {
    let prize_regex = Regex::new(r"(X|Y)=(\d+)").unwrap();
    let mut prize_x: i64 = -1;
    let mut prize_y: i64 = -1;

    for cap in prize_regex.captures_iter(line) {
        match &cap[1] {
            "X" => prize_x = cap[2].parse::<i64>().unwrap(),
            "Y" => prize_y = cap[2].parse::<i64>().unwrap(),
            _ => {}
        }
    }

    (prize_x, prize_y)
}

fn parse_input(input: &str) -> Vec<ClawMachine> {
    let mut machines = Vec::new();
    let lines = input.lines().collect::<Vec<_>>();

    for machine_lines in lines.chunks(4) {
        let a = parse_button_line(machine_lines[0]);
        let b = parse_button_line(machine_lines[1]);
        let prize = parse_prize_line(machine_lines[2]);

        machines.push(ClawMachine { a, b, prize });
    }

    machines
}

fn calculate_total_cost(machines: &Vec<ClawMachine>) -> i64 {
    machines
        .par_iter()
        .map(|machine| {
            let cost = calculate_cost(machine);
            cost
        })
        .sum::<i64>()
}

// Oh duh, I can just do math
fn calculate_cost(machine: &ClawMachine) -> i64 {
    let b = (machine.prize.1 * machine.a.0 - machine.prize.0 * machine.a.1)
        / (machine.b.1 * machine.a.0 - machine.b.0 * machine.a.1);
    let a = (machine.prize.0 - b * machine.b.0) / machine.a.0;
    if machine.a.0 * a + machine.b.0 * b != machine.prize.0
        || machine.a.1 * a + machine.b.1 * b != machine.prize.1
    {
        return 0;
    }

    a * A_COST + b * B_COST
}

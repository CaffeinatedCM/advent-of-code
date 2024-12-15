use rayon::prelude::*;
use regex::{Regex};

const A_COST: i64 = 3;
const B_COST: i64 = 1;

fn main() {
    let input_file = std::env::args()
        .nth(1)
        .expect("Usage: <program> <input file>");

    let input_str = std::fs::read_to_string(input_file).unwrap();
    let machines = parse_input(&input_str);
    println!("{:?}", machines);
    let total_cost = calculate_total_cost(&machines);
    println!("Total cost: {}", total_cost);
    let machines_part_two = machines.clone().into_iter().map(|mut machine| {
        machine.prize = (machine.prize.0 + 10000000000000, machine.prize.1 + 10000000000000);
        machine
    }).collect::<Vec<_>>();
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
    machines.par_iter().map(|machine| {
        let cost = calculate_cost(machine);
        println!("Cost: {}", cost);
        cost
    }).sum::<i64>()
}

// Return cost if the machine is able to grab the prize, otherwise return 0
fn calculate_cost(machine: &ClawMachine) -> i64 {
    // FIgure out maximum number of B presses
    let b_x_presses = machine.prize.0 / machine.b.0;
    let b_y_presses = machine.prize.1 / machine.b.1;
    let max_b_presses = b_x_presses.min(b_y_presses);

    // Figure out how many A presses are needed
    // If it can't with A presses, subtract B presses and try again
    // If it still can't, return 0
    let mut b_presses = max_b_presses;
    let mut a_presses = 0;
    println!("Max B presses: {}", max_b_presses);

    while b_presses >= 0 {
        let remaining_x = machine.prize.0 - b_presses * machine.b.0;
        let remaining_y = machine.prize.1 - b_presses * machine.b.1;

        let a_x_presses = remaining_x / machine.a.0;
        let a_y_presses = remaining_y / machine.a.1;
        let max_a_presses = a_x_presses.min(a_y_presses);
        
        if remaining_x - max_a_presses * machine.a.0 == 0 && remaining_y - max_a_presses * machine.a.1 == 0 {
            a_presses = max_a_presses;
            break;
        }

        b_presses -= 1;
    }
    println!("A presses: {}, B presses: {}", a_presses, b_presses);

    if a_presses < 0 || b_presses < 0 {
        println!("0");
        return 0;
    }

    println!("Cost: {}", a_presses * A_COST + b_presses * B_COST);
    a_presses * A_COST + b_presses * B_COST
}


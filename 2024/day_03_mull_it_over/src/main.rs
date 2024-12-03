use regex::{Match, Regex};

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    if args.len() < 2 {
        println!("Usage: {} <input file>", args[0]);
        std::process::exit(1);
    }

    let input = std::fs::read_to_string(&args[1]).unwrap();

    let result = solve(&input);

    println!("Result: {}", result);

    let result_with_conditions = solve_with_conditions(&input);
    println!("Result with conditions: {}", result_with_conditions);
}


fn solve(input: &str) -> i32 {
    let regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut result = 0;

    regex.captures_iter(input).for_each(|cap| {
        let a = cap[1].parse::<i32>().unwrap();
        let b = cap[2].parse::<i32>().unwrap();
        result += a * b;
    });

    result
}

fn solve_with_conditions(input: &str) -> i32 {
    let regex = Regex::new(r"(do\(\)|don\'t\(\)|mul\((\d+),(\d+)\))").unwrap();
    let mut result = 0;
    let mut enabled = true;

    regex.captures_iter(input).for_each(|cap| {
        match &cap[0] {
            "do()" => {
                enabled = true;
            }
            "don't()" => {
                enabled = false;
            }
            _ => {
                if enabled {
                    let a = cap[2].parse::<i32>().unwrap();
                    let b = cap[3].parse::<i32>().unwrap();
                    result += a * b;
                }
            }
        }
    });

    result
}

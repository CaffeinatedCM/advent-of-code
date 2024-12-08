fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    if args.len() < 2 {
        println!("Usage: {} <input file>", args[0]);
        std::process::exit(1);
    }

    let input = std::fs::read_to_string(&args[1]).unwrap();
    let equations = parse_input(&input);

    let result = calc_calibration(&equations);
    println!("Calibration Sum: {}", result);
}

#[derive(Debug)]
struct CalibrationEquation {
    lhs: i64,
    rhs: Vec<i64>,
}

fn parse_input(input: &str) -> Vec<CalibrationEquation> {
    let mut equations = Vec::new();

    for line in input.lines() {
        let mut parts = line.split(":");
        let lhs_str = parts.next().unwrap();
        let rhs_str = parts.next().unwrap();
        let lhs = lhs_str.parse::<i64>().unwrap();
        let rhs = rhs_str
            .split_whitespace()
            .map(|num| num.parse::<i64>().unwrap())
            .collect();

        equations.push(CalibrationEquation { lhs, rhs });
    }

    equations
}

fn do_the_thing(target: i64, operator: &str, current: i64, remaining: &[i64]) -> bool {
    if remaining.is_empty() {
        return current == target;
    }

    let next = remaining[0];
    let rest = &remaining[1..];
    let mut temp = current;

    match operator {
        "+" => temp += next,
        "*" => temp *= next,
        "||" => temp = (temp.to_string() + &next.to_string())
            .parse::<i64>()
            .unwrap(),
        _ => panic!("Invalid operator"),
    }

    do_the_thing(target, "+", temp, rest)
        || do_the_thing(target, "*", temp, rest)
        || do_the_thing(target, "||", temp, rest)
}

fn validate_equation(equation: &CalibrationEquation) -> bool {
    let target = equation.lhs;
    let remaining = &equation.rhs[1..];
    do_the_thing(target, "+", equation.rhs[0], remaining)
        || do_the_thing(target, "*", equation.rhs[0], remaining)
        || do_the_thing(target, "||", equation.rhs[0], remaining)
}

fn calc_calibration(equations: &Vec<CalibrationEquation>) -> i64 {
    let mut result = 0;

    for equation in equations {
        if validate_equation(equation) {
            result += equation.lhs;
        }
    }

    result
}

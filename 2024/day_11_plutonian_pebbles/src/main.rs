fn main() {
    let input_file = std::env::args().nth(1).expect("Usage: <program> <input file>");
    let input_str = std::fs::read_to_string(input_file).unwrap();

    let input = parse_input(&input_str);
    let after25 = blink_times(&input, 25);
    println!("After 25 blinks, there are {} rocks", after25.len());
    //let after75 = blink_times(&after25, 50);
    //println!("After 75 blinks, there are {} rocks", after75.len());
}

fn parse_input(input: &str) -> Vec<i64> {
    input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn blink_times(rocks: &Vec<i64>, times: i32) -> Vec<i64> {
    let mut rocks = rocks.clone();

    for _ in 0..times {
        rocks = blink(&rocks);
    }

    rocks
}

fn blink(rocks: &Vec<i64>) -> Vec<i64> {
    let mut new_rocks = Vec::new();

    for &rock in rocks {
        if rock == 0 {
            new_rocks.push(1);
            continue;
        }

        let rock_str = rock.to_string();
        if rock_str.len() % 2 == 0 {
            let half = rock_str.len() / 2;
            let left = rock_str[..half].parse::<i64>().unwrap();
            let right = rock_str[half..].parse::<i64>().unwrap();
            new_rocks.push(left);
            new_rocks.push(right);
            continue;
        }

        new_rocks.push(rock * 2024);
    }

    new_rocks
}


use regex::Regex;

fn main() {
    let mut input_file = String::new();
    let mut width = 101;  // Default width
    let mut height = 103; // Default height
    let mut iterations = 100;

    let args: Vec<String> = std::env::args().collect();
    let mut iter = args.iter().skip(1); // Skip the program name

    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "--width" => {
                width = iter.next()
                    .expect("Expected a value after --width")
                    .parse::<i32>()
                    .expect("Width must be a number");
            }
            "--height" => {
                height = iter.next()
                    .expect("Expected a value after --height")
                    .parse::<i32>()
                    .expect("Height must be a number");
            }
            "--iterations" => {
                iterations = iter.next()
                    .expect("Expected a value after --iterations")
                    .parse::<i32>()
                    .expect("Iterations must be a number");
            }
            _ if input_file.is_empty() => {
                input_file = arg.clone();
            }
            _ => {
                eprintln!("Unknown argument: {}", arg);
                std::process::exit(1);
            }
        }
    }

    let input_str = std::fs::read_to_string(input_file)
        .expect("Failed to read input file");
    let (mut robots,mut map) = parse_input(&input_str, width, height);

    simulate(&mut robots, &mut map, iterations);
    let safety_factor = get_safety_factor(&map);
    println!("Safety factor: {}", safety_factor);
}

#[derive(Debug)]
struct Robot {
    pos: (i32, i32),
    velocity: (i32, i32),
}

fn parse_input(input: &str, width: i32, height: i32) -> (Vec<Robot>, Vec<Vec<i32>>) {
    let mut robots = Vec::new();
    let mut map = vec![vec![0; width as usize]; height as usize];

    let line_regex = Regex::new(r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)").unwrap();
    for line in input.lines() {
        let cap = line_regex.captures(line).unwrap();
        let pos = (cap[1].parse().unwrap(), cap[2].parse().unwrap());
        let velocity = (cap[3].parse().unwrap(), cap[4].parse().unwrap());
        map[pos.1 as usize][pos.0 as usize] += 1;
        robots.push(Robot { pos, velocity });
    }


    (robots, map)
}

fn simulate(robots: &mut Vec<Robot>, map: &mut Vec<Vec<i32>>, iterations: i32) {
    let width = map[0].len() as i32;
    let height = map.len() as i32;

    for i in 0..iterations {
        for robot in robots.iter_mut() {
            map[robot.pos.1 as usize][robot.pos.0 as usize] -= 1;
            robot.pos.0 = (robot.pos.0 + robot.velocity.0).rem_euclid(width);
            robot.pos.1 = (robot.pos.1 + robot.velocity.1).rem_euclid(height);
            map[robot.pos.1 as usize][robot.pos.0 as usize] += 1;
        }
        // if no robots overlap, print the map and iteration
        // kinda lame, but the prompt didn't really describe what the tree would look like
        // so this simple dumb way seems fine. It _is_ an assumption that there's no overlap but
        // the forums say that is the case
        let mut overlap = false;
        for row in map.iter() {
            for cell in row.iter() {
                if *cell > 1 {
                    overlap = true;
                    break;
                }
            }
            if overlap {
                break;
            }
        }
        if !overlap {
            println!("Possible Easter Egg at Iteration: {}", i);
            print_map(&map);
        }
    }
}

fn print_map(map: &Vec<Vec<i32>>) {
    for row in map {
        for cell in row {
            if *cell == 0 {
                print!(".");
            } else {
                print!("{}", cell);
            }
        }
        println!();
    }
}

fn get_safety_factor(map: &Vec<Vec<i32>>) -> i32 {
    let width = map[0].len() as i32;
    let height = map.len() as i32;
    let center_row = (height as f32/2.0).floor() as i32;
    let center_col = (width as f32/2.0).floor() as i32;

    let mut safety_factor = 1;
    let quadrants = vec![
        (0, 0, center_col, center_row),
        (center_col + 1, 0, width, center_row),
        (0, center_row + 1, center_col, height),
        (center_col + 1, center_row + 1, width, height),
    ];

    for (start_col, start_row, end_col, end_row) in quadrants {
        let mut robots = 0;
        for row in start_row..end_row {
            for col in start_col..end_col {
                robots += map[row as usize][col as usize];
            }
        }
        safety_factor *= robots;
    }

    safety_factor
}

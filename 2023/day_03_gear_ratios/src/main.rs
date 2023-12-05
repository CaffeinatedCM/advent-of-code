fn main() {
    let input = include_str!("./input.txt");
    let engine = parse_part_schematic(input);

    println!(
        "Sum of part numbers: {}",
        sum_part_numbers(&engine)
    );

    println!(
        "Sum of gear ratios: {}",
        sum_gear_ratios(&engine)
    );
}

const DIRECTIONS: [[i32; 2]; 8] = [[-1, -1], [-1, 0], [-1, 1], [0, -1], [0, 1], [1, -1], [1, 0], [1, 1]];

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
struct Part {
    line: usize,
    start_char: usize,
    end_char: usize,
    number: i32,
}

struct Gear {
    line: usize,
    char: usize,
    ratio: i32,
}

struct Engine {
    parts: Vec<Part>,
    gears: Vec<Gear>
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    let mut result = Vec::new();

    for line in input.lines() {
        let chars = line.chars();
        result.push(chars.collect());
    }

    result
}

// I'm pretty sure this is not great, but its what I got right now
fn parse_part_schematic(input_str: &str) -> Engine {
    let input = parse_input(input_str);

    let mut engine = Engine {
        parts: Vec::new(),
        gears: Vec::new()
    };

    for (i, line) in input.iter().enumerate() {
        let mut cur_seq = Vec::new();
        let mut cur_part = Part {
            line: i,
            start_char: 0,
            end_char: 0,
            number: 0
        };

        let mut adjacent_to_symbol = false;
        for (j, c) in line.iter().enumerate() {
            if c.is_digit(10) {
                if cur_seq.is_empty() {
                    cur_part.start_char = j;
                }

                cur_seq.push(*c);

                for direction in DIRECTIONS.iter() {
                    let x = i as i32 + direction[0];
                    let y = j as i32 + direction[1];
                    if x >= 0 && y >= 0 && x < input.len() as i32 && y < line.len() as i32 {
                        if input[x as usize][y as usize].is_digit(10)  {
                            continue;
                        }
                        if input[x as usize][y as usize] != '.' {
                            adjacent_to_symbol = true;
                            break;
                        }
                    }
                }

                if j == line.len() -1 && adjacent_to_symbol {
                    let num = cur_seq.iter().collect::<String>().parse::<i32>().unwrap();
                    cur_part.end_char = j;
                    cur_part.number = num;
                    engine.parts.push(cur_part);
                }

            } else {
                if !cur_seq.is_empty() && adjacent_to_symbol {
                    let num = cur_seq.iter().collect::<String>().parse::<i32>().unwrap();
                    cur_part.end_char = j - 1;
                    cur_part.number = num;
                    engine.parts.push(cur_part);
                }
                cur_seq.clear();
                adjacent_to_symbol = false;
            }

            if *c == '*' {
                engine.gears.push(Gear {
                    line: i,
                    char: j,
                    ratio: 0
                });
            }
        }
    }

    let mut gears_to_remove = Vec::new();
    for (i, gear) in engine.gears.iter_mut().enumerate() {
        let mut adjacent_parts = Vec::new();
        for part in engine.parts.iter() {
            if part.line == gear.line {
                if part.end_char == gear.char -1 || part.start_char == gear.char +1 {
                    adjacent_parts.push(part);
                }
            }
            if part.line == gear.line -1 || part.line == gear.line +1 {
                if part.start_char.checked_sub(1).or(Some(0)).unwrap() <= gear.char && part.end_char + 1 >= gear.char {
                    adjacent_parts.push(part);
                }
            }
        }

        if adjacent_parts.len() != 2 {
            gears_to_remove.push(i);
            continue;
        }

        gear.ratio = adjacent_parts.iter().map(|part| part.number).product();
    }

    for i in gears_to_remove.iter().rev() {
        engine.gears.remove(*i);
    }

    engine
}

#[test]
fn test_parse_part_engine() {
    let input = include_str!("./example1.txt");

    let result = parse_part_schematic(input);
    assert_eq!(result.parts.iter().map(|x| x.number).collect::<Vec<i32>>(), vec![467, 35, 633, 617, 592, 755, 664, 598]);
    assert_eq!(result.gears.iter().map(|x| x.ratio).collect::<Vec<i32>>(), vec![16345, 451490]);
}

#[test]
fn test_parse_part_engine_endline() {
    let input = include_str!("./example2.txt");

    let result = parse_part_schematic(input);
    assert_eq!(result.parts.iter().map(|x| x.number).collect::<Vec<i32>>(), vec![467, 35, 633, 617, 592, 755, 664, 598]);
    assert_eq!(result.gears.iter().map(|x| x.ratio).collect::<Vec<i32>>(), vec![16345, 451490]);
}

fn sum_part_numbers(engine: &Engine) -> i32 {
    let part_numbers = engine.parts.iter().map(|part| part.number);

    let mut result = 0;
    for num in part_numbers {
        result += num;
    }

    result
}

#[test]
fn test_sum_part_numbers() {
    let input = include_str!("./example1.txt");
    let engine = parse_part_schematic(input);
    assert_eq!(sum_part_numbers(&engine), 4361);
}

fn sum_gear_ratios(engine: &Engine) -> i32 {
    let mut result = 0;
    for gear in engine.gears.iter() {
        result += gear.ratio;
    }

    result
}

#[test]
fn test_sum_gear_ratios() {
    let input = include_str!("./example1.txt");
    let engine = parse_part_schematic(input);
    assert_eq!(sum_gear_ratios(&engine), 467835);
}
fn main() {
   let input = include_str!("./input.txt");

    println!(
        "Sum of part numbers: {}",
        sum_part_numbers(input)
    );
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    let mut result = Vec::new();

    for line in input.lines() {
        let chars = line.chars();
        result.push(chars.collect());
    }

    result
}

fn parse_part_numbers(input: &Vec<Vec<char>>) -> Vec<i32> {
    let directions = [[-1, -1], [-1, 0], [-1, 1], [0, -1], [0, 1], [1, -1], [1, 0], [1, 1]];
    let mut result = Vec::new();

    for (i, line) in input.iter().enumerate() {
        let mut cur_seq = Vec::new();
        let mut adjacent_to_symbol = false;
        for (j, c) in line.iter().enumerate() {
            if c.is_digit(10) {
                cur_seq.push(*c);

                for direction in directions.iter() {
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
                    result.push(num);
                }

            } else {
                if !cur_seq.is_empty() && adjacent_to_symbol {
                    let num = cur_seq.iter().collect::<String>().parse::<i32>().unwrap();
                    result.push(num);
                }
                cur_seq.clear();
                adjacent_to_symbol = false;
            }
        }
    }

    result
}

#[test]
fn test_parse_part_numbers() {
    let input = include_str!("./example1.txt");

    let parsed_input = parse_input(input);
    let result = parse_part_numbers(&parsed_input);
    assert_eq!(result, vec![467, 35, 633, 617, 592, 755, 664, 598])
}

#[test]
fn test_parse_part_numbers_endline() {
    let input = include_str!("./example2.txt");

    let parsed_input = parse_input(input);
    let result = parse_part_numbers(&parsed_input);
    assert_eq!(result, vec![467, 35, 633, 617, 592, 755, 664, 598])
}

fn sum_part_numbers(input: &str) -> i32 {
    let parsed_input = parse_input(input);
    let part_numbers = parse_part_numbers(&parsed_input);

    let mut result = 0;
    for num in part_numbers.iter() {
        result += num;
    }

    result
}

#[test]
fn test_sum_part_numbers() {
    let input = include_str!("./example1.txt");
    assert_eq!(sum_part_numbers(input), 4361);
}
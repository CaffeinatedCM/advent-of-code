use std::collections::HashMap;

fn main() {
    let input = include_str!("./input.txt");
    let sequence = parse_input(input);

    println!("Hash: {}", hash_initialization_sequence(&sequence));

    let boxes = run_initialization_sequence(&sequence);
    println!("Focusing power: {}", calc_focusing_power(&boxes));
}

fn hash(input: &str) -> i32 {
    let mut current_value = 0;

    /*
        Determine the ASCII code for the current character of the string.
        Increase the current value by the ASCII code you just determined.
        Set the current value to itself multiplied by 17.
        Set the current value to the remainder of dividing itself by 256.
     */
    for c in input.chars() {
        let ascii_code = c as i32;
        current_value += ascii_code;
        current_value *= 17;
        current_value %= 256;
    }

    current_value
}

#[test]
fn test_hash() {
    assert_eq!(hash(&"HASH"), 52);
    assert_eq!(hash(&"rn=1"), 30);
}

fn hash_initialization_sequence(sequence: &Vec<String>) -> i32 {
    sequence.iter().map(|s| hash(s)).sum()
}

fn parse_input(input: &str) -> Vec<String> {
    input.split(',').map(|s| s.to_string()).collect()
}

#[test]
fn test_hash_initialization_sequence() {
    let input = include_str!("./example1.txt");
    let sequence = parse_input(input);

    assert_eq!(hash_initialization_sequence(&sequence), 1320);
}

#[derive(Debug, PartialEq)]
enum Instruction<'a> {
    Add(&'a str, i32),
    Remove(&'a str),
}

fn parse_instruction(instruction: &str) -> Instruction {
    let instruction_idx = instruction.chars().position(|c| c == '=' || c == '-').unwrap();
    let (left, right) = instruction.split_at(instruction_idx);
    let (op, focal_length) = right.split_at(1);

    match op {
        "=" => Instruction::Add(left, focal_length.parse::<i32>().unwrap()),
        "-" => Instruction::Remove(left),
        _ => panic!("Invalid operator"),
    }
}

#[test]
fn test_parse_instruction() {
    assert_eq!(parse_instruction(&"rn=1"), Instruction::Add("rn", 1));
    assert_eq!(parse_instruction(&"rn-1"), Instruction::Remove("rn"));
}

fn run_initialization_sequence(sequence: &Vec<String>) -> HashMap<i32, Vec<(&str, i32)>> {
    let mut result = HashMap::new();

    for s in sequence {
        match parse_instruction(s) {
            Instruction::Add(left, focal_length) => {
                let box_id = hash(left);
                let v = result.entry(box_id).or_insert(Vec::new());
                match v.iter().position(|(l, _)| *l == left) {
                    Some(idx) => v[idx] = (left, focal_length),
                    None => v.push((left, focal_length)),
                }
            },
            Instruction::Remove(left) => {
                let box_id = hash(left);
                result.entry(box_id).or_insert(Vec::new()).retain(|(l, _) | *l != left)
            }
        }
    }

    result.retain(|_, v| !v.is_empty());
    result
}

#[test]
fn test_run_initialization_sequence() {
    let input = include_str!("./example1.txt");
    let sequence = parse_input(input);

    assert_eq!(run_initialization_sequence(&sequence), HashMap::from([
     (0, vec![("rn", 1), ("cm", 2)]),
     (3, vec![("ot", 7), ("ab", 5), ("pc", 6)])
    ]))
}

fn calc_focusing_power(boxes: &HashMap<i32, Vec<(&str, i32)>>) -> i32 {
    let mut focusing_power = 0;

    /*
    The focusing power of a single lens is the result of multiplying together:
    One plus the box number of the lens in question.
    The slot number of the lens within the box: 1 for the first lens, 2 for the second lens, and so on.
    The focal length of the lens.
     */
    for (box_id, lenses) in boxes {
        for (idx, (_, focal_length)) in lenses.iter().enumerate() {
            focusing_power += (box_id + 1) * (idx as i32 + 1) * focal_length;
        }
    }

    focusing_power
}

#[test]
fn test_calc_focusing_power() {
    let input = include_str!("./example1.txt");
    let sequence = parse_input(input);
    let boxes = run_initialization_sequence(&sequence);

    assert_eq!(calc_focusing_power(&boxes), 145);
}
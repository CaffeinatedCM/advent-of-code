use std::collections::{HashMap};

fn main() {
    let input = include_str!("./input.txt");
    let map = parse_input(input);

    println!("Steps: {}", traverse(&map));
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Node {
    id: String,
    left: Option<String>,
    right: Option<String>,
}

#[derive(Debug, PartialEq, Eq)]
struct Map {
    instructions: Vec<char>,
    network: HashMap<String, Node>,
}

fn parse_input(input: &str) -> Map {
    let mut lines = input.lines().into_iter();

    let instructions = lines.next().unwrap().chars().collect::<Vec<char>>();
    lines.next();

    let mut network = HashMap::new();
    for line in lines {
        let mut parts = line.split(" ");
        let id = parts.next().unwrap().to_string();
        parts.next(); // Skip the =
        let left = parts.next().unwrap().to_string().replace("(", "").replace(",", "");
        let right = parts.next().unwrap().to_string().replace(")", "");

        network.insert(id.clone(), Node {
            left: if left == id { None } else { Some(left) },
            right: if right == id { None } else { Some(right) },
            id,
        });
    }

    Map {
        instructions,
        network,
    }
}

#[test]
fn test_parse_input() {
    let input = include_str!("./example1.txt");
    let map = parse_input(input);

    assert_eq!(map.instructions,  vec!['R','L']);
    assert_eq!(map.network.len(), 7);
    assert_eq!(map.network.get("AAA").unwrap(), &Node {
        id: "AAA".to_string(),
        left: Some("BBB".to_string()),
        right: Some("CCC".to_string()),
    });
}

fn traverse(map: &Map) -> i32 {
    let mut current = map.network.get("AAA").unwrap();
    let mut steps = 0_i32;

    loop {
        let instruction = map.instructions[steps as usize % map.instructions.len()];
        println!("{}: {:?} -> {:?}", steps, instruction, current);

        match instruction {
            'R' => {
                if let Some(right) = &current.right {
                    current = map.network.get(right).unwrap();
                } else {
                   panic!("No right node found for {:?}", current)
                }
            },
            'L' => {
                if let Some(left) = &current.left {
                    current = map.network.get(left).unwrap();
                } else {
                    panic!("No left node found for {:?}", current)
                }
            },
            _ => panic!("Unknown instruction: {}", instruction),
        }

        steps += 1;

        if current.id == "ZZZ" {
            break;
        }
    }

    steps
}

#[test]
fn test_traverse() {
    let input = include_str!("./example1.txt");
    let map = parse_input(input);

    assert_eq!(traverse(&map), 2);
}

#[test]
fn test_traverse_looping() {
    let input = include_str!("./example2.txt");
    let map = parse_input(input);

    assert_eq!(traverse(&map), 6);
}
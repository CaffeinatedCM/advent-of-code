use std::collections::{HashMap};

fn main() {
    let input = include_str!("./input.txt");
    let map = parse_input(input);

    println!("Steps: {}", traverse(&map));
    println!("Ghost Steps: {}", ghost_traverse(&map));
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

// Oh no, it's slow as fuck
fn ghost_traverse(map: &Map) -> i64 {
    let mut current_nodes = map.network.iter().filter(|n| n.0.ends_with("A")).map(|x| x.1).collect::<Vec<&Node>>();
    let mut steps = 0_i64;

    println!("Starting Nodes: {:?}", current_nodes);
    loop {
        let mut next_nodes = Vec::new();
        let instruction = map.instructions[steps as usize % map.instructions.len()];

        for current_node in &current_nodes {
            match instruction {
                'R' => {
                    if let Some(right) = &current_node.right {
                        next_nodes.push(map.network.get(right).unwrap());
                    } else {
                       panic!("No right node found for {:?}", current_node)
                    }
                },
                'L' => {
                    if let Some(left) = &current_node.left {
                        next_nodes.push(map.network.get(left).unwrap());
                    } else {
                        panic!("No left node found for {:?}", current_node)
                    }
                },
                _ => panic!("Unknown instruction: {}", instruction),
            }
        }

        steps += 1;

        if next_nodes.iter().all(|n| n.id.ends_with("Z")) {
            break;
        }

        if (steps % 100000) == 0 {
            println!("Steps: {}, Nodes: {:?}", steps, next_nodes);
        }

        current_nodes.clear();
        current_nodes.extend(next_nodes);
    }

    steps
}

#[test]
fn test_ghost_traverse() {
    let input = include_str!("./example3.txt");
    let map = parse_input(input);

    assert_eq!(ghost_traverse(&map), 6);
}

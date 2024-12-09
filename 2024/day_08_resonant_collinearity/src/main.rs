use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    if args.len() < 2 {
        println!("Usage: {} <input file>", args[0]);
        std::process::exit(1);
    }

    let input_str = std::fs::read_to_string(&args[1]).unwrap();
    let input = parse_input(&input_str);

    let antinodes = find_antinodes(&input);
    println!("Antinode Count: {}", count_unique_antinodes(&antinodes));

    let antinodes_with_resonance = find_antinodes_with_resonance(&input);
    println!("Antinode Count with Resonance: {}", count_unique_antinodes(&antinodes_with_resonance));
}

#[derive(Debug)]
struct Input {
    map: Vec<Vec<char>>,
    map_width: usize,
    map_height: usize,
    antena_locations: HashMap<char, Vec<(usize, usize)>>,
}

fn parse_input(input: &str) -> Input {
    let mut map = Vec::new();
    let mut antena_locations = HashMap::new();

    for (y, line) in input.lines().enumerate() {
        let mut row = Vec::new();

        for (x, c) in line.chars().enumerate() {
            if c != '.' {
                let entry = antena_locations.entry(c).or_insert(Vec::new());
                entry.push((x, y));
            }

            row.push(c);
        }

        map.push(row);
    }

    let map_height = map.len();
    let map_width = map[0].len();
    Input { map, antena_locations, map_width, map_height }
}

fn count_unique_antinodes(antinodes: &HashMap<char, HashSet<(usize, usize)>>) -> usize {
    let mut unique_antinodes = HashSet::new();

    for set in antinodes.values() {
        for antinode in set {
            unique_antinodes.insert(*antinode);
        }
    }

    unique_antinodes.len()
}

fn find_antinodes(input: &Input) -> HashMap<char, HashSet<(usize, usize)>> {
    let mut antinodes = HashMap::new();

    for (antena, locations) in &input.antena_locations {
        let mut antinode_set = HashSet::new();
        if locations.len() == 1 {
            antinodes.insert(*antena, antinode_set);
            continue;
        }

        for i in 0..locations.len() {
            for j in i+1..locations.len() {
                let (x1, y1) = locations[i];
                let (x2, y2) = locations[j];

                let dx = x2 as i32 - x1 as i32;
                let dy = y2 as i32 - y1 as i32;

                let p1 = (x1 as i32 - dx, y1 as i32 - dy);
                let p2 = (x2 as i32 + dx, y2 as i32 + dy);

                if p1.0 >= 0 && p1.0 < input.map_width as i32 && p1.1 >= 0 && p1.1 < input.map_height as i32 {
                    antinode_set.insert((p1.0 as usize, p1.1 as usize));
                }
                if p2.0 >= 0 && p2.0 < input.map_width as i32 && p2.1 >= 0 && p2.1 < input.map_height as i32 {
                    antinode_set.insert((p2.0 as usize, p2.1 as usize));
                }
            }
        }

        antinodes.insert(*antena, antinode_set);
    }

   antinodes 
}

fn find_antinodes_with_resonance(input: &Input) -> HashMap<char, HashSet<(usize, usize)>> {
    let mut antinodes = HashMap::new();

    for (antena, locations) in &input.antena_locations {
        let mut antinode_set = HashSet::new();
        if locations.len() == 1 {
            antinodes.insert(*antena, antinode_set);
            continue;
        }

        for i in 0..locations.len() {
            for j in i+1..locations.len() {
                let (x1, y1) = locations[i];
                let (x2, y2) = locations[j];

                // Calculate the vector between the two antena
                let dx = x2 as i32 - x1 as i32;
                let dy = y2 as i32 - y1 as i32;

                // Find all antinodes along the vector
                for k in 1.. {
                    let p = (x1 as i32 + dx * k, y1 as i32 + dy * k);

                    if p.0 < 0 || p.0 >= input.map_width as i32 || p.1 < 0 || p.1 >= input.map_height as i32 {
                        break;
                    }

                    antinode_set.insert((p.0 as usize, p.1 as usize));
                }
                for k in 1.. {
                    let p = (x2 as i32 - dx * k, y2 as i32 - dy * k);

                    if p.0 < 0 || p.0 >= input.map_width as i32 || p.1 < 0 || p.1 >= input.map_height as i32 {
                        break;
                    }

                    antinode_set.insert((p.0 as usize, p.1 as usize));
                }
            }
        }

        antinodes.insert(*antena, antinode_set);
    }

    antinodes
}

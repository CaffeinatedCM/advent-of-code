use std::collections::HashSet;

fn main() {
    let input = include_str!("./input.txt");
    let mut maps = parse_input(input);

    tilt_north(&mut maps[0]);
    println!("Before spin cycle: {}", calculate_total_load(&maps[0]));

    println!("Spinning...");
    spin_cycle(&mut maps[0], 1000000000);
    println!("After spin cycle: {}", calculate_total_load(&maps[0]));
}

fn parse_input(input: &str) -> Vec<Vec<Vec<char>>> {
    let mut maps = Vec::new();
    let mut lines = input.lines();

    let mut current_map = Vec::new();
    while let Some(line) = lines.next() {
        if line.is_empty() {
            maps.push(current_map);
            current_map = Vec::new();
            continue;
        }

        current_map.push(line.chars().collect::<Vec<char>>());
    }
    maps.push(current_map);

    maps.into_iter().map(|map| {
        let max_cols = map.iter().map(|row| row.len()).max().unwrap_or(0);
        (0..max_cols).map(|col| {
            map.iter()
                .map(|row| *row.get(col).unwrap_or(&'.'))
                .collect()
        }).collect()
    }).collect()
}

#[test]
fn test_parse_input() {
    let input = include_str!("./example1.txt");
    let maps = parse_input(input);

    assert_eq!(maps, vec![
        vec![
            vec!['O', 'O', '.', 'O', '.', 'O', '.', '.', '#', '#'],
            vec!['.', '.', '.', 'O', 'O', '.', '.', '.', '.', 'O'],
            vec!['.', 'O', '.', '.', '.', '#', 'O', '.', '.', 'O'],
            vec!['.', 'O', '.', '#', '.', '.', '.', '.', '.', '.'],
            vec!['.', '#', '.', 'O', '.', '.', '.', '.', '.', '.'],
            vec!['#', '.', '#', '.', '.', 'O', '#', '.', '#', '#'],
            vec!['.', '.', '#', '.', '.', '.', 'O', '.', '#', '.'],
            vec!['.', '.', '.', '.', 'O', '#', '.', 'O', '#', '.'],
            vec!['.', '.', '.', '.', '#', '.', '.', '.', '.', '.'],
            vec!['.', '#', '.', 'O', '.', '#', 'O', '.', '.', '.']
        ]
    ]);
}

fn tilt_north(map: &mut Vec<Vec<char>>) {
    for row in map.iter_mut() {

        for i in 0..row.len() {
            if row[i] == 'O' {
                let mut j = i;
                while j > 0 && row[j - 1] == '.' {
                    row.swap(j, j - 1);
                    j -= 1;
                }
            }
        }

    }
}

#[test]
fn test_tilt_north() {
    let input = include_str!("./example1.txt");
    let mut maps = parse_input(input);

    tilt_north(&mut maps[0]);
    assert_eq!(maps[0], vec![
            vec!['O', 'O', 'O', 'O', '.', '.', '.', '.', '#', '#'],
            vec!['O', 'O', 'O', '.', '.', '.', '.', '.', '.', '.'],
            vec!['O', '.', '.', '.', '.', '#', 'O', 'O', '.', '.'],
            vec!['O', '.', '.', '#', '.', '.', '.', '.', '.', '.'],
            vec!['.', '#', 'O', '.', '.', '.', '.', '.', '.', '.'],
            vec!['#', '.', '#', 'O', '.', '.', '#', '.', '#', '#'],
            vec!['.', '.', '#', 'O', '.', '.', '.', '.', '#', '.'],
            vec!['O', '.', '.', '.', '.', '#', 'O', '.', '#', '.'],
            vec!['.', '.', '.', '.', '#', '.', '.', '.', '.', '.'],
            vec!['.', '#', 'O', '.', '.', '#', 'O', '.', '.', '.']
    ]);
}

fn calculate_total_load(map: &Vec<Vec<char>>) -> i32 {
    let mut total_load = 0;
    let total_cols = map[0].len();
    for col in 0..total_cols {
        let mut rocks = 0;
        for row in map.iter() {
            if row[col] == 'O' {
                rocks += 1;
            }
        }
        total_load += rocks * (total_cols - col) as i32;
    }

    total_load
}

#[test]
fn test_calculate_total_load() {
    let input = include_str!("./example1.txt");
    let mut maps = parse_input(input);

    tilt_north(&mut maps[0]);
    assert_eq!(calculate_total_load(&maps[0]), 136);
}

fn tilt_south(map: &mut Vec<Vec<char>>) {
    for row in map.iter_mut() {
        for i in (0..row.len()).rev() {
            if row[i] == 'O' {
                let mut j = i;
                while j < row.len() - 1 && row[j + 1] == '.' {
                    row.swap(j, j + 1);
                    j += 1;
                }
            }
        }
    }
}

#[test]
fn test_tilt_south() {
    let input = include_str!("./example1.txt");
    let mut maps = parse_input(input);

    tilt_south(&mut maps[0]);
    assert_eq!(maps[0], vec![
        vec!['.', '.', '.', '.', 'O', 'O', 'O', 'O', '#', '#'],
        vec!['.', '.', '.', '.', '.', '.', '.', 'O', 'O', 'O'],
        vec!['.', '.', '.', '.', 'O', '#', '.', '.', 'O', 'O'],
        vec!['.', '.', 'O', '#', '.', '.', '.', '.', '.', '.'],
        vec!['.', '#', '.', '.', '.', '.', '.', '.', '.', 'O'],
        vec!['#', '.', '#', '.', '.', 'O', '#', '.', '#', '#'],
        vec!['.', '.', '#', '.', '.', '.', '.', 'O', '#', '.'],
        vec!['.', '.', '.', '.', 'O', '#', '.', 'O', '#', '.'],
        vec!['.', '.', '.', '.', '#', '.', '.', '.', '.', '.'],
        vec!['.', '#', '.', '.', 'O', '#', '.', '.', '.', 'O']
    ]);
}

fn tilt_west(map: &mut Vec<Vec<char>>) {
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] == 'O' {
                let mut j = y;
                while j > 0 && map[j - 1][x] == '.' {
                    let temp = map[j][x];
                    map[j][x] = map[j - 1][x];
                    map[j - 1][x] = temp;
                    j -=1;
                }
            }
        }
    }
}

#[test]
fn test_tilt_west() {
    let input = include_str!("./example1.txt");
    let mut maps = parse_input(input);

    tilt_west(&mut maps[0]);
    assert_eq!(maps[0], vec![
        vec!['O', 'O', '.', 'O', 'O', 'O', 'O', 'O', '#', '#'],
        vec!['.', 'O', '.', 'O', 'O', '.', '.', '.', '.', 'O'],
        vec!['.', 'O', '.', '.', '.', '#', '.', '.', '.', 'O'],
        vec!['.', '.', '.', '#', '.', 'O', '.', '.', '.', '.'],
        vec!['.', '#', '.', 'O', '.', '.', '.', '.', '.', '.'],
        vec!['#', '.', '#', 'O', '.', '.', '#', '.', '#', '#'],
        vec!['.', '.', '#', '.', '.', '.', 'O', '.', '#', '.'],
        vec!['.', '.', '.', '.', '.', '#', 'O', '.', '#', '.'],
        vec!['.', '.', '.', '.', '#', '.', '.', '.', '.', '.'],
        vec!['.', '#', '.', '.', '.', '#', '.', '.', '.', '.']
    ]);
}

fn tilt_east(map: &mut Vec<Vec<char>>) {
    for y in (0..map.len()).rev() {
        for x in 0..map[0].len() {
            if map[y][x] == 'O' {
                let mut j = y;
                while j < map.len() - 1 && map[j + 1][x] == '.' {
                    let temp = map[j][x];
                    map[j][x] = map[j + 1][x];
                    map[j + 1][x] = temp;
                    j += 1;
                }
            }
        }
    }
}

#[test]
fn test_tilt_east() {
    let input = include_str!("./example1.txt");
    let mut maps = parse_input(input);

    tilt_east(&mut maps[0]);
    assert_eq!(maps[0], vec![
        vec!['.', '.', '.', '.', '.', '.', '.', '.', '#', '#'],
        vec!['.', 'O', '.', 'O', '.', 'O', '.', '.', '.', '.'],
        vec!['.', 'O', '.', 'O', '.', '#', '.', '.', '.', '.'],
        vec!['.', 'O', '.', '#', '.', '.', '.', '.', '.', 'O'],
        vec!['O', '#', '.', '.', '.', '.', 'O', '.', '.', 'O'],
        vec!['#', '.', '#', '.', '.', '.', '#', '.', '#', '#'],
        vec!['.', '.', '#', '.', 'O', 'O', '.', '.', '#', '.'],
        vec!['.', '.', '.', '.', 'O', '#', '.', '.', '#', '.'],
        vec!['.', '.', '.', 'O', '#', '.', 'O', '.', '.', '.'],
        vec!['.', '#', '.', 'O', '.', '#', 'O', 'O', '.', '.']
    ]);
}

fn spin_cycle(map: &mut Vec<Vec<char>>, cycles: usize) {
    let mut seen : HashSet<String> = HashSet::new();

    for c in 0..cycles {
        tilt_north(map);
        tilt_west(map);
        tilt_south(map);
        tilt_east(map);

        if c % 1000 == 0 {
            println!("Cycle: {}", c);
        }

        let map_str = map.iter().map(|row| row.iter().collect::<String>()).collect::<Vec<String>>().join("\n");
        if seen.contains(&map_str) {
            break;
        }
    }


}

#[test]
fn test_spin_cycle() {
    let input = include_str!("./example1.txt");
    let mut maps = parse_input(input);
    let output = include_str!("./spin1.txt");
    let output_map = parse_input(output);

    spin_cycle(&mut maps[0], 1);
    assert_eq!(maps[0], output_map[0]);
}

#[test]
fn test_spin_cycle_3() {
    let input = include_str!("./example1.txt");
    let mut maps = parse_input(input);
    let output = include_str!("./spin3.txt");
    let output_map = parse_input(output);

    spin_cycle(&mut maps[0], 3);
    assert_eq!(maps[0], output_map[0]);
}
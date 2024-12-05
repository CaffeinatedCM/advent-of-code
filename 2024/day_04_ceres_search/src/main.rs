fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    if args.len() < 2 {
        println!("Usage: {} <input file>", args[0]);
        std::process::exit(1);
    }

    let input = std::fs::read_to_string(&args[1]).unwrap();
    let map = parse_input(&input);

    let xmas = count_xmas(&map);

    println!("XMAS: {}", xmas);

    let x_mas = count_x_mas(&map);

    println!("X-MAS: {}", x_mas);
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn count_xmas(map: &Vec<Vec<char>>) -> i32 {
    let mut xmas = 0;

    let directions: [(i32, i32); 8] = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    for y in 0..map.len() {
        for x in 0..map[y].len() {
            for &(dx, dy) in &directions {
                if search_word(map, x, y, &['X', 'M', 'A', 'S'], 0, (dx, dy)) {
                    xmas += 1;
                }
            }
        }
    }

    xmas
}

fn search_word(
    map: &Vec<Vec<char>>,
    x: usize,
    y: usize,
    target_string: &[char],
    current_index: usize,
    direction: (i32, i32),
) -> bool {
    if x >= map[0].len() || y >= map.len() {
        return false;
    }

    if map[y][x] != target_string[current_index] {
        return false;
    }

    if current_index == target_string.len() - 1 {
        return true;
    }

    let new_x = x as i32 + direction.0;
    let new_y = y as i32 + direction.1;

    if new_x >= 0
        && new_y >= 0
        && (new_x as usize) < map[0].len()
        && (new_y as usize) < map.len()
    {
        return search_word(map, new_x as usize, new_y as usize, target_string, current_index + 1, direction);
    }

    false
}

fn count_x_mas(map: &Vec<Vec<char>>) -> i32 {
    let mut xmas = 0;

    let directions = [(-1, -1), (-1, 1), (1, -1), (1, 1)];

    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] == 'A' {
                let mut found_mases = 0;

                for direction in directions {
                    // start from opposite direction of direction
                    let start_x = x as i32 - direction.0;
                    let start_y = y as i32 - direction.1;

                    if start_x < 0 || start_y < 0 || start_x >= map[y].len() as i32 || start_y >= map.len() as i32 {
                        continue;
                    }

                    if search_word(map, start_x as usize, start_y as usize, &['M', 'A', 'S'], 0, direction) {
                        found_mases += 1;
                    }
                }

                if found_mases == 2 {
                    xmas += 1;
                }
            }
        }
    }

    xmas
}


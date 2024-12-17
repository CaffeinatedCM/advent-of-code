fn main() {
    let input_file = std::env::args()
        .nth(1)
        .expect("Usage: <program> <input file>");

    let input_str = std::fs::read_to_string(input_file).unwrap();
    let input = parse_input(&input_str);

    println!("Before simulation:");
    print_map(&input.map);
    let after_simulation = simulate(&input);
    println!("After simulation:");
    print_map(&after_simulation);
    let gps_score = gps(&after_simulation);
    println!("GPS: {}", gps_score);
    let wide_map_input = make_wide_map(&input);
    println!("Wide map:");
    print_map(&wide_map_input.map);
    println!("Wide map start pos: {:?}", wide_map_input.start_pos);
    let after_wide_simulation = simulate(&wide_map_input);
    println!("After wide simulation:");
    print_map(&after_wide_simulation);
    let wide_gps = gps(&after_wide_simulation);
    println!("Wide GPS: {}", wide_gps);
}

fn print_map(map: &Vec<Vec<char>>) {
    for row in map {
        println!("{}", row.iter().collect::<String>());
    }
}

#[derive(Debug)]
struct Input {
    map: Vec<Vec<char>>,
    start_pos: (usize, usize),
    moves: Vec<char>,
}

fn parse_input(input: &str) -> Input {
    let mut lines = input.lines();
    let map: Vec<Vec<char>> = lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(|line| line.chars().collect())
        .collect();

    let start_pos = map
        .iter()
        .enumerate()
        .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, &c)| (x, y, c)))
        .find(|(_, _, c)| *c == '@')
        .map(|(x, y, _)| (x, y))
        .expect("Expected start position");

    let moves = lines
        .take_while(|line| !line.is_empty())
        .flat_map(|line| line.chars())
        .collect::<Vec<char>>();

    Input {
        map,
        start_pos,
        moves,
    }
}

fn make_wide_map(input: &Input) -> Input {
    let map = &input.map;
    let mut wide_map = vec![vec![' '; map[0].len() * 2]; map.len()];

    for y in 0..map.len() {
        for x in 0..map[0].len() {
            match map[y][x] {
                '#' => {
                    wide_map[y][x * 2] = '#';
                    wide_map[y][x * 2 + 1] = '#';
                }
                '.' => {
                    wide_map[y][x * 2] = '.';
                    wide_map[y][x * 2 + 1] = '.';
                }
                '@' => {
                    wide_map[y][x * 2] = '@';
                    wide_map[y][x * 2 + 1] = '.';
                }
                'O' => {
                    wide_map[y][x * 2] = '[';
                    wide_map[y][x * 2 + 1] = ']';
                }
                _ => {}
            }
        }
    }

    Input {
        map: wide_map,
        start_pos: (input.start_pos.0 * 2, input.start_pos.1),
        moves: input.moves.clone(),
    }
}

fn simulate(input: &Input) -> Vec<Vec<char>> {
    let mut map = input.map.clone();
    let mut pos = (input.start_pos.0 as i32, input.start_pos.1 as i32);

    for &m in input.moves.iter() {
        let (dx, dy) = match m {
            '^' => (0, -1),
            'v' => (0, 1),
            '<' => (-1, 0),
            '>' => (1, 0),
            _ => panic!("Invalid move: {}", m),
        };

        let new_pos = (pos.0 as i32 + dx, pos.1 as i32 + dy);
        match map[new_pos.1 as usize][new_pos.0 as usize] {
            '.' => {
                map[pos.1 as usize][pos.0 as usize] = '.';
                pos = new_pos;
                map[pos.1 as usize][pos.0 as usize] = '@';
            }
            'O' => {
                if push(&mut map, new_pos, (dx, dy)) {
                    map[pos.1 as usize][pos.0 as usize] = '.';
                    pos = new_pos;
                    map[pos.1 as usize][pos.0 as usize] = '@';
                }
            }
            '[' => {
                if dy == 0 && push(&mut map, new_pos, (dx, dy)) {
                    map[pos.1 as usize][pos.0 as usize] = '.';
                    pos = new_pos;
                    map[pos.1 as usize][pos.0 as usize] = '@';
                } else if dy != 0 {
                    let other_half = (new_pos.0 + 1, new_pos.1);
                    let mut tmp_map = map.clone();
                    if push(&mut tmp_map, new_pos, (dx, dy))
                        && push(&mut tmp_map, other_half, (dx, dy))
                    {
                        map = tmp_map;
                        map[pos.1 as usize][pos.0 as usize] = '.';
                        pos = new_pos;
                        map[pos.1 as usize][pos.0 as usize] = '@';
                    }
                }
            }
            ']' => {
                if dy == 0 && push(&mut map, new_pos, (dx, dy)) {
                    map[pos.1 as usize][pos.0 as usize] = '.';
                    pos = new_pos;
                    map[pos.1 as usize][pos.0 as usize] = '@';
                } else if dy != 0 {
                    let other_half = (new_pos.0 - 1, new_pos.1);
                    let mut tmp_map = map.clone();
                    if push(&mut tmp_map, new_pos, (dx, dy))
                        && push(&mut tmp_map, other_half, (dx, dy))
                    {
                        map = tmp_map;
                        map[pos.1 as usize][pos.0 as usize] = '.';
                        pos = new_pos;
                        map[pos.1 as usize][pos.0 as usize] = '@';
                    }
                }
            }
            '#' => {}
            _ => {}
        }
    }

    map
}

fn push(map: &mut Vec<Vec<char>>, pos: (i32, i32), dir: (i32, i32)) -> bool {
    let c = map[pos.1 as usize][pos.0 as usize];
    let new_pos = (pos.0 + dir.0, pos.1 + dir.1);
    match map[new_pos.1 as usize][new_pos.0 as usize] {
        '.' => {
            map[pos.1 as usize][pos.0 as usize] = '.';
            map[new_pos.1 as usize][new_pos.0 as usize] = c;
            true
        }
        'O' => {
            if push(map, new_pos, dir) {
                map[pos.1 as usize][pos.0 as usize] = '.';
                map[new_pos.1 as usize][new_pos.0 as usize] = c;
                return true;
            }
            false
        }
        '[' => {
            // first, is this my other half?
            // if so, push it
            if c == ']' && dir == (-1, 0) {
                if push(map, new_pos, dir) {
                    map[pos.1 as usize][pos.0 as usize] = '.';
                    map[new_pos.1 as usize][new_pos.0 as usize] = c;
                    return true;
                }
            }
            if dir.1 != 0 {
                // if I'm vertical, see if we can push both halves
                let mut tmp_map = map.clone();
                if push(&mut tmp_map, new_pos, dir)
                    && push(&mut tmp_map, (new_pos.0 + 1, new_pos.1), dir)
                {
                    *map = tmp_map;
                    map[pos.1 as usize][pos.0 as usize] = '.';
                    map[new_pos.1 as usize][new_pos.0 as usize] = c;
                    return true;
                }
                return false;
            }
            if push(map, new_pos, dir) {
                map[pos.1 as usize][pos.0 as usize] = '.';
                map[new_pos.1 as usize][new_pos.0 as usize] = c;
                return true;
            }
            false
        }
        ']' => {
            if c == '[' && dir == (1, 0) {
                if push(map, new_pos, dir) {
                    map[pos.1 as usize][pos.0 as usize] = '.';
                    map[new_pos.1 as usize][new_pos.0 as usize] = c;
                    return true;
                }
            }
            if dir.1 != 0 {
                // if I'm vertical, see if we can push both halves
                let mut tmp_map = map.clone();
                if push(&mut tmp_map, new_pos, dir)
                    && push(&mut tmp_map, (new_pos.0 - 1, new_pos.1), dir)
                {
                    *map = tmp_map;
                    map[pos.1 as usize][pos.0 as usize] = '.';
                    map[new_pos.1 as usize][new_pos.0 as usize] = c;
                    return true;
                }
                return false;
            }
            if push(map, new_pos, dir) {
                map[pos.1 as usize][pos.0 as usize] = '.';
                map[new_pos.1 as usize][new_pos.0 as usize] = c;
                return true;
            }
            false
        }
        '#' => false,
        _ => false,
    }
}

fn gps(map: &Vec<Vec<char>>) -> usize {
    map.iter()
        .enumerate()
        .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, &c)| (x, y, c)))
        .filter(|(_, _, c)| *c == 'O' || *c == '[')
        .map(|(x, y, _)| x + 100 * y)
        .sum()
}

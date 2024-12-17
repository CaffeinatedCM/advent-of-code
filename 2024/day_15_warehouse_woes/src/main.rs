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
    let gps = gps(&after_simulation);
    println!("GPS: {}", gps);
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

    Input { map, start_pos, moves }
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
            '#' => {}
            _ => {}
        }
    }

    map
}

fn push(map: &mut Vec<Vec<char>>, pos: (i32, i32), dir: (i32, i32)) -> bool {
    let new_pos = (pos.0 + dir.0, pos.1 + dir.1);
    match map[new_pos.1 as usize][new_pos.0 as usize] {
        '.' => {
            map[pos.1 as usize][pos.0 as usize] = '.';
            map[new_pos.1 as usize][new_pos.0 as usize] = 'O';
            true
        }
        'O' => {
            if push(map, new_pos, dir) {
                map[pos.1 as usize][pos.0 as usize] = '.';
                map[new_pos.1 as usize][new_pos.0 as usize] = 'O';
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
        .filter(|(_, _, c)| *c == 'O')
        .map(|(x, y, _)| x + 100*y)
        .sum()
}

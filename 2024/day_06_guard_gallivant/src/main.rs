fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    if args.len() < 2 {
        println!("Usage: {} <input file>", args[0]);
        std::process::exit(1);
    }

    let input = std::fs::read_to_string(&args[1]).unwrap();
    let map = parse_input(&input);

    let positions = count_positions(&map);
    println!("Positions: {}", positions);
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn get_start_pos(map: &Vec<Vec<char>>) -> (usize, usize) {
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] == '^' {
                return (x, y);
            }
        }
    }

    panic!("No start position found");
}

fn count_positions(map: &Vec<Vec<char>>) -> i32 {
    let mut positions = 0;
    let mut cur_pos = get_start_pos(map);
    let mut visited = vec![vec![false; map[0].len()]; map.len()];

    // up, right, down, left
    let directions: [(i32, i32); 4] = [
        (0, -1),
        (1, 0),
        (0, 1),
        (-1, 0),
    ];
    let mut cur_dir: usize = 0;

    loop {
        let (x, y) = cur_pos;
        let (dx, dy) = directions[cur_dir];
        let next_x: i32 = x as i32 + dx;
        let next_y: i32 = y as i32 + dy;

        if !visited[y][x] {
            visited[y][x] = true;
            positions += 1;
        }

        if next_x < 0 || next_x >= map[0].len() as i32 || next_y < 0 || next_y >= map.len() as i32 {
            break;
        }

        if map[next_y as usize][next_x as usize] == '#' {
            cur_dir = (cur_dir + 1) % 4;
            continue;
        }

        cur_pos = (next_x as usize, next_y as usize);
    }

    positions
}



use std::collections::HashSet;

fn main() {
    let input = include_str!("./input.txt");
    let map = parse_input(input);

    println!("Options: {}", find_possible_points(&map.map, map.start, 64));
}

struct Map {
    map: Vec<Vec<char>>,
    start: (i32, i32),
}

fn parse_input(input: &str) -> Map {
    let mut map = Vec::new();
    let mut start = (0, 0);

    for (y, line) in input.lines().enumerate() {
        let mut row = Vec::new();
        for (x, c) in line.chars().enumerate() {
            row.push(c);
            if c == 'S' {
                start = (x as i32, y as i32);
            }
        }
        map.push(row);
    }

    Map { map, start }
}

fn find_possible_points(map: &Vec<Vec<char>>, position: (i32, i32), steps: i32) -> i32 {
    let (x, y) = position;

    if x < 0
        || x >= map.len() as i32
        || y < 0
        || y >= map[0].len() as i32
        || map[y as usize][x as usize] == '#'
    {
        return 0;
    }

    if steps == 0 {
        return 1;
    }

    let mut points = HashSet::new();
    points.insert((x, y));
    for _ in 0..steps {
        let mut next_points = HashSet::new();
        for point in points {
            let (x, y) = point;

            if x < 0
                || x >= map[0].len() as i32
                || y < 0
                || y >= map.len() as i32
                || map[y as usize][x as usize] == '#'
            {
                continue;
            }

            next_points.insert((x + 1, y));
            next_points.insert((x - 1, y));
            next_points.insert((x, y + 1));
            next_points.insert((x, y - 1));
        }
        points = next_points;
    }

    points
        .iter()
        .filter(|(x, y)| x >= &0 && y >= &0)
        .filter(|(x, y)| x < &(map[0].len() as i32) && y < &(map.len() as i32))
        .filter(|(x, y)| map[*y as usize][*x as usize] != '#')
        .count() as i32
}

#[test]
fn test_find_possible_points() {
    let input = include_str!("./example1.txt");
    let map = parse_input(input);

    assert_eq!(find_possible_points(&map.map, map.start, 6), 16);
}

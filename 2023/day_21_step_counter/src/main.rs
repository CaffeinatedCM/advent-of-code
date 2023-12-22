use rayon::prelude::*;
use std::collections::HashSet;
use std::sync::{Arc, Mutex};

fn main() {
    let input = include_str!("./input.txt");
    let map = parse_input(input);

    println!("Options: {}", find_possible_points(&map, 64));
    println!("Options OH MY: {}", find_possible_points(&map, 26501365));
}

struct Map {
    map: Vec<Vec<char>>,
    start: (i64, i64),
}

impl Map {
    fn at(&self, x: i64, y: i64) -> char {
        let map_width = self.map[0].len() as i64;
        let map_height = self.map.len() as i64;

        let virtual_x = (x % map_width + map_width) % map_width;
        let virtual_y = (y % map_height + map_height) % map_height;
        self.map[virtual_y as usize][virtual_x as usize]
    }
}

fn parse_input(input: &str) -> Map {
    let mut map = Vec::new();
    let mut start = (0, 0);

    for (y, line) in input.lines().enumerate() {
        let mut row = Vec::new();
        for (x, c) in line.chars().enumerate() {
            row.push(c);
            if c == 'S' {
                start = (x as i64, y as i64);
            }
        }
        map.push(row);
    }

    Map { map, start }
}

fn find_possible_points(map: &Map, steps: i64) -> i64 {
    let (x, y) = map.start;

    let mut points = HashSet::new();
    points.insert((x, y));
    for _ in 0..steps {
        let mut next_points = HashSet::new();
        next_points.par_extend(
            points
                .par_iter()
                .filter(|(x, y)| map.at(*x, *y) != '#')
                .flat_map(|(x, y)| vec![(x + 1, *y), (x - 1, *y), (*x, y + 1), (*x, y - 1)]),
        );
        points = next_points;
    }

    points.iter().filter(|(x, y)| map.at(*x, *y) != '#').count() as i64
}

#[test]
fn test_find_possible_points() {
    let input = include_str!("./example1.txt");
    let map = parse_input(input);

    assert_eq!(find_possible_points(&map, 6), 16);
    assert_eq!(find_possible_points(&map, 50), 1594);
    assert_eq!(find_possible_points(&map, 500), 167004);
    assert_eq!(find_possible_points(&map, 5000), 16733044);
}

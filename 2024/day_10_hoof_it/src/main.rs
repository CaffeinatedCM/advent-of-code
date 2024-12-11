use std::collections::VecDeque;
use std::collections::HashSet;

fn main() {
    let input_file = std::env::args().nth(1).expect("Usage: <program> <input file>");
    let input_str = std::fs::read_to_string(input_file).unwrap();

    let input = parse_input(&input_str);
    let score = score_trailheads(&input);
    println!("Score: {}", score);
    let rating = rate_trailheads(&input);
    println!("Rating: {}", rating);
}

fn parse_input(input: &str) -> Vec<Vec<i8>> {
    input.lines().map(|line| line.chars().map(|c| c.to_digit(10).unwrap() as i8).collect()).collect()
}

fn score_trailheads(map: &Vec<Vec<i8>>) -> i32 {
    let mut score = 0;

    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] == 0 {
               score += score_trailhead(map, x, y); 
            }
        }
    }

    score
}

fn score_trailhead(map: &Vec<Vec<i8>>, x: usize, y: usize) -> i32 {
    let mut score = 0;
    let mut visited = vec![vec![false; map[0].len()]; map.len()];
    let mut queue = VecDeque::new();

    queue.push_back((x, y));

    while let Some((x, y)) = queue.pop_front() {
        if visited[y][x] {
            continue;
        }

        visited[y][x] = true;
        let cur_height = map[y][x];

        if cur_height == 9 {
            score += 1;
        }

        if x > 0 && map[y][x - 1] == cur_height + 1 {
            queue.push_back((x - 1, y));
        }
        if x < map[y].len() - 1 && map[y][x + 1] == cur_height + 1 {
            queue.push_back((x + 1, y));
        }
        if y > 0 && map[y - 1][x] == cur_height + 1 {
            queue.push_back((x, y - 1));
        }
        if y < map.len() - 1 && map[y + 1][x] == cur_height + 1 {
            queue.push_back((x, y + 1));
        }
    }

    score
}

fn rate_trailheads(map: &Vec<Vec<i8>>) -> i32 {
    let mut score = 0;

    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] == 0 {
               score += rate_trailhead(map, x, y); 
            }
        }
    }

    score
}

fn rate_trailhead(map: &Vec<Vec<i8>>, x: usize, y: usize) -> i32 {
    let mut rating = 0;
    let mut visited: Vec<Vec<i32>> = vec![vec![0; map[0].len()]; map.len()];
    let mut queue = VecDeque::new();
    let mut nines = HashSet::new();

    queue.push_back((x, y));
    visited[y][x] = 1;

    while let Some((x, y)) = queue.pop_front() {
        let cur_height = map[y][x];

        if cur_height == 9 {
            nines.insert((x, y));
        }

        if x > 0 && map[y][x - 1] == cur_height + 1 {
            visited[y][x - 1] += 1;
            queue.push_back((x - 1, y));
        }
        if x < map[y].len() - 1 && map[y][x + 1] == cur_height + 1 {
            visited[y][x + 1] += 1; 
            queue.push_back((x + 1, y));
        }
        if y > 0 && map[y - 1][x] == cur_height + 1 {
            visited[y - 1][x] += 1;
            queue.push_back((x, y - 1));
        }
        if y < map.len() - 1 && map[y + 1][x] == cur_height + 1 {
            visited[y + 1][x] += 1;
            queue.push_back((x, y + 1));
        }
    }

    println!("{:?}", visited);
    for (x, y) in nines {
        rating += visited[y][x];
    }

    rating
}

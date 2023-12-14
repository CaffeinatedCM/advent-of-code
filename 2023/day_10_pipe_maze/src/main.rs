use std::collections::{HashMap, VecDeque};

fn main() {
    let input = include_str!("./input.txt");
    let maze = parse_input(input);

    println!("Furthest distance: {}", find_furthest_distance(&maze));
}

struct Maze {
    start: (usize, usize),
    map: Vec<Vec<char>>,
}

fn parse_input(input: &str) -> Maze {
    let mut map = Vec::new();
    let mut start = (0, 0);

    for (y, line) in input.lines().enumerate() {
        let mut row = Vec::new();

        for (x, c) in line.chars().enumerate() {
            if c == 'S' {
                start = (x, y);
            }

            row.push(c);
        }

        map.push(row);
    }

    Maze {
        start,
        map,
    }
}

#[test]
fn test_parse_input() {
    let input = include_str!("./example1.txt");
    let maze = parse_input(input);

    assert_eq!(maze.start, (1, 1));
    assert_eq!(maze.map, vec![
        vec!['.', '.', '.', '.', '.'],
        vec!['.', 'S', '-', '7', '.'],
        vec!['.', '|', '.', '|', '.'],
        vec!['.', 'L', '-', 'J', '.'],
        vec!['.', '.', '.', '.', '.'],
    ]);
}

fn traverse(maze: &Maze) -> Vec<Vec<i32>> {
    let mut queue = VecDeque::new();
    let mut visited = vec![vec![false; maze.map[0].len()]; maze.map.len()];
    let mut distance = vec![vec![i32::MAX; maze.map[0].len()]; maze.map.len()];

    let directions = HashMap::from([
        ('|', vec![(0_isize, 1_isize), (0, -1)]),
        ('-', vec![(1, 0), (-1, 0)]),
        ('L', vec![(0, -1), (1, 0)]),
        ('J', vec![(0, -1), (-1, 0)]),
        ('7', vec![(0, 1), (-1, 0)]),
        ('F', vec![(0, 1), (1, 0)]),
        ('S', vec![(0, 1), (0, -1), (1, 0), (-1, 0)]),
        ('.', vec![]),
    ]);

    visited[maze.start.1][maze.start.0] = true;
    distance[maze.start.1][maze.start.0] = 0;

    // Find the points that are connected to the start
    for potential_point in directions.get(&'S').unwrap() {
        let next = {
            let next_x = (maze.start.0 as isize).checked_add(potential_point.0);
            let next_y = (maze.start.1 as isize).checked_add(potential_point.1);

            if next_x.is_none() || next_y.is_none() {
                continue;
            }

            if next_x.unwrap() < 0 || next_y.unwrap() < 0 {
                continue;
            }

            (next_x.unwrap() as usize, next_y.unwrap() as usize)
        };

        // Check if next connects to the start
        for direction in directions.get(&maze.map[next.1][next.0]).unwrap() {
            let next_next = {
                let next_x = (next.0 as isize).checked_add(direction.0);
                let next_y = (next.1 as isize).checked_add(direction.1);

                if next_x.is_none() || next_y.is_none() {
                    continue;
                }

                if next_x.unwrap() < 0 || next_y.unwrap() < 0 {
                    continue;
                }

                (next_x.unwrap() as usize, next_y.unwrap() as usize)
            };

            if next_next == maze.start {
                queue.push_back(next);
                visited[next.1][next.0] = true;
                distance[next.1][next.0] = 1;
            }
        }
    }

    while !queue.is_empty() {
        let current = queue.pop_front().unwrap();

        if maze.map[current.1][current.0] == '.' {
            continue;
        }

        for direction in directions.get(&maze.map[current.1][current.0]).unwrap() {
            let next = {
                let next_x = (current.0 as isize).checked_add(direction.0);
                let next_y = (current.1 as isize).checked_add(direction.1);

                if next_x.is_none() || next_y.is_none() {
                    continue;
                }

                if next_x.unwrap() < 0 || next_y.unwrap() < 0 {
                    continue;
                }

                (next_x.unwrap() as usize, next_y.unwrap() as usize)
            };

            if maze.map[next.1][next.0] != '.' && !visited[next.1][next.0] {
                visited[next.1][next.0] = true;

                distance[next.1][next.0] = distance[current.1][current.0] + 1;

                queue.push_back(next);
            }
        }
    }

    distance
}

#[test]
fn test_traverse() {
    let input = include_str!("./example1.txt");
    let maze = parse_input(input);

    assert_eq!(traverse(&maze), vec![
        vec![i32::MAX, i32::MAX, i32::MAX, i32::MAX, i32::MAX],
        vec![i32::MAX, 0, 1, 2, i32::MAX],
        vec![i32::MAX, 1, i32::MAX, 3, i32::MAX],
        vec![i32::MAX, 2, 3, 4, i32::MAX],
        vec![i32::MAX, i32::MAX, i32::MAX, i32::MAX, i32::MAX],
    ])
}

#[test]
fn test_traverse2() {
    let input = include_str!("./example3.txt");
    let maze = parse_input(input);

    assert_eq!(traverse(&maze), vec![
        vec![i32::MAX, i32::MAX, 4, 5, i32::MAX],
        vec![i32::MAX, 2, 3, 6, i32::MAX],
        vec![0, 1, i32::MAX, 7, 8],
        vec![1, 4, 5, 6, 7],
        vec![2, 3, i32::MAX, i32::MAX, i32::MAX]
    ])
}

fn find_furthest_distance(maze: &Maze) -> i32 {
    let distances = traverse(maze);

    distances.iter().map(|row| row.iter().filter(|&&x| x != i32::MAX).max().unwrap_or_else(|| &i32::MIN)).max().unwrap().clone()
}

#[test]
fn test_find_furthest_distance() {
    let input = include_str!("./example1.txt");
    let maze = parse_input(input);

    assert_eq!(find_furthest_distance(&maze), 4);
}

#[test]
fn test_find_furthest_distance2() {
    let input = include_str!("./example3.txt");
    let maze = parse_input(input);

    assert_eq!(find_furthest_distance(&maze), 8);
}

#[test]
fn test_real_input() {
    let input = include_str!("./input.txt");
    let maze = parse_input(input);

    assert_eq!(find_furthest_distance(&maze), 6882);
}

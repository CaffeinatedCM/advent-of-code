use std::collections::VecDeque;

fn main() {
    let input_file = std::env::args()
        .nth(1)
        .expect("Usage: <program> <input file>");

    let input_str = std::fs::read_to_string(input_file).unwrap();
    let map = parse_input(&input_str);

    println!("{:?}", map);
    let price = calculate_fence_price(&map);
    println!("Price: {}", price);
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

fn calculate_fence_price(map: &Vec<Vec<char>>) -> usize {
    let mut price = 0;
    let mut visited = vec![vec![false; map[0].len()]; map.len()];
    
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if visited[y][x] {
                continue;
            }

            let (area, permimeter) = find_region(&map, &mut visited, x, y);
            price += area *  permimeter ;
            println!("Region: {} Area: {}, Perimeter: {}", map[y][x], area, permimeter);
        }
    }

    price
}

fn find_region(map: &Vec<Vec<char>>, visited: &mut Vec<Vec<bool>>, x: usize, y: usize) -> (usize, usize) {
    let mut queue = VecDeque::new();
    let mut permimeter = 0;
    let mut area = 0;

    let target = map[y][x];

    queue.push_back((x, y));
    visited[y][x] = true;

    while let Some((x, y)) = queue.pop_front() {
        area += 1;
        
        if x > 0 {
            if map[y][x-1] == target {
                if !visited[y][x-1] {
                    queue.push_back((x-1, y));
                    visited[y][x-1] = true;
                }
            } else {
                permimeter += 1;
            }
        } else {
            permimeter += 1;
        }

        if y > 0 {
            if map[y-1][x] == target {
                if !visited[y-1][x] {
                    queue.push_back((x, y-1));
                    visited[y-1][x] = true;
                }
            } else {
                permimeter += 1;
            }
        } else {
            permimeter += 1;
        }

        if x < map[0].len() - 1 {
            if map[y][x+1] == target {
                if !visited[y][x+1] {
                    queue.push_back((x+1, y));
                    visited[y][x+1] = true;
                }
            } else {
                permimeter += 1;
            }
        } else {
            permimeter += 1;
        }

        if y < map.len() - 1 {
            if map[y+1][x] == target {
                if !visited[y+1][x] {
                    queue.push_back((x, y+1));
                    visited[y+1][x] = true;
                }
            } else {
                permimeter += 1;
            }
        } else {
            permimeter += 1;
        }
    }

    (area, permimeter)
}


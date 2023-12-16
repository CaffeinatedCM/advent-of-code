fn main() {
    let input = include_str!("./input.txt");
    let maps = parse_input(input);

    println!("Sum of reflections: {}", sum_reflection(&maps));
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

        current_map.push(line.chars().collect());
    }
    maps.push(current_map);

    maps
}

#[test]
fn test_parse_input() {
    let input = include_str!("./example1.txt");
    let maps = parse_input(input);

    assert_eq!(maps, vec![
        vec![
            "#.##..##.".chars().collect::<Vec<char>>(),
            "..#.##.#.".chars().collect::<Vec<char>>(),
            "##......#".chars().collect::<Vec<char>>(),
            "##......#".chars().collect::<Vec<char>>(),
            "..#.##.#.".chars().collect::<Vec<char>>(),
            "..##..##.".chars().collect::<Vec<char>>(),
            "#.#.##.#.".chars().collect::<Vec<char>>(),
        ],
        vec![
            "#...##..#".chars().collect::<Vec<char>>(),
            "#....#..#".chars().collect::<Vec<char>>(),
            "..##..###".chars().collect::<Vec<char>>(),
            "#####.##.".chars().collect::<Vec<char>>(),
            "#####.##.".chars().collect::<Vec<char>>(),
            "..##..###".chars().collect::<Vec<char>>(),
            "#....#..#".chars().collect::<Vec<char>>(),
        ]
    ]);
}

fn find_reflection_column(map: &Vec<Vec<char>>) -> (i32, i32) {
    let num_cols = map[0].len();


    let mut reflection_col = -1;
    let mut reflection_count = 0;
    for col in 0..(num_cols -1) {
        let mut is_reflection = true;
        for row in 0..map.len() {
            if map[row][col] != map[row][col + 1] {
                is_reflection = false;
                break;
            }
        }

        if is_reflection {
            let mut count = 0;
            for c in 0..col+1 {
                let mut reflection = true;
                if col + 1 + c >= num_cols || col.checked_sub(c).is_none() {
                    count += 1;
                    continue;
                }
                for r in 0..map.len() {
                    if map[r][col-c] != map[r][col+1+c] {
                        reflection = false;
                        break;
                    }
                }
                if reflection {
                    count += 1;
                } else {
                    break;
                }
            }
            reflection_col = col as i32;
            reflection_count = count;
        }
    }

    if reflection_count == 1 {
        return (-1, 0);
    }

    (reflection_col, reflection_count)
}

#[test]
fn test_find_reflection_column() {
    let input = include_str!("./example1.txt");
    let maps = parse_input(input);

    assert_eq!(find_reflection_column(&maps[0]), (4, 5));
    assert_eq!(find_reflection_column(&maps[1]), (-1, 0));
}

fn find_reflection_row(map: &Vec<Vec<char>>) -> (i32, i32) {
    let num_rows = map.len();

    let mut reflection_row = -1;
    let mut reflection_count = 0;
    for row in 0..(num_rows -1) {
        if map[row] == map[row + 1] {
            let mut count = 0;
            for r in 0..row+1 {
                if row + 1 + r >= num_rows || row.checked_sub(r).is_none() {
                    count += 1;
                    continue;
                }
                let reflection = map[row-r] == map[row+1+r];
                if reflection {
                    count += 1;
                } else {
                    break;
                }
            }
            reflection_row = row as i32;
            reflection_count = count;
        }
    }

    if reflection_count == 1 {
        return (-1, 0);
    }

    (reflection_row, reflection_count)
}

#[test]
fn test_find_reflection_row() {
    let input = include_str!("./example1.txt");
    let maps = parse_input(input);

    assert_eq!(find_reflection_row(&maps[0]), (2, 2));
    assert_eq!(find_reflection_row(&maps[1]), (3, 4));
}

fn find_reflection(map: &Vec<Vec<char>>) -> (i32, i32) {
    let (col, col_count) = find_reflection_column(map);
    let (row, row_count) = find_reflection_row(map);

    if col_count > row_count {
        (col, col_count)
    } else {
        (row, row_count)
    }
}

#[test]
fn test_find_reflection() {
    let input = include_str!("./example1.txt");
    let maps = parse_input(input);

    assert_eq!(find_reflection(&maps[0]), (4, 5));
    assert_eq!(find_reflection(&maps[1]), (3, 4));
}

fn sum_reflection(maps: &Vec<Vec<Vec<char>>>) -> i32 {
    let mut total = 0;
    for map in maps {
        let (_col, col_count) = find_reflection_column(map);
        let (_row, row_count) = find_reflection_row(map);

        if col_count >= row_count {
            total += col_count;
        } else {
            total += 100 * row_count;
        }
    }

    total
}

#[test]
fn test_sum_reflection() {
    let input = include_str!("./example1.txt");
    let maps = parse_input(input);

    assert_eq!(sum_reflection(&maps), 405);
}
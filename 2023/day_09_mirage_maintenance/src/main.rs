fn main() {
    let input = include_str!("./input.txt");
    let map = parse_input(input);

    println!("Sum: {}", sum_next_vals(&map));

    let reversed = map.iter().map(|row| {
        let mut row = row.clone();
        row.reverse();
        row
    }).collect::<Vec<Vec<i64>>>();

    println!("Sum: {}", sum_next_vals(&reversed));
}

fn parse_input(input: &str) -> Vec<Vec<i64>> {
    input.lines().map(|line| {
        line.split_whitespace().map(|word| {
            word.parse::<i64>().unwrap()
        }).collect::<Vec<i64>>()
    }).collect::<Vec<Vec<i64>>>()
}

#[test]
fn test_parse_input() {
    let input = include_str!("./example1.txt");
    let map = parse_input(input);

    assert_eq!(map, vec![
        vec![0, 3, 6, 9, 12, 15],
        vec![1, 3, 6, 10, 15, 21],
        vec![10, 13, 16, 21, 30, 45]
    ]);
}

fn find_differences(input: &Vec<i64>) -> Vec<i64> {
    let mut differences = Vec::new();

    for i in 1..input.len() {
        differences.push(input[i] - input[i - 1]);
    }

    differences
}

#[test]
fn test_find_differences() {
    let input = vec![0, 3, 6, 9, 12, 15];
    let differences = find_differences(&input);

    assert_eq!(differences, vec![3, 3, 3, 3, 3]);
}

fn extrapolate_next_val(input: &Vec<i64>) -> i64 {
    let differences = find_differences(input);

    if differences.iter().all(|&x| x == 0) {
        return *input.last().unwrap();
    }

    input.last().unwrap() + extrapolate_next_val(&differences)
}

#[test]
fn test_extrapolate_next_val() {
    let input = vec![0, 3, 6, 9, 12, 15];
    let next_val = extrapolate_next_val(&input);

    assert_eq!(next_val, 18);
}

fn sum_next_vals(input: &Vec<Vec<i64>>) -> i64 {
    input.iter().map(|row| extrapolate_next_val(row)).sum()
}

#[test]
fn test_sum_next_vals() {
    let input = include_str!("./example1.txt");
    let map = parse_input(input);

    assert_eq!(sum_next_vals(&map), 114);
}

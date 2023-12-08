fn main() {
    let input = include_str!("./input.txt");

    let races = parse_input(input);

    let result : i64 = races.iter().map(|x| count_win_options(x)).product();

    println!("Result: {}", result);

    let combined_race = parse_input2(input);

    println!("Combined race result: {}", count_win_options(&combined_race));
}

fn parse_input_line(input: &str) -> Vec<i64> {
    input.split_whitespace().skip(1).map(|s| s.parse::<i64>().unwrap()).collect::<Vec<i64>>()
}

fn parse_input(input: &str) -> Vec<(i64, i64)> {
    let mut lines = input.lines();
    let times = parse_input_line(lines.next().unwrap());
    let distance = parse_input_line(lines.next().unwrap());

    times.into_iter().zip(distance.into_iter()).collect::<Vec<(i64, i64)>>()
}

#[test]
fn test_parse_input() {
    let input = include_str!("./example1.txt");
    let result = parse_input(input);
    assert_eq!(result, vec![(7, 9), (15, 40), (30, 200)]);
}

fn parse_input2(input: &str) -> (i64, i64) {
    let mut lines = input.lines();
    let times = parse_input_line(lines.next().unwrap()).iter().fold(String::new(), |acc, x| acc + x.to_string().as_str());
    let distance = parse_input_line(lines.next().unwrap()).iter().fold(String::new(), |acc, x| acc + x.to_string().as_str());

    (times.parse::<i64>().unwrap(), distance.parse::<i64>().unwrap())
}

#[test]
fn test_parse_input2() {
    let input = include_str!("./example1.txt");
    let result = parse_input2(input);
    assert_eq!(result, (71530, 940200));
}

fn count_win_options(race: &(i64, i64)) -> i64 {
    let mut win_options = 0;
    for t in 1..race.0 {
        let speed = t;
        let drive_time = race.0 - t;
        let distance = speed * drive_time;

        if distance <= race.1 {
            continue;
        }

        win_options += 1;
    }

    win_options
}

#[test]
fn test_count_win_options() {
    assert_eq!(count_win_options(&(7, 9)), 4);
    assert_eq!(count_win_options(&(15, 40)), 8);
    assert_eq!(count_win_options(&(30, 200)), 9);
}

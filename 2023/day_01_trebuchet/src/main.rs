use once_cell::sync::Lazy;
use regex::{Match, Regex};

fn main() {
    let input = include_str!("./input.txt");

    println!(
        "Sum of calibration numbers: {}",
        sum_calibration_numbers(input)
    );
}

fn parse_digit(str: &str) -> i32 {
    if str.len() == 1 {
        return str.parse::<i32>().unwrap();
    }

    match str {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => panic!("Invalid digit: {}", str),
    }
}

fn get_calibration_number(str: &str) -> i32 {
    let mut first_digit = 0;
    let mut second_digit = 0;

    static RE: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"(?i)one|two|three|four|five|six|seven|eight|nine|\d").unwrap());

    for i in 0..str.len() {
        let match_str = &str[i..];
        let mat = RE.find(match_str);
        match mat {
            Some(mat) => {
                let digit = parse_digit(mat.as_str());
                if first_digit == 0 {
                    first_digit = digit;
                }
                second_digit = digit;
            }
            None => {
                break;
            }
        }
    }

    return (first_digit * 10) + second_digit;
}

#[test]
fn test_get_calibration_number_oneight() {
    assert_eq!(get_calibration_number("oneight"), 18);
}

fn sum_calibration_numbers(input: &str) -> i32 {
    let mut result = 0;
    for line in input.lines() {
        result += get_calibration_number(line);
    }

    return result;
}

#[test]
fn test_sum_calibration_numbers() {
    let input = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet";
    assert_eq!(sum_calibration_numbers(input), 142);
}

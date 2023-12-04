use std::cmp::max;

const MAX_RED_CUBES: i32 = 12;
const MAX_GREEN_CUBES: i32 = 13;
const MAX_BLUE_CUBES: i32 = 14;

fn main() {
    let input = include_str!("./input.txt");

    let games = input.lines().map(|line| parse_input_line(line)).collect::<Vec<Game>>();

    println!(
        "Sum of possible game IDs: {}",
        sum_possible_game_ids(&games, MAX_RED_CUBES, MAX_BLUE_CUBES, MAX_GREEN_CUBES)
    );
    println!(
        "Sum of games min power: {}",
        sum_games_min_power(&games)
    );
}

#[derive(Eq, PartialEq, Debug)]
struct DiceSet {
    num_red_cubes: i32,
    num_green_cubes: i32,
    num_blue_cubes: i32,
}

struct Game {
    id: i32,
    rounds: Vec<DiceSet>,
}

impl Game {
    fn min_power(&self) -> i32 {
        let mut min_dice_set = DiceSet { num_red_cubes: 0, num_blue_cubes: 0, num_green_cubes: 0 };

        for round in &self.rounds {
            min_dice_set.num_red_cubes = max(min_dice_set.num_red_cubes, round.num_red_cubes);
            min_dice_set.num_green_cubes = max(min_dice_set.num_green_cubes, round.num_green_cubes);
            min_dice_set.num_blue_cubes = max(min_dice_set.num_blue_cubes, round.num_blue_cubes);
        }

        min_dice_set.num_red_cubes * min_dice_set.num_green_cubes * min_dice_set.num_blue_cubes
    }
}

#[test]
fn test_min_power() {
    let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
    let game = parse_input_line(input);
    assert_eq!(game.min_power(), 48);
}


fn parse_input_line(input: &str) -> Game {
    let mut game = Game { id: 0, rounds: Vec::new() };

    let split: Vec<&str> = input.split(":").collect();
    let game_split = split[0].split_whitespace().collect::<Vec<&str>>();
    game.id = game_split[1].parse::<i32>().unwrap();

    for round in split[1].split(';') {
        let mut game_round = DiceSet { num_red_cubes: 0, num_blue_cubes: 0, num_green_cubes: 0 };
        for cube in round.split(',') {
            let cube_split: Vec<&str> = cube.trim().split(' ').collect();
            let num_cubes = cube_split[0].parse::<i32>().unwrap();
            match cube_split[1] {
                "red" => game_round.num_red_cubes = num_cubes,
                "blue" => game_round.num_blue_cubes = num_cubes,
                "green" => game_round.num_green_cubes = num_cubes,
                _ => panic!("Invalid cube color: {}", cube_split[1]),
            }
        }
        game.rounds.push(game_round);
    }

    game
}

#[test]
fn test_parse_input_line() {
    let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
    let game = parse_input_line(input);
    assert_eq!(game.id, 1);
    assert_eq!(game.rounds.len(), 3);
    assert_eq!(game.rounds[0], DiceSet { num_red_cubes: 4, num_blue_cubes: 3, num_green_cubes: 0 });
    assert_eq!(game.rounds[1], DiceSet { num_red_cubes: 1, num_blue_cubes: 6, num_green_cubes: 2 });
    assert_eq!(game.rounds[2], DiceSet { num_red_cubes: 0, num_blue_cubes: 0, num_green_cubes: 2 });
}

fn check_game_possible(game: &Game, max_red_cubes: i32, max_blue_cubes: i32, max_green_cubes: i32) -> bool {
    for round in &game.rounds {
        if round.num_red_cubes > max_red_cubes {
            return false;
        }
        if round.num_blue_cubes > max_blue_cubes {
            return false;
        }
        if round.num_green_cubes > max_green_cubes {
            return false;
        }
    }

    true
}

#[test]
fn test_possible_game() {
    let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
    let game = parse_input_line(input);
    assert_eq!(check_game_possible(&game, 4, 6, 2), true);
}

#[test]
fn test_inpossible_game() {
    let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
    let game = parse_input_line(input);
    assert_eq!(check_game_possible(&game, 4, 6, 1), false);
}

fn sum_possible_game_ids(games: &[Game], max_red_cubes: i32, max_blue_cubes: i32, max_green_cubes: i32) -> i32 {
    let mut result = 0;

    for game in games {
        if check_game_possible(&game, max_red_cubes, max_blue_cubes, max_green_cubes) {
            result += game.id;
        }
    }

    result
}

#[test]
fn test_part_1_example() {
    let input = include_str!("./example1.txt");
    let games = input.lines().map(|line| parse_input_line(line)).collect::<Vec<Game>>();
    assert_eq!(sum_possible_game_ids(&games, 12, 13, 14), 8);
}

fn sum_games_min_power(games: &[Game]) -> i32 {
    let mut result = 0;

    for game in games {
        result += game.min_power();
    }

    result
}

#[test]
fn test_part_2_example() {
    let input = include_str!("./example1.txt");
    let games = input.lines().map(|line| parse_input_line(line)).collect::<Vec<Game>>();
    assert_eq!(sum_games_min_power(&games), 2286);
}

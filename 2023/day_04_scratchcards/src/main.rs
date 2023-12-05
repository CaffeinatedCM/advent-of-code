fn main() {
    let input = include_str!("./input.txt");
    let cards = parse_input(input);

    println!(
        "Sum of scores: {}",
        sum_scores(&cards)
    );
}


#[derive(Debug, Eq, PartialEq)]
struct GameCard {
    id: usize,
    winning_numbers: Vec<u32>,
    numbers: Vec<u32>,
    score: u32,
}

fn parse_input(input: &str) -> Vec<GameCard> {
    let mut result = Vec::new();

    for line in input.lines() {
        let (label, numbers) = line.split_once(":").unwrap();
        let parts = &label.split_whitespace().collect::<Vec<&str>>()[..];
        let id = parts[1].parse::<usize>().unwrap();

        let (winning_numbers_str, numbers_str) = numbers.split_once("|").unwrap();
        let winning_numbers = winning_numbers_str.split_whitespace().map(|n| n.parse::<u32>().unwrap()).collect::<Vec<u32>>();
        let numbers = numbers_str.split_whitespace().map(|n| n.parse::<u32>().unwrap()).collect::<Vec<u32>>();

        let matches = numbers.iter().filter(|n| winning_numbers.contains(n)).count() as i32;

        result.push(GameCard {
            id,
            winning_numbers,
            numbers,
            score: {
                if matches == 0 {
                    0
                } else {
                    2_u32.pow((matches - 1) as u32)
                }
            },
        });
    }

    result
}

#[test]
fn test_parse_input() {
    let input = "Card 1: 1 2 3 4 5 | 6 7 8 9 10 5 1 ";
    let result = parse_input(input);

    assert_eq!(result.len(), 1);
    assert_eq!(result[0], GameCard {
        id: 1,
        winning_numbers: vec![1, 2, 3, 4, 5],
        numbers: vec![6, 7, 8, 9, 10, 5, 1],
        score: 2,
    })
}

fn sum_scores(cards: &Vec<GameCard>) -> u32 {
    cards.iter().map(|c| c.score).sum()
}

#[test]
fn test_sum_scores() {
    let input = include_str!("./example1.txt");
    let cards = parse_input(input);

    assert_eq!(sum_scores(&cards), 13);
}
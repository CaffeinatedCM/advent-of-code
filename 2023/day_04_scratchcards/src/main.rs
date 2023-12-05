use std::collections::HashMap;

fn main() {
    let input = include_str!("./input.txt");
    let cards = parse_input(input);

    println!(
        "Sum of scores: {}",
        sum_scores(&cards)
    );

    let mut state = CardsState {
        queue: cards.iter().map(|c| c.id).collect::<Vec<usize>>(),
        cards,
        memo: HashMap::new(),
    };

    state.queue.reverse();

    println!(
        "Total cards: {}",
        solve_for_total_cards(&mut state)
    );
}


#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct GameCard {
    id: usize,
    winning_numbers: Vec<u32>,
    numbers: Vec<u32>,
    matches: Vec<u32>,
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

        let matches = numbers.iter().filter(|n| winning_numbers.contains(n)).map(|x| *x).collect::<Vec<u32>>();
        let match_count = matches.len();

        result.push(GameCard {
            id,
            winning_numbers,
            numbers,
            matches,
            score: {
                if match_count == 0 {
                    0
                } else {
                    2_u32.pow((match_count - 1) as u32)
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
        matches: vec![5, 1],
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

struct CardsState {
    queue: Vec<usize>,
    cards: Vec<GameCard>,
    memo: HashMap<usize, usize>
}

fn solve_for_total_cards(state: &mut CardsState) -> u32 {
    if state.queue.is_empty() {
        return 0;
    }

    let current_card_id = state.queue.remove(0);

    if state.memo.contains_key(&current_card_id) {
        return 1 + *state.memo.get(&current_card_id).unwrap() as u32;
    }

    let current_card = &state.cards[current_card_id - 1];
    let mut total = current_card.matches.len();
    for c in &state.cards[(current_card.id )..(current_card.id  + current_card.matches.len())] {
        total += state.memo.get(&c.id).unwrap();
    }

    state.memo.insert(current_card.id, total);

    1 + total as u32 + solve_for_total_cards(state)
}

#[test]
fn test_solve_total_cards() {
    let input = include_str!("./example1.txt");
    let cards = parse_input(input);

    let mut state = CardsState {
        queue: cards.iter().map(|c| c.id).collect::<Vec<usize>>(),
        cards,
        memo: HashMap::new(),
    };

    state.queue.reverse();

    assert_eq!(solve_for_total_cards(&mut state), 30);
}
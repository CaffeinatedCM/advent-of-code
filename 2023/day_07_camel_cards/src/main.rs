use std::str::FromStr;

fn main() {
    let input = include_str!("./input.txt");

    let hands = parse_input(input);

    println!("Total winnings: {}", total_winnings(hands));
}

#[derive(Eq, PartialEq, Debug, PartialOrd, Ord, Copy, Clone)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace
}

#[derive(Eq, PartialEq, Debug, PartialOrd, Ord, Copy, Clone)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Eq, PartialEq, Debug, Copy, Clone, Ord)]
struct Hand {
    bid: i32,
    cards: [Card; 5],
    hand_type: HandType
}

impl Hand {
    fn new(bid: i32, cards: [Card; 5]) -> Hand {
        let mut hand_type = HandType::HighCard;

        let mut card_counts = [0; 13];
        for card in cards.iter() {
            card_counts[*card as usize] += 1;
        }

        let mut pair_count = 0;
        let mut three_of_a_kind = false;
        let mut four_of_a_kind = false;
        let mut five_of_a_kind = false;

        for count in card_counts.iter() {
            match count {
                2 => pair_count += 1,
                3 => three_of_a_kind = true,
                4 => four_of_a_kind = true,
                5 => five_of_a_kind = true,
                _ => ()
            }
        }

        if five_of_a_kind {
            hand_type = HandType::FiveOfAKind;
        } else if four_of_a_kind {
            hand_type = HandType::FourOfAKind;
        } else if three_of_a_kind && pair_count == 1 {
            hand_type = HandType::FullHouse;
        } else if three_of_a_kind {
            hand_type = HandType::ThreeOfAKind;
        } else if pair_count == 2 {
            hand_type = HandType::TwoPair;
        } else if pair_count == 1 {
            hand_type = HandType::OnePair;
        }

        Hand {
            bid,
            cards,
            hand_type
        }
    }
}

impl FromStr for Hand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cards = [Card::Two; 5];

        let mut split = s.split_whitespace();
        let card_str = split.next().unwrap().chars();

        for (i, card) in card_str.enumerate() {
            cards[i] = match card {
                '2' => Card::Two,
                '3' => Card::Three,
                '4' => Card::Four,
                '5' => Card::Five,
                '6' => Card::Six,
                '7' => Card::Seven,
                '8' => Card::Eight,
                '9' => Card::Nine,
                'T' => Card::Ten,
                'J' => Card::Jack,
                'Q' => Card::Queen,
                'K' => Card::King,
                'A' => Card::Ace,
                _ => panic!("Invalid card")
            };
        }

        let bid = split.next().unwrap().parse::<i32>().unwrap();

        Ok(Hand::new(bid, cards))
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Hand) -> Option<std::cmp::Ordering> {
        if self.hand_type != other.hand_type {
            return Some(self.hand_type.cmp(&other.hand_type));
        }

        Some(self.cards.cmp(&other.cards))
    }
}

#[test]
fn test_hand_from_str() {
    let hand = Hand::from_str("32T3K 765").unwrap();
    assert_eq!(hand, Hand::new(765, [Card::Three, Card::Two, Card::Ten, Card::Three, Card::King]));
}

#[test]
fn test_hand_sort() {
    let hands = include_str!("./example1.txt").lines().map(|s| Hand::from_str(s).unwrap()).collect::<Vec<Hand>>();

    let mut sorted = hands.clone();
    sorted.sort();

    assert_eq!(sorted, vec![
        Hand::from_str("32T3K 765").unwrap(),
        Hand::from_str("KTJJT 220").unwrap(),
        Hand::from_str("KK677  28").unwrap(),
        Hand::from_str("T55J5 684").unwrap(),
        Hand::from_str("QQQJA 483").unwrap(),
    ]);
}

fn parse_input(input: &str) -> Vec<Hand> {
    input.lines().map(|s| Hand::from_str(s).unwrap()).collect::<Vec<Hand>>()
}

fn total_winnings(mut hands: Vec<Hand>) -> i32 {
    let mut total = 0;

    hands.sort();
    for (i, hand) in hands.iter().enumerate() {
        total += hand.bid * (i as i32 + 1);
    }

    total
}

#[test]
fn test_total_winnings() {
    let input = include_str!("./example1.txt");
    let hands = parse_input(input);
    assert_eq!(total_winnings(hands), 6440);
}

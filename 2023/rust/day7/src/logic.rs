use std::collections::HashMap;

pub fn part_1(lines: &[String]) -> u32 {
    execute(lines, false)
}

pub fn part_2(lines: &[String]) -> u32 {
    execute(lines, true)
}

fn execute(lines: &[String], joker: bool) -> u32 {
    let mut hands = lines
        .iter()
        .map(|line| {
            let (hand, bid) = parse_line(line);
            if joker {
                (classify_hand_joker(hand), bid)
            } else {
                (classify_hand_normal(hand), bid)
            }
        })
        .collect::<Vec<(Hand, u32)>>();

    sort_hand(&mut hands, joker);

    calculate_winnings(&hands)
}

fn parse_line(line: &str) -> (&str, u32) {
    let parts: Vec<&str> = line.split_whitespace().collect();
    (parts[0], parts[1].parse().unwrap())
}

fn classify_hand_normal(hand: &str) -> Hand {
    let mut counts = HashMap::new();
    for card in hand.chars() {
        *counts.entry(card).or_insert(0) += 1;
    }

    let unique_cards = counts.len();
    let max_count = counts.values().max().unwrap();

    let hand_type = match (unique_cards, max_count) {
        (1, 5) => HandType::FiveOfAKind,
        (2, 4) => HandType::FourOfAKind,
        (2, 3) => HandType::FullHouse,
        (3, 3) => HandType::ThreeOfAKind,
        (3, 2) => HandType::TwoPair,
        (4, 2) => HandType::OnePair,
        _ => HandType::HighCard,
    };

    Hand {
        hand_str: hand.into(),
        hand_type,
    }
}

fn classify_hand_joker(hand: &str) -> Hand {
    let mut counts = HashMap::new();

    if hand == "JJJJJ" {
        return Hand {
            hand_str: hand.into(),
            hand_type: HandType::FiveOfAKind,
        };
    } else {
        for card in hand.chars() {
            if card != 'J' {
                *counts.entry(card).or_insert(0) += 1;
            }
        }
    }

    let (most_common_card, _) = counts.iter().max_by_key(|(_, count)| *count).unwrap();

    let jokers_replaced = hand.replace('J', &most_common_card.to_string());

    Hand {
        hand_str: hand.into(),
        hand_type: classify_hand_normal(&jokers_replaced).hand_type,
    }
}

fn sort_hand(hands: &mut [(Hand, u32)], joker: bool) {
    hands.sort_by(|a, b| {
        let hand_type = a.0.hand_type.cmp(&b.0.hand_type);
        if hand_type != std::cmp::Ordering::Equal {
            return hand_type;
        }

        a.0.hand_str
            .chars()
            .map(|c| card_rank_lut(c, joker))
            .cmp(b.0.hand_str.chars().map(|c| card_rank_lut(c, joker)))
    });
}

fn card_rank_lut(card: char, joker: bool) -> u32 {
    match card {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => {
            if joker {
                1
            } else {
                11
            }
        }
        'T' => 10,
        _ => card.to_digit(10).unwrap(),
    }
}

fn calculate_winnings(hands: &[(Hand, u32)]) -> u32 {
    hands
        .iter()
        .enumerate()
        .map(|(rank, &(_, bid))| bid * (rank as u32 + 1))
        .sum()
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug)]
struct Hand {
    hand_str: Box<str>,
    hand_type: HandType,
}

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
            let (cards, bid) = parse_line(line);
            let hand = if joker {
                classify_hand_joker(cards)
            } else {
                classify_hand_normal(cards)
            };
            HandBid { hand, bid }
        })
        .collect::<Vec<_>>();

    sort_hand(&mut hands, joker);

    calculate_winnings(&hands)
}

fn parse_line(line: &str) -> (&str, u32) {
    let parts: Vec<&str> = line.split_whitespace().collect();

    (parts[0], parts[1].parse().unwrap())
}

fn classify_hand_normal(hand: &str) -> HandPlay {
    let mut counts = HashMap::new();
    for card in hand.chars() {
        *counts.entry(card).or_insert(0) += 1;
    }

    let unique_cards = counts.len();
    let max_count = counts.values().max().unwrap();

    let hand_type = match (unique_cards, max_count) {
        (1, 5) => Play::FiveOfAKind,
        (2, 4) => Play::FourOfAKind,
        (2, 3) => Play::FullHouse,
        (3, 3) => Play::ThreeOfAKind,
        (3, 2) => Play::TwoPair,
        (4, 2) => Play::OnePair,
        _ => Play::HighCard,
    };

    HandPlay {
        string: hand.into(),
        play: hand_type,
    }
}

fn classify_hand_joker(cards: &str) -> HandPlay {
    let mut counts = HashMap::new();

    if cards == "JJJJJ" {
        return HandPlay {
            string: cards.into(),
            play: Play::FiveOfAKind,
        };
    } else {
        for card in cards.chars() {
            if card != 'J' {
                *counts.entry(card).or_insert(0) += 1;
            }
        }
    }

    let (most_common_card, _) = counts.iter().max_by_key(|(_, count)| *count).unwrap();

    let jokers_replaced = cards.replace('J', &most_common_card.to_string());

    HandPlay {
        string: cards.into(),
        play: classify_hand_normal(&jokers_replaced).play,
    }
}

fn sort_hand(hands: &mut [HandBid], joker: bool) {
    hands.sort_by(|a, b| {
        let hand_type = a.hand.play.cmp(&b.hand.play);
        if hand_type != std::cmp::Ordering::Equal {
            return hand_type;
        }

        a.hand
            .string
            .chars()
            .map(|card| card_rank_lut(card, joker))
            .cmp(b.hand.string.chars().map(|c| card_rank_lut(c, joker)))
    });
}

fn calculate_winnings(hands: &[HandBid]) -> u32 {
    hands
        .iter()
        .enumerate()
        .map(|(rank, hand_bid)| hand_bid.bid * (rank as u32 + 1))
        .sum()
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

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum Play {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

struct HandPlay {
    string: Box<str>,
    play: Play,
}

struct HandBid {
    hand: HandPlay,
    bid: u32,
}

pub fn part_1(lines: &[String]) -> u32 {
    let (winning, player) = extract_cards(lines);

    total_points(&winning, &player)
}

pub fn part_2(lines: &[String]) -> u32 {
    let (winning, player) = extract_cards(lines);

    total_cards(&winning, &player)
}

fn extract_cards(lines: &[String]) -> (Vec<Vec<u8>>, Vec<Vec<u8>>) {
    lines.iter().fold(
        (Vec::new(), Vec::new()),
        |(mut winning_cards, mut player_cards), line| {
            let (_card, numbers_pre_split) = line.split_once(':').unwrap();
            let (winning_numbers, player_numbers) = numbers_pre_split.split_once('|').unwrap();

            winning_cards.push(line_to_numbers(winning_numbers));
            player_cards.push(line_to_numbers(player_numbers));

            (winning_cards, player_cards)
        },
    )
}

fn line_to_numbers(line: &str) -> Vec<u8> {
    line.split_whitespace()
        .filter_map(|n| n.parse().ok())
        .collect()
}

fn total_points(winning: &[Vec<u8>], player: &[Vec<u8>]) -> u32 {
    player
        .iter()
        .zip(winning.iter())
        .map(|(player_card, winning_card)| {
            player_card.iter().fold(0, |points, number| {
                if winning_card.contains(number) {
                    if points == 0 {
                        1
                    } else {
                        points * 2
                    }
                } else {
                    points
                }
            })
        })
        .sum()
}

fn total_cards(winning: &[Vec<u8>], player: &[Vec<u8>]) -> u32 {
    let mut total_cards = 0;
    let mut indices_to_process: Vec<usize> = (0..player.len()).collect();

    while let Some(i) = indices_to_process.pop() {
        let player_card = &player[i];
        let matches = winning[i]
            .iter()
            .filter(|&winning_number| player_card.contains(winning_number))
            .count();

        total_cards += 1;

        if matches > 0 && i + matches < winning.len() {
            indices_to_process.extend(i + 1..=i + matches);
        }
    }

    total_cards
}

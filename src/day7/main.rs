use anyhow::anyhow;
use itertools::Itertools;
use std::{collections::btree_map::Values, io::BufRead};

const INPUT: &[u8] = include_bytes!("input.txt");

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
    }
}
#[derive(Eq, PartialEq, PartialOrd, Ord, Debug, Copy, Clone)]
struct Card(u8);

impl From<u8> for Card {
    fn from(val: u8) -> Self {
        Card(val)
    }
}

impl From<char> for Card {
    fn from(val: char) -> Self {
        char_to_value(val)
    }
}

fn char_to_value(c: char) -> Card {
    let num = match c {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        _ => c.to_digit(10).unwrap() as u8,
    };
    num.into()
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
struct NOfAKind {
    n: u8,
    value: Card,
}

#[derive(Eq, PartialEq, Debug)]
struct Hand {
    cards: Vec<NOfAKind>,
}

impl Hand {
    fn n_score(&self) -> u8 {
        self.cards.iter().map(|c| c.n * c.n).sum()
    }

    fn cards_as_vec(&self) -> Vec<Card> {
        let mut card_vec = Vec::new();
        for NOfAKind { n, value } in self.cards.iter() {
            for _ in 0..*n {
                card_vec.push(*value);
            }
        }
        card_vec
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.n_score().cmp(&other.n_score()) {
            std::cmp::Ordering::Equal => {
                for (self_card, other_card) in
                    self.cards_as_vec().iter().zip(other.cards_as_vec().iter())
                {
                    match self_card.0.cmp(&other_card.0) {
                        std::cmp::Ordering::Equal => continue,
                        other => return other,
                    }
                }
                std::cmp::Ordering::Equal
            }
            other => other,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Ord, Debug, Eq, PartialEq, PartialOrd)]
struct Play {
    hand: Hand,
    bid: i64,
}

impl Play {
    fn new(hand: Hand, bid: i64) -> Self {
        Self { hand, bid }
    }
    fn from_strs(cards: &str, bid: &str) -> Self {
        let cards: Vec<Card> = cards.chars().map(|c| c.into()).collect();
        let mut counts = [0u8; 15];
        for i in cards {
            counts[i.0 as usize] += 1;
        }
        let hand = Hand {
            cards: counts
                .iter()
                .enumerate()
                .filter_map(|(value, n)| {
                    if *n == 0 {
                        None
                    } else {
                        Some(NOfAKind {
                            n: *n,
                            value: Card(value as u8),
                        })
                    }
                })
                .sorted_unstable() // Don't need to worry about order of equal elements
                .rev() // Reverse to get highest first
                .collect(),
        };
        Self::new(hand, bid.parse().unwrap())
    }

    fn from_line(line: &str) -> anyhow::Result<Self> {
        let mut parts = line.split_whitespace();
        let cards = parts.next().unwrap();
        let bid = parts.next().unwrap();
        if let Some(unexpected) = parts.next() {
            return Err(anyhow!(
                "Invalid input, read cards: {}, bid: {}, then found unexpected: {}",
                cards,
                bid,
                unexpected
            ));
        }
        let play = Self::from_strs(cards, bid);
        Ok(play)
    }
}

// Idea: hand represented as tuples (Repeats, CardNumber) for example (3, 5), (2, 3) for three 5s + two 3s. Hand tuples are then sorted by first then 2nd entry.
// Comparison is then done by comparing the tuples in order.
pub fn run() -> anyhow::Result<i64> {
    let reader = INPUT;
    let plays = get_plays_from_input(reader);
    // let mut winnings = 0;
    // for (play, i) in plays.iter().zip(0i64..) {
    //     let score = (i + 1) * play.bid;
    //     let ns = play.hand.cards.iter().map(|n| n.n).collect::<Vec<_>>();
    //     let vs = play
    //         .hand
    //         .cards
    //         .iter()
    //         .map(|n| n.value.0)
    //         .collect::<Vec<_>>();
    //     winnings += score;
    //     println!(
    //         "Rank {}: {:?}  {:?} with bid {} has score {} (total: {})",
    //         i + 1,
    //         ns,
    //         vs,
    //         play.bid,
    //         score,
    //         winnings
    //     );
    // }
    // println!("Total winnings: {}", winnings);

    let winnings = get_winnings_from_plays(plays);
    println!("Total winnings: {}", winnings);
    Ok(winnings)
}

fn get_winnings_from_plays(plays: Vec<Play>) -> i64 {
    let mut winnings = 0;
    for (play, i) in plays.iter().zip(0i64..) {
        let score = (i + 1) * play.bid;
        let ns = play.hand.cards.iter().map(|n| n.n).collect::<Vec<_>>();
        let vs = play
            .hand
            .cards
            .iter()
            .map(|n| n.value.0)
            .collect::<Vec<_>>();
        winnings += score;
        println!(
            "Rank {}: {:?}  {:?} with bid {} has score {} (total: {})",
            i + 1,
            ns,
            vs,
            play.bid,
            score,
            winnings
        );
    }
    println!("Total winnings: {}", winnings);

    plays
        .into_iter()
        .zip(1i64..)
        .map(|(play, rank)| (rank) * play.bid)
        .sum()
}

fn get_plays_from_input(reader: &[u8]) -> Vec<Play> {
    reader
        .lines()
        .map(|line| Play::from_line(&line.unwrap()).unwrap())
        .sorted_unstable()
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = "32T3K 765\nT55J5 684\nKK677 28\nKTJJT 220\nQQQJA 483";
        let plays = get_plays_from_input(input.as_bytes());
        let winnings = get_winnings_from_plays(plays);
        assert_eq!(winnings, 6440);
    }

    #[test]
    fn test_parse_play() {
        let play = Play::from_line("32T3K 765").unwrap();
        let ns = [2, 1, 1, 1];
        let vs = [3, 13, 10, 2];
        let cards = ns
            .iter()
            .zip(vs.iter())
            .map(|(n, v)| NOfAKind {
                n: *n,
                value: Card(*v),
            })
            .collect();
        assert_eq!(play, Play::new(Hand { cards }, 765));
    }
}

// --- Day 7: Camel Cards ---
// Your all-expenses-paid trip turns out to be a one-way, five-minute ride in an airship. (At least it's a cool airship!) It drops you off at the edge of a vast desert and descends back to Island Island.

// "Did you bring the parts?"

// You turn around to see an Elf completely covered in white clothing, wearing goggles, and riding a large camel.

// "Did you bring the parts?" she asks again, louder this time. You aren't sure what parts she's looking for; you're here to figure out why the sand stopped.

// "The parts! For the sand, yes! Come with me; I will show you." She beckons you onto the camel.

// After riding a bit across the sands of Desert Island, you can see what look like very large rocks covering half of the horizon. The Elf explains that the rocks are all along the part of Desert Island that is directly above Island Island, making it hard to even get there. Normally, they use big machines to move the rocks and filter the sand, but the machines have broken down because Desert Island recently stopped receiving the parts they need to fix the machines.

// You've already assumed it'll be your job to figure out why the parts stopped when she asks if you can help. You agree automatically.

// Because the journey will take a few days, she offers to teach you the game of Camel Cards. Camel Cards is sort of similar to poker except it's designed to be easier to play while riding a camel.

// In Camel Cards, you get a list of hands, and your goal is to order them based on the strength of each hand. A hand consists of five cards labeled one of A, K, Q, J, T, 9, 8, 7, 6, 5, 4, 3, or 2. The relative strength of each card follows this order, where A is the highest and 2 is the lowest.

// Every hand is exactly one type. From strongest to weakest, they are:

// Five of a kind, where all five cards have the same label: AAAAA
// Four of a kind, where four cards have the same label and one card has a different label: AA8AA
// Full house, where three cards have the same label, and the remaining two cards share a different label: 23332
// Three of a kind, where three cards have the same label, and the remaining two cards are each different from any other card in the hand: TTT98
// Two pair, where two cards share one label, two other cards share a second label, and the remaining card has a third label: 23432
// One pair, where two cards share one label, and the other three cards have a different label from the pair and each other: A23A4
// High card, where all cards' labels are distinct: 23456
// Hands are primarily ordered based on type; for example, every full house is stronger than any three of a kind.

// If two hands have the same type, a second ordering rule takes effect. Start by comparing the first card in each hand. If these cards are different, the hand with the stronger first card is considered stronger. If the first card in each hand have the same label, however, then move on to considering the second card in each hand. If they differ, the hand with the higher second card wins; otherwise, continue with the third card in each hand, then the fourth, then the fifth.

// So, 33332 and 2AAAA are both four of a kind hands, but 33332 is stronger because its first card is stronger. Similarly, 77888 and 77788 are both a full house, but 77888 is stronger because its third card is stronger (and both hands have the same first and second card).

// To play Camel Cards, you are given a list of hands and their corresponding bid (your puzzle input). For example:

// 32T3K 765
// T55J5 684
// KK677 28
// KTJJT 220
// QQQJA 483
// This example shows five hands; each hand is followed by its bid amount. Each hand wins an amount equal to its bid multiplied by its rank, where the weakest hand gets rank 1, the second-weakest hand gets rank 2, and so on up to the strongest hand. Because there are five hands in this example, the strongest hand will have rank 5 and its bid will be multiplied by 5.

// So, the first step is to put the hands in order of strength:

// 32T3K is the only one pair and the other hands are all a stronger type, so it gets rank 1.
// KK677 and KTJJT are both two pair. Their first cards both have the same label, but the second card of KK677 is stronger (K vs T), so KTJJT gets rank 2 and KK677 gets rank 3.
// T55J5 and QQQJA are both three of a kind. QQQJA has a stronger first card, so it gets rank 5 and T55J5 gets rank 4.
// Now, you can determine the total winnings of this set of hands by adding up the result of multiplying each hand's bid with its rank (765 * 1 + 220 * 2 + 28 * 3 + 684 * 4 + 483 * 5). So the total winnings in this example are 6440.

// Find the rank of every hand in your set. What are the total winnings?

use std::collections::{VecDeque, HashSet};
use std::str::FromStr;
use std::cmp::Ordering;

#[derive(Debug, Clone)]
struct Player {
    id: String,
    deck: VecDeque<u8>,
}
impl Player {
    fn with_subdeck(&self, n: u8) -> Player {
        Player{
            id: self.id.to_string(),
            deck: self.deck.iter()
                .take(n as usize)
                .copied()
                .collect()
        }
    }
}
#[derive(Debug)]
struct ParsePlayerError;
impl FromStr for Player {
    type Err = ParsePlayerError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        Ok(Player{
            id: lines.next().unwrap().to_string(),
            deck: lines.filter_map(|line| line.parse::<u8>().ok())
                .collect()
        })
    }
}
impl PartialEq for Player {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

pub fn run(input: &str) {
    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

fn part_1(input: &str) -> u64 {
    let (mut p1, mut p2) = parse_input(input);
    let winner = combat(&mut p1, &mut p2);
    score(&winner)
}

fn part_2(input: &str) -> u64 {
    let (p1, p2) = parse_input(input);
    let winner = recursive_combat(p1, p2);
    score(&winner)
}

fn combat(p1: &mut Player, p2: &mut Player) -> Player {
    loop {
        let p1_card = match p1.deck.pop_front() {
            None => return p2.clone(),
            Some(card) => card,
        };
        let p2_card = match p2.deck.pop_front() {
            None => {
                p1.deck.push_front(p1_card);
                return p1.clone()
            },
            Some(card) => card,
        };
        match p1_card.cmp(&p2_card) {
            Ordering::Less => {
                p2.deck.push_back(p2_card);
                p2.deck.push_back(p1_card);
            },
            Ordering::Greater => {
                p1.deck.push_back(p1_card);
                p1.deck.push_back(p2_card);
            },
            Ordering::Equal => panic!("Drawed equal values!"),
        };
    }
}

fn recursive_combat(mut p1: Player, mut p2: Player) -> Player {
    let mut p1_history = HashSet::<VecDeque<u8>>::new();
    let mut p2_history = HashSet::<VecDeque<u8>>::new();
    loop {
        match p1_history.contains(&p1.deck) {
            true => return p1,
            _ => {}
        };
        p1_history.insert(p1.deck.clone());
        match p2_history.contains(&p2.deck) {
            true => return p1,
            _ => {}
        };
        p2_history.insert(p2.deck.clone());

        let p1_card = match p1.deck.pop_front() {
            None => return p2,
            Some(card) => card,
        };
        let p2_card = match p2.deck.pop_front() {
            None => {
                p1.deck.push_front(p1_card);
                return p1;
            },
            Some(card) => card,
        };

        let winner = match p1.deck.len() >= p1_card as usize && p2.deck.len() >= p2_card as usize {
            true => recursive_combat(p1.with_subdeck(p1_card), p2.with_subdeck(p2_card)),
            false => match p1_card.cmp(&p2_card) { 
                Ordering::Less => p2.clone(),
                Ordering::Greater => p1.clone(),
                Ordering::Equal => panic!("Drawed equal values!")
            }
        };
        if winner == p1 {
            p1.deck.push_back(p1_card);
            p1.deck.push_back(p2_card);
        } else {
            p2.deck.push_back(p2_card);
            p2.deck.push_back(p1_card);
        }

    }
}

fn score(deck: &Player) -> u64 {
    deck.deck.iter().rev().enumerate()
        .map(|(i, card)| (i+1) as u64 * *card as u64)
        .sum()
}

fn parse_input(input: &str) -> (Player, Player) {
    let parts = input.split("\n\n").collect::<Vec<_>>();
    (parts[0].parse::<Player>().unwrap(), parts[1].parse::<Player>().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = "Player 1:\n9\n2\n6\n3\n1\n\nPlayer 2:\n5\n8\n4\n7\n10";
        assert_eq!(part_1(input), 306);
    }

    #[test]
    fn test_part_2_small() {
        let input = "Player 1:\n43\n19\n\nPlayer 2:\n2\n29\n14";
        assert_eq!(part_2(input), 105);
    }

    #[test]
    fn test_part_2() {
        let input = "Player 1:\n9\n2\n6\n3\n1\n\nPlayer 2:\n5\n8\n4\n7\n10";
        assert_eq!(part_2(input), 291);
    }
}

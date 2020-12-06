use std::{collections};

pub fn run(input: String) {
        println!("{:?}", part_1(&input));
        println!("{:?}", part_2(&input));
}

fn part_1(input: &String) -> usize {
    input.split("\n\n")
        .map(|group| group.chars()
            .filter(|c| !c.is_whitespace())
            .collect::<collections::HashSet<char>>()
            .len()
        )
        .sum()
}

fn part_2(input: &String) -> usize {
    input.split("\n\n")
        .map(|group| group.chars()
            .collect::<collections::HashSet<char>>()
            .iter()
            .fold(0, |acc, &c| match group.matches(c).count() == group.lines().count() {
                true => acc + 1,
                false => acc
            })
        )
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn part_1_test() {
        let input = "abc\n\na\nb\nc\n\nab\nac\n\na\na\na\na\n\nb".to_owned();
        assert_eq!(part_1(&input), 11);
    }

    #[test]
    fn part_2_test() {
        let input = "abc\n\na\nb\nc\n\nab\nac\n\na\na\na\na\n\nb".to_owned();
        assert_eq!(part_2(&input), 6)
    }
}

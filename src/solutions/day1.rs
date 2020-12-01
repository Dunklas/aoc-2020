pub fn run(input: String) -> i64 {
    let input: Vec<i64> = input.split("\n")
        .into_iter()
        .map(|s| s.parse::<i64>())
        .filter_map(Result::ok)
        .collect();
    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
    
    return 0;
}

fn part_1(input: &Vec<i64>) -> i64 {
    for x in 0..input.len() {
        for y in 0..input.len() {
            if input[x] + input[y] == 2020 {
                return input[x] * input[y];
            }
        }
    }
    return -1;
}

fn part_2(input: &Vec<i64>) -> i64 {
    for x in 0..input.len() {
        for y in 0..input.len() {
            for z in 0..input.len() {
                if input[x] + input[y] + input[z] == 2020 {
                    return input[x] * input[y] * input[z];
                }
            }
        }
    }
    return -1;
}

#[cfg(test)]
mod tests {

    #[test]
    fn part_1() {
        let input: Vec<i64> = vec![1721, 979, 366, 299, 675, 1456];
        assert_eq!(super::part_1(&input), 514579);
    }

    #[test]
    fn part_2() {
        let input: Vec<i64> = vec![1721, 979, 366, 299, 675, 1456];
        assert_eq!(super::part_2(&input), 241861950);
    }
}

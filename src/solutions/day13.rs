pub fn run(input: &str) {
    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

fn part_1(input: &str) -> u64 {
    let (earliest_ts, bus_ids) = parse_input(input);
    let bus_ids: Vec<u64> = bus_ids.iter()
        .filter_map(|id| id.parse::<u64>().ok())
        .collect();
    for ts in earliest_ts.. {
        for id in &bus_ids {
            if ts % id == 0 {
                let wait = ts - earliest_ts;
                return id * wait;
            }
        }
    }
    0
}

fn part_2(input: &str) -> u64 {
    let (_ts, bus_ids) = parse_input(input);
    let bus_ids: Vec<u64> = bus_ids.iter()
        .map(|id| id.parse::<u64>().unwrap_or_default())
        .collect();
    
    let mut step = bus_ids.first().unwrap().clone();
    let mut n = 1usize;
    let mut t = 0u64;
    loop {
        if n == bus_ids.len() {
            break;
        }
        if bus_ids[n] == 0 {
            n += 1;
            continue;
        }
        if (t + n as u64) % bus_ids[n as usize] == 0 {
            step = lcm(bus_ids[n as usize], step);
            n += 1;
        } 
        t += step;
    }
    t - step
}

fn lcm(a: u64, b: u64) -> u64 {
    (a / gcd(a, b)) * b
}

fn gcd(a: u64, b: u64) -> u64 {
    if a == 0 {
        return b;
    }
    gcd(b % a, a)
}

fn parse_input(input: &str) -> (u64, Vec<&str>) {
    let mut input = input.lines();
    (input.next().unwrap().parse::<u64>().unwrap(), input.next().unwrap().split(",").collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = "939\n7,13,x,x,59,x,31,19";
        assert_eq!(part_1(input), 295);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2("000\n17,x,13,19"), 3417);
        assert_eq!(part_2("000\n67,7,59,61"), 754018);
        assert_eq!(part_2("000\n67,x,7,59,61"), 779210);
        assert_eq!(part_2("939\n7,13,x,x,59,x,31,19"), 1068781);
        assert_eq!(part_2("000\n67,7,x,59,61"), 1261476);
        assert_eq!(part_2("000\n1789,37,47,1889"), 1202161486);
    }
}
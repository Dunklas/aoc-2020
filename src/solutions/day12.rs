use regex::Regex;

pub fn run(input: &str) {
    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

fn parse_input(input: &str) -> Vec<(char, i64)> {
    let re = Regex::new(r"(\w{1})(\d+)").unwrap(); 
    re.captures_iter(input)
        .map(|cap| (cap[1].parse::<char>().unwrap(), cap[2].parse::<i64>().unwrap()))
        .collect()
}

fn part_1(input: &str) -> i64 {
    let mut x = 0i64;
    let mut y = 0i64;
    let mut rotation = 0i64;
    parse_input(input).iter()
        .for_each(|(action, value)| {
            match action {
                'N' => y += value,
                'S' => y -= value,
                'E' => x += value,
                'W' => x -= value,
                'L' => rotation -= value,
                'R' => rotation += value,
                'F' => match rotation % 360 {
                    0 => x += value,
                    90 | -270 => y -= value,
                    180 | -180 => x -= value,
                    270 | -90 => y += value,
                    _ => {},
                },
                _ => {}
            };
        });
    x.abs() + y.abs()
}

fn part_2(input: &str) -> i64 {
    let mut waypoint_x = 10i64;
    let mut waypoint_y = 1i64;
    let mut x = 0i64;
    let mut y = 0i64;
    parse_input(input).iter()
        .for_each(|(action, value)| {
            match action {
                'N' => waypoint_y += value,
                'S' => waypoint_y -= value,
                'E' => waypoint_x += value,
                'W' => waypoint_x -= value,
                'L' => match rotate_waypoint_left(waypoint_x, waypoint_y, *value) {
                    Ok((new_x, new_y)) => {
                        waypoint_x = new_x;
                        waypoint_y = new_y;
                    },
                    Err(e) => panic!(e)
                },
                'R' => match rotate_waypoint_right(waypoint_x, waypoint_y, *value) {
                    Ok((new_x, new_y)) => {
                        waypoint_x = new_x;
                        waypoint_y = new_y;
                    },
                    Err(e) => panic!(e)
                } 
                'F' => {
                    x += value * waypoint_x;
                    y += value * waypoint_y;
                },
                _ => {}
            }
        });
    x.abs() + y.abs()
}

fn rotate_waypoint_right(x: i64, y: i64, value: i64) -> Result<(i64, i64), String> {
    match value {
        90 => Ok((0 + y, 0 - x)),
        180 => Ok((0 - x, 0 - y)),
        270 => Ok((0 - y, x)),
        _ => Err(format!("Unrecognized rotation: {}", value)) 
    }
}

fn rotate_waypoint_left(x: i64, y: i64, value: i64) -> Result<(i64, i64), String> {
    match value {
        90 => Ok((0 - y, x)),
        180 => Ok((0 - x, 0 - y)),
        270 => Ok((0 + y, 0 - x)),
        _ => Err(format!("Unrecognized rotation: {}", value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test() {
        let input = "F10\nN3\nF7\nR90\nF11";
        assert_eq!(part_1(input), 25);
    }

    #[test]
    fn part_2_test() {
        let input = "F10\nN3\nF7\nR90\nF11";
        assert_eq!(part_2(input), 286);
    }

    #[test]
    fn test_rotate_waypoint_right() {
        assert_eq!(rotate_waypoint_right(10, 4, 90).unwrap(), (4, -10));
        assert_eq!(rotate_waypoint_right(4, -10, 90).unwrap(), (-10, -4));
        assert_eq!(rotate_waypoint_right(-10, -4, 90).unwrap(), (-4, 10));
        assert_eq!(rotate_waypoint_right(-4, 10, 90).unwrap(), (10, 4));
        assert_eq!(rotate_waypoint_right(10, 4, 180).unwrap(), (-10, -4));
        assert_eq!(rotate_waypoint_right(4, -10, 180).unwrap(), (-4, 10));
        assert_eq!(rotate_waypoint_right(-10, -4, 180).unwrap(), (10, 4));
        assert_eq!(rotate_waypoint_right(-4, 10, 180).unwrap(), (4, -10));
        assert_eq!(rotate_waypoint_right(10, 4, 270).unwrap(), (-4, 10));
        assert_eq!(rotate_waypoint_right(4, -10, 270).unwrap(), (10, 4));
        assert_eq!(rotate_waypoint_right(-10, -4, 270).unwrap(), (4, -10));
        assert_eq!(rotate_waypoint_right(-4, 10, 270).unwrap(), (-10, -4));
    }

    #[test]
    fn test_rotate_waypoint_left() {
        assert_eq!(rotate_waypoint_left(10, 4, 90).unwrap(), (-4, 10));
        assert_eq!(rotate_waypoint_left(-4, 10, 90).unwrap(), (-10, -4));
        assert_eq!(rotate_waypoint_left(-10, -4, 90).unwrap(), (4, -10));
        assert_eq!(rotate_waypoint_left(4, -10, 90).unwrap(), (10, 4));
        assert_eq!(rotate_waypoint_left(10, 4, 180).unwrap(), (-10, -4));
        assert_eq!(rotate_waypoint_left(-4, 10, 180).unwrap(), (4, -10));
        assert_eq!(rotate_waypoint_left(-10, -4, 180).unwrap(), (10, 4));
        assert_eq!(rotate_waypoint_left(4, -10, 180).unwrap(), (-4, 10));
        assert_eq!(rotate_waypoint_left(10, 4, 270).unwrap(), (4, -10));
        assert_eq!(rotate_waypoint_left(-4, 10, 270).unwrap(), (10, 4));
        assert_eq!(rotate_waypoint_left(-10, -4, 270).unwrap(), (-4, 10));
        assert_eq!(rotate_waypoint_left(4, -10, 270).unwrap(), (-10, -4));
    }
}
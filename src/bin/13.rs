advent_of_code::solution!(13);
use itertools::Itertools;

fn parse_line(line: &str) -> (i64, i64) {
    line
        .replace("Button", "")
        .replace("Prize:", "")
        .replace("A: X+", "")
        .replace("B: X", "")
        .replace("Y+", "")
        .replace("X=", "")
        .replace("Y=", "")
        .split(",")
        .map(|c| c.trim().parse::<i64>().unwrap())
        .collect_tuple::<(i64, i64)>()
        .unwrap()
}

fn get_tokens(offset: i64, lines: &str) -> i64 {
    let lines: Vec<_> = lines.lines().collect();
    let mut tokens = 0;
    for i in (0..lines.len()).step_by(4) {
        let (ax, ay) = parse_line(lines[i]);
        let (bx, by) = parse_line(lines[i+1]);
        let (mut px, mut py) = parse_line(lines[i+2]);
        (px, py) = (px+offset, py+offset);
        if (ay*px-ax*py) % (ay*bx-ax*by) == 0 {
            let nb = (ay*px-ax*py) / (ay*bx-ax*by);
            if (px - bx * nb) % ax == 0 {
                let na = (px - bx*nb) / ax;
                tokens += 3 * na + nb;
            }
        }

    }
    tokens
}
pub fn part_one(input: &str) -> Option<i64> {
    Some(get_tokens(0, input))
}

pub fn part_two(input: &str) -> Option<i64> {
    Some(get_tokens(10000000000000, input))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(875318608908));
    }
}


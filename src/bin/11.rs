advent_of_code::solution!(11);
use memoize::memoize;

fn parse_input(input: &str) -> Vec<u64> {
    input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

// Memoization here does the heck of a job...
#[memoize]
fn blink(stone: u64, n_blinks: u32) -> u64 {
    if n_blinks == 0 {
        return 1;
    }
    if stone == 0 {
        return blink(1, n_blinks - 1);
    }

    let digits = stone.ilog10() as u32 + 1;
    if digits % 2 == 0 {
        let left = stone / 10_u64.pow(digits / 2);
        let right = stone % 10_u64.pow(digits / 2);
        return blink(left, n_blinks - 1) + blink(right, n_blinks - 1);
    }

    return blink(stone * 2024, n_blinks - 1);
}

fn blinks(input: &str, times: u32) -> u64 {
    let stones = parse_input(input);
    stones.iter().map(|&stone| blink(stone, times)).sum()
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(blinks(input, 25))
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(blinks(input, 75))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result =
            part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result =
            part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(65601038650482));
    }
}

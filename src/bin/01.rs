advent_of_code::solution!(1);

use advent_of_code::transpose;

pub fn parse_input(input: &str) -> Vec<Vec<u32>> {
    transpose(
        input
            .lines()
            .map(|line| {
                line.split_whitespace()
                    .map(|s| s.parse::<u32>().unwrap())
                    .collect()
            })
            .collect(),
    )
}

pub fn part_one(input: &str) -> Option<u32> {
    let parsed = parse_input(input);
    let mut l1 = parsed[0].clone();
    let mut l2 = parsed[1].clone();
    l1.sort();
    l2.sort();
    Some((0..l1.len()).map(|i| l1[i].abs_diff(l2[i])).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let parsed = parse_input(input);
    let l1 = parsed[0].clone();
    let l2 = parsed[1].clone();
    Some(
        (0..l1.len())
            .map(|i| l2.iter().filter(|&x| *x == l1[i]).count() as u32 * l1[i])
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result =
            part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result =
            part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}

advent_of_code::solution!(2);

pub fn parse_input(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse::<u32>().unwrap())
                .collect()
        })
        .collect()
}

fn is_safe(l1: &Vec<u32>) -> bool {
    let increasing =
        (0..l1.len() - 1).all(|i| l1[i] < l1[i + 1] && l1[i + 1] - l1[i] <= 3);
    let decreasing =
        (0..l1.len() - 1).all(|i| l1[i] > l1[i + 1] && l1[i] - l1[i + 1] <= 3);
    increasing || decreasing
}

fn is_safe_with_dampener(l1: &Vec<u32>) -> bool {
    (0..l1.len()).any(|i| is_safe(&[&l1[..i], &l1[i + 1..]].concat()))
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        parse_input(input)
            .iter()
            .map(|x| is_safe(&x))
            .filter(|&x| x == true)
            .count() as u32,
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        parse_input(input)
            .iter()
            .map(|x| is_safe_with_dampener(&x))
            .filter(|&x| x == true)
            .count() as u32,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result =
            part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result =
            part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}

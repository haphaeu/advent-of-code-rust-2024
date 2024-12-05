advent_of_code::solution!(5);

fn parse_input(input: &str) -> (Vec<Vec<u32>>, Vec<Vec<u32>>) {
    let mut rules = Vec::new();
    let mut updates = Vec::new();

    for line in input.lines() {
        if line.is_empty() {
            continue;
        }
        if line.contains("|") {
            rules.push(line.split("|").map(|s| s.parse().unwrap()).collect());
        } else {
            updates.push(line.split(",").map(|s| s.parse().unwrap()).collect());
        }
    }
    (rules, updates)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (rules, updates) = parse_input(input);

    Some(
        updates
            .iter()
            .filter(|update| {
                update.iter().zip(update.iter().skip(1)).all(|(&a, &b)| {
                    !rules.iter().any(|rule| rule[0] == b && rule[1] == a)
                })
            })
            .map(|update| update[update.len() / 2])
            .sum(),
    )
}

fn correct(update: &mut Vec<u32>, rules: &Vec<Vec<u32>>) -> bool {
    for i in 0..update.len() - 1 {
        if rules
            .iter()
            .any(|rule| rule[1] == update[i] && rule[0] == update[i + 1])
        {
            update.swap(i, i + 1);
            return true;
        }
    }
    false
}

pub fn part_two(input: &str) -> Option<u32> {
    let (rules, updates) = parse_input(input);

    let mut invalid = updates
        .into_iter()
        .filter(|update| {
            update.iter().zip(update.iter().skip(1)).any(|(&a, &b)| {
                rules.iter().any(|rule| rule[0] == b && rule[1] == a)
            })
        })
        .collect::<Vec<_>>();

    for update in invalid.iter_mut() {
        while correct(update, &rules) {}
    }

    Some(invalid.iter().map(|update| update[update.len() / 2]).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result =
            part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result =
            part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}

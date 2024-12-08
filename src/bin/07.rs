advent_of_code::solution!(7);

use itertools::Itertools;

fn parse_input(input: &str) -> Vec<Vec<u64>> {
    input
        .replace(":", "")
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect()
        })
        .collect()
}

fn add(a: u64, b: u64) -> u64 {
    a + b
}
fn mul(a: u64, b: u64) -> u64 {
    a * b
}
fn concat(a: u64, b: u64) -> u64 {
    a * 10_u64.pow(b.ilog10() as u32 + 1) + b
}

fn solve(mut values: Vec<u64>, mut ops: Vec<fn(u64, u64) -> u64>) -> u64 {
    let result = ops[0](values[0], values[1]);
    if values.len() == 2 {
        return result;
    }

    // replace the 2 first values with the results of their operation
    values[1] = result;
    values.remove(0);
    // remove the first operation
    ops.remove(0);

    solve(values, ops)
}

fn check_valid(values: Vec<u64>, incl_concat: bool) -> u64 {
    let result = values[0];
    let values_rest = values.into_iter().skip(1).collect::<Vec<u64>>();

    let ops: Vec<fn(u64, u64) -> u64>;
    if incl_concat {
        ops = vec![add, mul, concat];
    } else {
        ops = vec![add, mul];
    }
    for equations in (0..values_rest.len() - 1)
        .map(|_i| ops.clone())
        .multi_cartesian_product()
    {
        //println!("{:?} = {}", values_rest, result);
        if solve(values_rest.clone(), equations) == result {
            //println!("  YES");
            return result;
        }
    }
    0
}

fn part_both(input: &str, incl_concat: bool) -> u64 {
    let data = parse_input(input);
    data.iter()
        .map(|values| check_valid(values.clone(), incl_concat))
        .sum()
}
pub fn part_one(input: &str) -> Option<u64> {
    Some(part_both(input, false))
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(part_both(input, true))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result =
            part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result =
            part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}

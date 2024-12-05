advent_of_code::solution!(3);

use regex::Regex;

fn parse_mul_sum(input: &str) -> Option<u32> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    println!("  input: {}", input);
    Some(
        re.captures_iter(input)
            .map(|cap| {
                let c1 = cap[1].parse::<u32>().unwrap();
                let c2 = cap[2].parse::<u32>().unwrap();
                println!("  mul({},{})", c1, c2);
                c1 * c2
            })
            .sum(),
    )
}

fn parse_slice(re: &Regex, input: &str, i: usize) -> u32 {
    let input = input.trim().replace("\n", "");
    let input = input.as_str();

    let mut caps = re.captures_iter(input);
    match i {
        0 => parse_mul_sum(&caps.next().unwrap()[1]).unwrap(),
        2 => parse_mul_sum(&caps.last().unwrap()[1]).unwrap(),
        1 => caps
            .map(|c| c.extract())
            .map(|(_, [instructs])| parse_mul_sum(&instructs).unwrap())
            .sum(),
        _ => unreachable!(),
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    parse_mul_sum(input)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut reg_expressions = vec![
        // match the instructions before the first don't() instruction
        Regex::new(r"(.*?)don't\(\)").unwrap(),
        // match the instructions after the first do() and before the last don't()
        Regex::new(r"do\(\)(.*?)don't\(\)").unwrap(),
        // match the instructions after the last do()
        // TODO: This is wrong if the memory ends with a don't().
        //       Looking at the set, the last instruction is a don't(),
        //       so can just ignore this case.
        //Regex::new(r".*do\(\)(.*)").unwrap(),
    ];
    // little cheat to fix the last case
    if input.rfind("do()").unwrap() > input.rfind("don't()").unwrap() {
        reg_expressions.push(Regex::new(r".*do\(\)(.*)").unwrap());
    }

    Some(
        reg_expressions
            .iter()
            .enumerate()
            .map(|(i, re)| {
                println!("re: {}", re.to_string());
                parse_slice(re, input, i)
            })
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
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(48));
    }
}

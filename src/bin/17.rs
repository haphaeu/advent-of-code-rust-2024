advent_of_code::solution!(17);
use regex::Regex;

fn parse_input(input: &str) -> (Vec<u64>, u64, u64, u64) {
    let pattern = Regex::new(r"(\d+)").unwrap();
    let mut matches = pattern.find_iter(input).map(|x| x.as_str().parse().unwrap());
    let a = matches.next().unwrap();
    let b = matches.next().unwrap();
    let c = matches.next().unwrap();
    let prog = matches.collect();
    (prog, a, b, c)
}

fn get_combo_operand(operand: u64, a: u64, b: u64, c: u64) -> u64 {
    match operand {
        0..=3 => operand,
        4 => a,
        5 => b,
        6 => c,
        _ => 0,
    }
}

fn compute(program: Vec<u64>, mut a: u64, mut b: u64, mut c: u64) -> Option<Vec<u64>> {
    let size = program.len();
    let mut out: Vec<u64> = vec![];
    let mut pointer: usize = 0;

    while pointer < size {
        let opcode = program[pointer];
        let operand = program[pointer + 1];
        let combo_operand = get_combo_operand(operand, a, b, c);

        match opcode {
            0 | 6 | 7 => {
                let numerator = a;
                let denominator = 2u64.pow(combo_operand as u32);
                let res = numerator / denominator;
                match opcode {
                    0 => a = res,
                    6 => b = res,
                    7 => c = res,
                    _ => {}
                }
            }
            1 => b ^= operand,
            2 => b = combo_operand % 8,
            3 => {
                if a != 0 {
                    pointer = operand as usize;
                    continue;
                }
            }
            4 => b ^= c,
            5 => out.push(combo_operand % 8),
            _ => {}
        }
        pointer += 2;
    }
    Some(out)
}

pub fn part_one(input: &str) -> Option<String> {
    let (program, a, b, c) = parse_input(input);
    let out = compute(program, a, b, c)?;
    Some(out.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(","))
}

pub fn part_two(input: &str) -> Option<u32> {
    let (program, a, b, c) = parse_input(input);

    let out = compute(program, a, b, c)?;
    Some(out.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(","));
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("4,6,3,5,6,3,5,2,1,0".to_string()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}

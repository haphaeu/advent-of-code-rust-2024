advent_of_code::solution!(4);

fn transpose(input: &str) -> String {
    let lines: Vec<&str> = input.lines().collect();
    let num_lines = lines.len();
    let line_length = lines.len();

    let mut transposed = String::new();

    for i in 0..line_length {
        for j in 0..num_lines {
            transposed.push(lines[j].chars().nth(i).unwrap());
        }
        if i < line_length - 1 {
            transposed.push('\n');
        }
    }

    transposed
}

fn count_diagonal_matches(input: &str, word: &str) -> u32 {
    let lines: Vec<&str> = input.lines().collect();
    let num_lines = lines.len();
    let line_length = lines.len();
    let word_length = word.len();
    let mut count = 0;

    // Check diagonally from top-left to bottom-right
    for i in 0..=num_lines - word_length {
        for j in 0..=line_length - word_length {
            let mut found = true;
            for k in 0..word_length {
                if lines[i + k].chars().nth(j + k).unwrap()
                    != word.chars().nth(k).unwrap()
                {
                    found = false;
                    break;
                }
            }
            if found {
                count += 1;
            }
        }
    }

    // Check diagonally from top-right to bottom-left
    for i in 0..=num_lines - word_length {
        for j in (word_length - 1)..line_length {
            let mut found = true;
            for k in 0..word_length {
                if lines[i + k].chars().nth(j - k).unwrap()
                    != word.chars().nth(k).unwrap()
                {
                    found = false;
                    break;
                }
            }
            if found {
                count += 1;
            }
        }
    }

    count
}

fn count_word(input: &str, word: &str) -> u32 {
    input
        .lines()
        .map(|line| line.matches(word).count() as u32)
        .sum()
}

pub fn part_one(input: &str) -> Option<u32> {
    let w1 = "XMAS";
    let w2 = "SAMX";

    let transposed = transpose(input);

    Some(
        count_word(input, w1)
            + count_word(input, w2)
            + count_word(&transposed, w1)
            + count_word(&transposed, w2)
            + count_diagonal_matches(input, w1)
            + count_diagonal_matches(input, w2),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines: Vec<&str> = input.lines().collect();

    Some(
        (0..lines.len() - 2)
            .map(|i| {
                (0..lines[0].len() - 2)
                    .map(|j| {
                        let mut c = 0;
                        for (k, l, m, n, o) in [
                            ('M', 'M', 'A', 'S', 'S'),
                            ('M', 'S', 'A', 'M', 'S'),
                            ('S', 'M', 'A', 'S', 'M'),
                            ('S', 'S', 'A', 'M', 'M'),
                        ] {
                            if lines[i].chars().nth(j).unwrap() == k
                                && lines[i].chars().nth(j + 2).unwrap() == l
                                && lines[i + 1].chars().nth(j + 1).unwrap() == m
                                && lines[i + 2].chars().nth(j).unwrap() == n
                                && lines[i + 2].chars().nth(j + 2).unwrap() == o
                            {
                                c += 1;
                            }
                        }
                        c
                    })
                    .sum::<u32>()
            })
            .sum::<u32>(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result =
            part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result =
            part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}

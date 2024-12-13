advent_of_code::solution!(8);

use itertools::Itertools;
use std::collections::HashSet;

fn find_coords(frequency: char, lines: &Vec<&str>) -> Vec<(isize, isize)> {
    let mut coords = vec![];
    for (i, line) in lines.iter().enumerate() {
        match line.find(frequency) {
            Some(j) => coords.push((i as isize, j as isize)),
            None => (),
        }
    }
    coords
}

fn calc_antinodes(
    coords: Vec<(isize, isize)>,
    size: isize,
) -> HashSet<(isize, isize)> {
    let mut antinodes = HashSet::<(isize, isize)>::new();
    // Tuple combinations thanks to alice:
    // https://users.rust-lang.org/t/how-to-pattern-match-a-combination/122386
    for ((i1, j1), (i2, j2)) in coords.iter().tuple_combinations() {
        let (di, dj) = (i2 - i1, j2 - j1);
        let (an1i, an1j, an2i, an2j) = (i1 - di, j1 - dj, i2 + di, j2 + dj);
        if 0 <= an1i && an1i < size && 0 <= an1j && an1j < size {
            antinodes.insert((an1i, an1j));
        }
        if 0 <= an2i && an2i < size && 0 <= an2j && an2j < size {
            antinodes.insert((an2i, an2j));
        }
    }
    antinodes
}

fn calc_antinodes2(
    coords: Vec<(isize, isize)>,
    size: isize,
) -> HashSet<(isize, isize)> {
    let mut antinodes = HashSet::<(isize, isize)>::new();
    for ((i1, j1), (i2, j2)) in coords.iter().tuple_combinations() {
        let (di, dj) = (i2 - i1, j2 - j1);
        let mut n = 0;
        loop {
            let (mut out1, mut out2) = (false, false);
            let (an1i, an1j, an2i, an2j) =
                (i1 - n * di, j1 - n * dj, i2 + n * di, j2 + n * dj);
            if 0 <= an1i && an1i < size && 0 <= an1j && an1j < size {
                antinodes.insert((an1i, an1j));
            } else {
                out1 = true;
            }
            if 0 <= an2i && an2i < size && 0 <= an2j && an2j < size {
                antinodes.insert((an2i, an2j));
            } else {
                out2 = true;
            }
            if out1 && out2 {
                break;
            }
            n += 1;
        }
    }
    antinodes
}

fn part_both(
    input: &str,
    func_get_antinodes: fn(
        Vec<(isize, isize)>,
        isize,
    ) -> HashSet<(isize, isize)>,
) -> u32 {
    let data: Vec<&str> = input.lines().collect();
    let size = data.len() as isize;
    let frequencies =
        HashSet::<char>::from_iter(input.replace(&['.', '\n'][..], "").chars());
    let mut antinodes = HashSet::<(isize, isize)>::new();
    for f in frequencies {
        antinodes.extend(func_get_antinodes(find_coords(f, &data), size));
    }
    antinodes.len() as u32
}
pub fn part_one(input: &str) -> Option<u32> {
    Some(part_both(input, calc_antinodes))
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(part_both(input, calc_antinodes2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result =
            part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result =
            part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}

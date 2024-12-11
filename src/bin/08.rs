advent_of_code::solution!(8);

use itertools::Itertools;
use std::collections::HashSet;

fn find_coords(frequency: char, lines: &Vec<&str>) -> Vec<(usize, usize)> {
    let mut coords = vec![];
    for (i, line) in lines.iter().enumerate() {
        match line.find(frequency) {
            Some(j) => coords.push((i, j)),
            None => (),
        }
    }
    coords
}

fn calc_antinodes(
    coords: Vec<(usize, usize)>,
    size: isize,
) -> HashSet<(usize, usize)> {
    let mut antinodes = HashSet::<(usize, usize)>::new();
    for c in coords.iter().combinations(2) {
        // TODO: there has to be a batter way to do this
        let (v1, v2) = (c[0], c[1]);
        let (i1, j1) = (v1.0 as isize, v1.1 as isize);
        let (i2, j2) = (v2.0 as isize, v2.1 as isize);

        let (di, dj) = (i2 - i1, j2 - j1);
        let (an1i, an1j, an2i, an2j) = (i1 - di, j1 - dj, i2 + di, j2 + dj);
        if 0 <= an1i && an1i < size && 0 <= an1j && an1j < size {
            antinodes.insert((an1i as usize, an1j as usize));
        }
        if 0 <= an2i && an2i < size && 0 <= an2j && an2j < size {
            antinodes.insert((an2i as usize, an2j as usize));
        }
    }
    antinodes
}

fn calc_antinodes2(
    coords: Vec<(usize, usize)>,
    size: isize,
) -> HashSet<(usize, usize)> {
    let mut antinodes = HashSet::<(usize, usize)>::new();
    for c in coords.iter().combinations(2) {
        let (v1, v2) = (c[0], c[1]);
        let (i1, j1) = (v1.0 as isize, v1.1 as isize);
        let (i2, j2) = (v2.0 as isize, v2.1 as isize);

        let (di, dj) = (i2 - i1, j2 - j1);
        let mut n = 0;
        loop {
            let (mut out1, mut out2) = (false, false);
            let (an1i, an1j, an2i, an2j) =
                (i1 - n * di, j1 - n * dj, i2 + n * di, j2 + n * dj);
            if 0 <= an1i && an1i < size && 0 <= an1j && an1j < size {
                antinodes.insert((an1i as usize, an1j as usize));
            } else {
                out1 = true;
            }
            if 0 <= an2i && an2i < size && 0 <= an2j && an2j < size {
                antinodes.insert((an2i as usize, an2j as usize));
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
        Vec<(usize, usize)>,
        isize,
    ) -> HashSet<(usize, usize)>,
) -> u32 {
    let data: Vec<&str> = input.lines().collect();
    let size = data.len() as isize;
    let frequencies =
        HashSet::<char>::from_iter(input.replace(&['.', '\n'][..], "").chars());
    let mut antinodes = HashSet::<(usize, usize)>::new();
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

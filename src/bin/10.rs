advent_of_code::solution!(10);

use std::collections::HashSet;

fn parse_input(input: &str) -> (Vec<Vec<u32>>, Vec<(usize, usize)>, u32, u32) {
    let input: Vec<Vec<u32>> = input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    // get the width and height of the input
    let (height, width) = (input.len(), input[0].len());

    // find all trail heads, ie, char '0' in the map
    let mut trail_heads = vec![];
    for i in 0..height {
        for j in 0..width {
            if input[i][j] == 0 {
                trail_heads.push((i, j));
            }
        }
    }

    (input, trail_heads, height as u32, width as u32)
}

// for a given point on the map, return all the points that can be reached in one step
// Working recursively, starting from a trailhead, it will return a vec with all the possible paths to all the summits
// reached by that trailhead.
// So, given the output `summits`, a score will be a HashSet of `summits`, for part one,
// and a rating will be the length of the vec `summits`, for part two.
fn step(
    input: &Vec<Vec<u32>>,
    elevation: u32,
    coord: (usize, usize),
    height: u32,
    width: u32,
) -> Option<Vec<(usize, usize)>> {
    if elevation == 9 {
        return Some(vec![coord]);
    }
    let next_elevation = elevation + 1;
    let mut next_coords = vec![];
    let deltas: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    for (di, dj) in deltas {
        let i: isize = coord.0 as isize + di;
        let j: isize = coord.1 as isize + dj;
        if i >= 0
            && i < height as isize
            && j >= 0
            && j < width as isize
            && input[i as usize][j as usize] == next_elevation
        {
            next_coords.push((i as usize, j as usize));
        }
    }
    if next_coords.is_empty() {
        return None;
    }
    Some(
        next_coords
            .iter()
            .map(|coord| step(input, next_elevation, *coord, height, width))
            .filter(|coord| coord.is_some())
            .map(|coord| coord.unwrap())
            .flatten()
            .collect(),
    )
}
pub fn part_one(input: &str) -> Option<u32> {
    let (input, trail_heads, height, width) = parse_input(input);
    let mut score = 0;
    for trail_head in trail_heads {
        let summits = HashSet::<(usize, usize)>::from_iter(
            step(
                &input,
                input[trail_head.0][trail_head.1],
                trail_head,
                height,
                width,
            )
            .unwrap(),
        );
        score += summits.len() as u32;
    }
    Some(score)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (input, trail_heads, height, width) = parse_input(input);
    let mut rating = 0;
    for trail_head in trail_heads {
        let summits = step(
            &input,
            input[trail_head.0][trail_head.1],
            trail_head,
            height,
            width,
        )
        .unwrap();
        rating += summits.len() as u32;
    }
    Some(rating)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result =
            part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result =
            part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}

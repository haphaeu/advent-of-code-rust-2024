advent_of_code::solution!(6);
use std::collections::HashSet;

fn parse_input(input: &str) -> (Vec<Vec<char>>, i32, i32, i32, i32) {
    let input: Vec<Vec<char>> =
        input.lines().map(|line| line.chars().collect()).collect();

    // get the width and height of the input
    let (height, width) = (input.len(), input[0].len());

    // find the char ´^´ in input:
    let (i, j) = input
        .iter()
        .enumerate()
        .find_map(|(i, row)| match row.iter().position(|&c| c == '^') {
            Some(j) => Some((i, j)),
            None => None,
        })
        .unwrap();

    (input, height as i32, width as i32, i as i32, j as i32)
}

fn move_guard(i: i32, j: i32, moving: char) -> (i32, i32) {
    match moving {
        '^' => (i - 1, j),
        '>' => (i, j + 1),
        'v' => (i + 1, j),
        '<' => (i, j - 1),
        _ => unreachable!(),
    }
}

fn turn(moving: char) -> char {
    match moving {
        '^' => '>',
        '>' => 'v',
        'v' => '<',
        '<' => '^',
        _ => unreachable!(),
    }
}

fn get_path(
    input: Vec<Vec<char>>,
    height: i32,
    width: i32,
    mut i: i32,
    mut j: i32,
    mut moving: char,
) -> Option<Vec<(i32, i32, char)>> {
    //println!("Starting at {} {}", i, j);
    let mut path = vec![(i, j, moving)];
    let mut path_set = HashSet::new();
    path_set.insert((i, j, moving));
    let mut turns = 0;
    loop {
        let (i_try, j_try) = move_guard(i, j, moving);
        //println!("Trying to move to {} {}", i_try, j_try);
        if i_try < 0 || i_try >= width || j_try < 0 || j_try >= height {
            //println!("Out of bounds");
            break;
        }
        if input[i_try as usize][j_try as usize] == '#' {
            //println!("Blocked");
            moving = turn(moving);
            //println!("Turning to {}", moving);
            turns += 1;
            if turns == 3 {
                // Deadlock
                //println!("Deadlock, spinning around");
                return None;
            }
        } else {
            turns = 0;
            (i, j) = (i_try, j_try);
            if path_set.contains(&(i, j, moving)) {
                // Deadlock
                //println!("Deadlock, backtracking");
                return None;
            }
            path.push((i, j, moving));
            path_set.insert((i, j, moving));
            //println!("Moved to {} {}", i, j);
        }
    }
    Some(path)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (input, height, width, i, j) = parse_input(input);
    let path = get_path(input, height, width, i, j, '^');
    // remove duplicates from path
    Some(
        path.unwrap()
            .into_iter()
            .map(|(i, j, _)| (i, j))
            .collect::<HashSet<_>>()
            .len() as u32,
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let (input, height, width, i, j) = parse_input(input);

    // get the path without the first element
    let path = get_path(input.clone(), height, width, i, j, '^')
        .unwrap()
        .into_iter()
        .skip(1)
        .collect::<Vec<_>>();

    let mut blocks = vec![];
    path.into_iter().for_each(|(i0, j0, _moving)| {
        let mut input0 = input.clone();
        input0[i0 as usize][j0 as usize] = '#';
        //println!("\n#####\nAdding a block {} {} to check for deadlock", i0, j0);
        if get_path(input0, height, width, i, j, '^').is_none() {
            blocks.push((i0, j0));
        }
    });
    Some(
        blocks
            .into_iter()
            //.inspect(|(i, j)| println!("{} {}", i, j))
            .collect::<HashSet<_>>()
            .len() as u32,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result =
            part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result =
            part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}

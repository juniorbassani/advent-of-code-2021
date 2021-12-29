use once_cell::sync::Lazy;

use crate::utils;

static INPUT: Lazy<Vec<Vec<u8>>> =
    Lazy::new(|| utils::get_input_as_matrix(INPUT_PATH, std::convert::identity));
const INPUT_PATH: &str = "input/day25";

type Location = (usize, usize);

fn steps_until_no_movement(mut cucumbers: Vec<Vec<u8>>) -> usize {
    let mut container = Vec::with_capacity(128);
    let mut moved;
    let mut count = 0;

    loop {
        move_direction(&cucumbers, b'>', &mut container);
        update(&mut cucumbers, &container);
        moved = !container.is_empty();
        container.clear();
        move_direction(&cucumbers, b'v', &mut container);
        update(&mut cucumbers, &container);
        moved = moved || !container.is_empty();
        container.clear();

        count += 1;
        if !moved {
            break;
        }
    }

    count
}

fn update(cucumbers: &mut Vec<Vec<u8>>, container: &[(Location, Location)]) {
    for &((i, j), (new_i, new_j)) in container {
        cucumbers[new_i][new_j] = cucumbers[i][j];
        cucumbers[i][j] = b'.';
    }
}

fn move_direction(cucumbers: &[Vec<u8>], direction: u8, container: &mut Vec<(Location, Location)>) {
    for i in 0..cucumbers.len() {
        for j in 0..cucumbers[0].len() {
            if cucumbers[i][j] == direction {
                match direction {
                    b'>' => {
                        let new_location = (i, (j + 1) % cucumbers[0].len());
                        if cucumbers[i][new_location.1] == b'.' {
                            container.push(((i, j), new_location));
                        }
                    }
                    b'v' => {
                        let new_location = ((i + 1) % cucumbers.len(), j);
                        if cucumbers[new_location.0][j] == b'.' {
                            container.push(((i, j), new_location));
                        }
                    }
                    _ => unreachable!(),
                }
            }
        }
    }
}

pub fn part1() -> usize {
    steps_until_no_movement(INPUT.clone())
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: Lazy<Vec<Vec<u8>>> = Lazy::new(|| {
        vec![
            vec![b'v', b'.', b'.', b'.', b'>', b'>', b'.', b'v', b'v', b'>'],
            vec![b'.', b'v', b'v', b'>', b'>', b'.', b'v', b'v', b'.', b'.'],
            vec![b'>', b'>', b'.', b'>', b'v', b'>', b'.', b'.', b'.', b'v'],
            vec![b'>', b'>', b'v', b'>', b'>', b'.', b'>', b'.', b'v', b'.'],
            vec![b'v', b'>', b'v', b'.', b'v', b'v', b'.', b'v', b'.', b'.'],
            vec![b'>', b'.', b'>', b'>', b'.', b'.', b'v', b'.', b'.', b'.'],
            vec![b'.', b'v', b'v', b'.', b'.', b'>', b'.', b'>', b'v', b'.'],
            vec![b'v', b'.', b'v', b'.', b'.', b'>', b'>', b'v', b'.', b'v'],
            vec![b'.', b'.', b'.', b'.', b'v', b'.', b'.', b'v', b'.', b'>'],
        ]
    });

    #[test]
    fn part1_example() {
        assert_eq!(steps_until_no_movement(INPUT.clone()), 58);
    }
}

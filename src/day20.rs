use std::collections::VecDeque;

use once_cell::sync::Lazy;

use crate::utils;

static INPUT: Lazy<([u8; 512], Vec<Vec<u8>>)> = Lazy::new(|| {
    let res = utils::split_map(INPUT_PATH, "\n\n", std::convert::identity, |row| {
        row.bytes().collect()
    });
    (res.0.try_into().unwrap(), res.1)
});
const INPUT_PATH: &str = "input/day20";

struct Image {
    image: Vec<Vec<u8>>,
}

struct Margin {
    top: usize,
    bottom: usize,
    left: usize,
    right: usize,
}

impl Image {
    fn enhance(original: &[&[u8]], enhancement: [u8; 512], scale: usize) -> Self {
        let original_width = original[0].len();
        let original_height = original.len();
        let padding = scale * 2 + 1;
        let width = original[0].len() + padding + 1;
        let height = original.len() + padding + 1;
        let mut image = vec![vec![0u8; width]; height];
        let enhancement = enhancement.map(|pixel| if pixel == b'#' { 1 } else { 0 });
        let mut to_update = VecDeque::with_capacity(width * 2);

        for i in 0..original_height {
            for j in 0..original_width {
                if original[i][j] == b'#' {
                    image[i + scale + 1][j + scale + 1] = 1;
                }
            }
        }

        for s in 0..scale {
            let margin = Margin {
                top: scale - s,
                bottom: original_height + scale + s + 1,
                left: scale - s,
                right: original_width + scale + s + 1,
            };

            for i in scale - s..=original_height + scale + s + 1 {
                for j in scale - s..=original_width + scale + s + 1 {
                    let enhanced = Self::enhanced_pixel(&image, (i, j), &enhancement, &margin, s);

                    to_update.push_back((enhanced, (i, j)));

                    // Try to update outstanding pixels.
                    loop {
                        if let Some(&(_, pos)) = to_update.front() {
                            if Self::can_update((i, j), pos) {
                                let (pixel, (i, j)) = to_update.pop_front().unwrap();
                                image[i][j] = pixel;
                            } else {
                                break;
                            }
                        }
                    }
                }
            }

            // Update outstanding pixels.
            while let Some((pixel, (i, j))) = to_update.pop_front() {
                image[i][j] = pixel;
            }
        }

        Self { image }
    }

    fn enhanced_pixel(
        image: &[Vec<u8>],
        origin: (usize, usize),
        enhancement: &[u8; 512],
        margin: &Margin,
        round: usize,
    ) -> u8 {
        let (i, j) = origin;
        let mut pixel_index = 0;
        let mut bit_idx = 8;

        #[allow(clippy::needless_range_loop)]
        for x in i - 1..=i + 1 {
            for y in j - 1..=j + 1 {
                let neighbor = if x <= margin.top
                    || x >= margin.bottom
                    || y <= margin.left
                    || y >= margin.right
                {
                    Self::margin(enhancement, round)
                } else {
                    image[x][y]
                };
                let neighbor = neighbor as u16;
                debug_assert!(neighbor == 0 || neighbor == 1);
                pixel_index |= neighbor << bit_idx;
                bit_idx -= 1;
            }
        }

        enhancement[pixel_index as usize]
    }

    fn margin(enhancement: &[u8; 512], round: usize) -> u8 {
        match enhancement[0] {
            0 => 0,
            1 => {
                if round == 0 {
                    0
                } else if round % 2 != 0 {
                    enhancement[0]
                } else {
                    enhancement[enhancement.len() - 1]
                }
            }
            _ => unreachable!(),
        }
    }

    fn can_update(curr_pos: (usize, usize), to_update_pos: (usize, usize)) -> bool {
        curr_pos.0 > to_update_pos.0 + 1
    }

    fn lit_pixels(&self) -> usize {
        self.image
            .iter()
            .flatten()
            .filter(|&&pixel| pixel == 1)
            .count()
    }
}

pub fn part1() -> usize {
    let (enhancement, image) = &*INPUT;
    let image: Vec<&[u8]> = image.iter().map(|e| e.as_ref()).collect();
    Image::enhance(&image, *enhancement, 2).lit_pixels()
}

pub fn part2() -> usize {
    let (enhancement, image) = &*INPUT;
    let image: Vec<&[u8]> = image.iter().map(|e| e.as_ref()).collect();
    Image::enhance(&image, *enhancement, 50).lit_pixels()
}

#[cfg(test)]
mod tests {
    use super::*;

    const IMAGE: &[&[u8]] = &[
        &[b'#', b'.', b'.', b'#', b'.'],
        &[b'#', b'.', b'.', b'.', b'.'],
        &[b'#', b'#', b'.', b'.', b'#'],
        &[b'.', b'.', b'#', b'.', b'.'],
        &[b'.', b'.', b'#', b'#', b'#'],
    ];
    const ENHANCEMENT: [u8; 512] = [
        b'.', b'.', b'#', b'.', b'#', b'.', b'.', b'#', b'#', b'#', b'#', b'#', b'.', b'#', b'.',
        b'#', b'.', b'#', b'.', b'#', b'#', b'#', b'.', b'#', b'#', b'.', b'.', b'.', b'.', b'.',
        b'#', b'#', b'#', b'.', b'#', b'#', b'.', b'#', b'.', b'.', b'#', b'#', b'#', b'.', b'#',
        b'#', b'#', b'#', b'.', b'.', b'#', b'#', b'#', b'#', b'#', b'.', b'.', b'#', b'.', b'.',
        b'.', b'.', b'#', b'.', b'.', b'#', b'.', b'.', b'#', b'#', b'.', b'.', b'#', b'#', b'#',
        b'.', b'.', b'#', b'#', b'#', b'#', b'#', b'#', b'.', b'#', b'#', b'#', b'.', b'.', b'.',
        b'#', b'#', b'#', b'#', b'.', b'.', b'#', b'.', b'.', b'#', b'#', b'#', b'#', b'#', b'.',
        b'.', b'#', b'#', b'.', b'.', b'#', b'.', b'#', b'#', b'#', b'#', b'#', b'.', b'.', b'.',
        b'#', b'#', b'.', b'#', b'.', b'#', b'.', b'.', b'#', b'.', b'#', b'#', b'.', b'.', b'#',
        b'.', b'#', b'.', b'.', b'.', b'.', b'.', b'.', b'#', b'.', b'#', b'#', b'#', b'.', b'#',
        b'#', b'#', b'#', b'#', b'#', b'.', b'#', b'#', b'#', b'.', b'#', b'#', b'#', b'#', b'.',
        b'.', b'.', b'#', b'.', b'#', b'#', b'.', b'#', b'#', b'.', b'.', b'#', b'.', b'.', b'#',
        b'.', b'.', b'#', b'#', b'#', b'#', b'#', b'.', b'.', b'.', b'.', b'.', b'#', b'.', b'#',
        b'.', b'.', b'.', b'.', b'#', b'#', b'#', b'.', b'.', b'#', b'.', b'#', b'#', b'.', b'.',
        b'.', b'.', b'.', b'.', b'#', b'.', b'.', b'.', b'.', b'.', b'#', b'.', b'.', b'#', b'.',
        b'.', b'#', b'.', b'.', b'#', b'#', b'.', b'.', b'#', b'.', b'.', b'.', b'#', b'#', b'.',
        b'#', b'#', b'#', b'#', b'#', b'#', b'.', b'#', b'#', b'#', b'#', b'.', b'#', b'#', b'#',
        b'#', b'.', b'#', b'.', b'#', b'.', b'.', b'.', b'#', b'.', b'.', b'.', b'.', b'.', b'.',
        b'.', b'#', b'.', b'.', b'#', b'.', b'#', b'.', b'#', b'.', b'.', b'.', b'#', b'#', b'#',
        b'#', b'.', b'#', b'#', b'.', b'#', b'.', b'.', b'.', b'.', b'.', b'.', b'#', b'.', b'.',
        b'#', b'.', b'.', b'.', b'#', b'#', b'.', b'#', b'.', b'#', b'#', b'.', b'.', b'#', b'.',
        b'.', b'.', b'#', b'#', b'.', b'#', b'.', b'#', b'#', b'.', b'.', b'#', b'#', b'#', b'.',
        b'#', b'.', b'.', b'.', b'.', b'.', b'.', b'#', b'.', b'#', b'.', b'.', b'.', b'.', b'.',
        b'.', b'.', b'#', b'.', b'#', b'.', b'#', b'.', b'#', b'#', b'#', b'#', b'.', b'#', b'#',
        b'#', b'.', b'#', b'#', b'.', b'.', b'.', b'#', b'.', b'.', b'.', b'.', b'.', b'#', b'#',
        b'#', b'#', b'.', b'#', b'.', b'.', b'#', b'.', b'.', b'#', b'.', b'#', b'#', b'.', b'#',
        b'.', b'.', b'.', b'.', b'#', b'#', b'.', b'.', b'#', b'.', b'#', b'#', b'#', b'#', b'.',
        b'.', b'.', b'.', b'#', b'#', b'.', b'.', b'.', b'#', b'#', b'.', b'.', b'#', b'.', b'.',
        b'.', b'#', b'.', b'.', b'.', b'.', b'.', b'.', b'#', b'.', b'#', b'.', b'.', b'.', b'.',
        b'.', b'.', b'.', b'#', b'.', b'.', b'.', b'.', b'.', b'.', b'.', b'#', b'#', b'.', b'.',
        b'#', b'#', b'#', b'#', b'.', b'.', b'#', b'.', b'.', b'.', b'#', b'.', b'#', b'.', b'#',
        b'.', b'.', b'.', b'#', b'#', b'.', b'.', b'#', b'.', b'#', b'.', b'.', b'#', b'#', b'#',
        b'.', b'.', b'#', b'#', b'#', b'#', b'#', b'.', b'.', b'.', b'.', b'.', b'.', b'.', b'.',
        b'#', b'.', b'.', b'#', b'#', b'#', b'#', b'.', b'.', b'.', b'.', b'.', b'.', b'#', b'.',
        b'.', b'#',
    ];

    #[test]
    fn part1_example() {
        assert_eq!(Image::enhance(IMAGE, ENHANCEMENT, 2).lit_pixels(), 35);
    }

    #[test]
    fn part2_example() {
        assert_eq!(Image::enhance(IMAGE, ENHANCEMENT, 50).lit_pixels(), 3351);
    }
}

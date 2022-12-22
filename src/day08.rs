/* Copyright (C) 2022 Casper Meijn <casper@meijn.net>
 * SPDX-License-Identifier: GPL-3.0-or-later
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

use array2d::Array2D;
use std::usize;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Map {
    heights: Array2D<u8>,
}

pub fn parse(text: &str) -> Map {
    let heights: Vec<Vec<u8>> = text
        .lines()
        .map(|line| line.chars().map(|char| char as u8 - b'0').collect())
        .collect();
    Map {
        heights: Array2D::from_rows(&heights).unwrap(),
    }
}

fn scenic_score_iter(tree_height: u8, iter: impl Iterator<Item = u8>) -> usize {
    let mut score = 0;
    for h in iter {
        if h < tree_height {
            score += 1;
        } else {
            return score + 1;
        }
    }
    score
}

impl Map {
    fn is_edge(&self, x: usize, y: usize) -> bool {
        x == 0 || y == 0 || x == self.heights.num_rows() - 1 || y == self.heights.num_columns() - 1
    }

    fn top_iter(&self, x: usize, y: usize) -> impl Iterator<Item = &u8> {
        let skip = self.heights.column_len() - x;
        self.heights.column_iter(y).unwrap().rev().skip(skip)
    }
    fn bottom_iter(&self, x: usize, y: usize) -> impl Iterator<Item = &u8> {
        self.heights.column_iter(y).unwrap().skip(x + 1)
    }
    fn left_iter(&self, x: usize, y: usize) -> impl Iterator<Item = &u8> {
        let skip = self.heights.row_len() - y;
        self.heights.row_iter(x).unwrap().rev().skip(skip)
    }
    fn right_iter(&self, x: usize, y: usize) -> impl Iterator<Item = &u8> {
        self.heights.row_iter(x).unwrap().skip(y + 1)
    }

    fn visibility(&self) -> Array2D<bool> {
        let mut visibility =
            Array2D::filled_with(false, self.heights.num_rows(), self.heights.num_columns());

        for x in 0..visibility.num_rows() {
            for y in 0..visibility.num_columns() {
                if self.is_edge(x, y) {
                    visibility.set(x, y, true).unwrap();
                } else {
                    let tree_height = self.heights.get(x, y).unwrap();
                    let mut top = self.top_iter(x, y);
                    let mut bottom = self.bottom_iter(x, y);
                    let mut left = self.left_iter(x, y);
                    let mut right = self.right_iter(x, y);

                    let visible = top.all(|height| height < tree_height)
                        || bottom.all(|height| height < tree_height)
                        || left.all(|height| height < tree_height)
                        || right.all(|height| height < tree_height);
                    visibility.set(x, y, visible).unwrap();
                }
            }
        }

        visibility
    }

    fn scenic_score(&self) -> Array2D<usize> {
        let mut scenic_score =
            Array2D::filled_with(0, self.heights.num_rows(), self.heights.num_columns());

        for x in 0..scenic_score.num_rows() {
            for y in 0..scenic_score.num_columns() {
                let tree_height = self.heights.get(x, y).unwrap();
                let top = scenic_score_iter(*tree_height, self.top_iter(x, y).copied());
                let bottom = scenic_score_iter(*tree_height, self.bottom_iter(x, y).copied());
                let left = scenic_score_iter(*tree_height, self.left_iter(x, y).copied());
                let right = scenic_score_iter(*tree_height, self.right_iter(x, y).copied());

                scenic_score.set(x, y, top * bottom * left * right).unwrap();
            }
        }

        scenic_score
    }
}

pub fn challange1(map: &Map) -> usize {
    map.visibility()
        .elements_row_major_iter()
        .filter(|&&visible| visible)
        .count()
}

pub fn challange2(map: &Map) -> usize {
    map.scenic_score()
        .elements_row_major_iter()
        .copied()
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_TEXT: &str = "\
30373
25512
65332
33549
35390";

    #[test]
    fn parse_example() {
        let data = parse(EXAMPLE_TEXT);
        assert_eq!(
            data.heights.as_rows(),
            vec![
                vec![3, 0, 3, 7, 3],
                vec![2, 5, 5, 1, 2],
                vec![6, 5, 3, 3, 2],
                vec![3, 3, 5, 4, 9],
                vec![3, 5, 3, 9, 0],
            ]
        );
    }

    #[test]
    fn iters_example() {
        let data = parse(EXAMPLE_TEXT);
        assert_eq!(data.top_iter(1, 2).copied().collect::<Vec<_>>(), vec![3]);

        assert_eq!(
            data.bottom_iter(1, 2).copied().collect::<Vec<_>>(),
            vec![3, 5, 3]
        );

        assert_eq!(
            data.left_iter(1, 2).copied().collect::<Vec<_>>(),
            vec![5, 2]
        );
        assert_eq!(
            data.right_iter(1, 2).copied().collect::<Vec<_>>(),
            vec![1, 2]
        );
    }

    #[test]
    fn scenic_score_iter_example() {
        assert_eq!(scenic_score_iter(5, [3].iter().copied()), 1);
        assert_eq!(scenic_score_iter(5, [5, 2].iter().copied()), 1);
        assert_eq!(scenic_score_iter(5, [1, 2].iter().copied()), 2);
        assert_eq!(scenic_score_iter(5, [3, 5, 3].iter().copied()), 2);

        assert_eq!(scenic_score_iter(5, [3, 5, 3].iter().copied()), 2);
        assert_eq!(scenic_score_iter(5, [3, 3].iter().copied()), 2);
        assert_eq!(scenic_score_iter(5, [3].iter().copied()), 1);
        assert_eq!(scenic_score_iter(5, [4, 9].iter().copied()), 2);
    }

    #[test]
    fn visibility_example() {
        let data = parse(EXAMPLE_TEXT);
        assert_eq!(
            data.visibility().as_rows(),
            vec![
                vec![true, true, true, true, true,],
                vec![true, true, true, false, true,],
                vec![true, true, false, true, true,],
                vec![true, false, true, false, true,],
                vec![true, true, true, true, true,],
            ]
        );
    }

    #[test]
    fn scenic_score_example() {
        let data = parse(EXAMPLE_TEXT);
        let scenic_score = data.scenic_score();
        assert_eq!(scenic_score.get(1, 2).unwrap(), &4);
        assert_eq!(scenic_score.get(3, 2).unwrap(), &8);
    }

    #[test]
    fn challange1_example() {
        let data = parse(EXAMPLE_TEXT);
        assert_eq!(challange1(&data), 21);
    }

    #[test]
    fn challange2_example() {
        let data = parse(EXAMPLE_TEXT);
        assert_eq!(challange2(&data), 8);
    }
}

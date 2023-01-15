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

use std::collections::HashSet;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Point {
    x: i32,
    y: i32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Rope {
    knots: Vec<Point>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Movement {
    x: i32,
    y: i32,
}
pub fn parse(text: &str) -> Vec<Movement> {
    text.lines()
        .map(|line| {
            let (direction, amount) = line.split_once(' ').unwrap();
            let amount = amount.parse().unwrap();
            match direction {
                "R" => Movement { x: amount, y: 0 },
                "L" => Movement { x: -amount, y: 0 },
                "U" => Movement { y: amount, x: 0 },
                "D" => Movement { y: -amount, x: 0 },
                _ => panic!(),
            }
        })
        .collect()
}

impl Rope {
    fn new(len: usize) -> Self {
        let mut knots = Vec::with_capacity(len);
        knots.resize(len, Point::default());
        Self { knots }
    }

    fn move_head(&mut self, movement: Movement) {
        self.knots[0].x += movement.x;
        self.knots[0].y += movement.y;
    }

    fn move_knot(&mut self, i: usize) {
        let x_delta = self.knots[i - 1].x.abs_diff(self.knots[i].x);
        let y_delta = self.knots[i - 1].y.abs_diff(self.knots[i].y);

        if (x_delta + y_delta) > 2 {
            // Diagonal
            if self.knots[i - 1].x - self.knots[i].x >= 1 {
                self.knots[i].x += 1;
            } else if self.knots[i - 1].x - self.knots[i].x <= -1 {
                self.knots[i].x -= 1;
            }
            if self.knots[i - 1].y - self.knots[i].y >= 1 {
                self.knots[i].y += 1;
            } else if self.knots[i - 1].y - self.knots[i].y <= -1 {
                self.knots[i].y -= 1;
            }
        } else if (x_delta + y_delta) > 1 {
            // Horizonal or vertical
            if self.knots[i - 1].x - self.knots[i].x > 1 {
                self.knots[i].x += 1;
            } else if self.knots[i - 1].x - self.knots[i].x < -1 {
                self.knots[i].x -= 1;
            }
            if self.knots[i - 1].y - self.knots[i].y > 1 {
                self.knots[i].y += 1;
            } else if self.knots[i - 1].y - self.knots[i].y < -1 {
                self.knots[i].y -= 1;
            }
        }
    }

    fn move_knots(&mut self) {
        for i in 1..self.knots.len() {
            self.move_knot(i);
        }
    }

    fn tail(&self) -> &Point {
        self.knots.last().unwrap()
    }
}

impl Iterator for Movement {
    type Item = Movement;

    fn next(&mut self) -> Option<Self::Item> {
        if self.x > 0 {
            self.x -= 1;
            Some(Movement { x: 1, y: 0 })
        } else if self.x < 0 {
            self.x += 1;
            Some(Movement { x: -1, y: 0 })
        } else if self.y > 0 {
            self.y -= 1;
            Some(Movement { y: 1, x: 0 })
        } else if self.y < 0 {
            self.y += 1;
            Some(Movement { y: -1, x: 0 })
        } else {
            None
        }
    }
}
pub fn challange1(movements: &[Movement]) -> usize {
    let mut rope = Rope::new(2);
    let tail_positions = movements
        .iter()
        .copied()
        .flatten()
        .map(|m| {
            rope.move_head(m);
            rope.move_knots();
            *rope.tail()
        })
        .collect::<HashSet<Point>>();
    tail_positions.len()
}

pub fn challange2(movements: &[Movement]) -> usize {
    let mut rope = Rope::new(10);
    let tail_positions = movements
        .iter()
        .copied()
        .flatten()
        .map(|m| {
            rope.move_head(m);
            rope.move_knots();
            *rope.tail()
        })
        .collect::<HashSet<Point>>();
    tail_positions.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_TEXT: &str = "\
R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    const EXAMPLE2_TEXT: &str = "\
R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

    #[test]
    fn parse_example() {
        let data = parse(EXAMPLE_TEXT);
        assert_eq!(
            data,
            vec![
                Movement { x: 4, y: 0 },
                Movement { x: 0, y: 4 },
                Movement { x: -3, y: 0 },
                Movement { x: 0, y: -1 },
                Movement { x: 4, y: 0 },
                Movement { x: 0, y: -1 },
                Movement { x: -5, y: 0 },
                Movement { x: 2, y: 0 },
            ]
        );
    }

    #[test]
    fn challange1_example() {
        let data = parse(EXAMPLE_TEXT);
        assert_eq!(challange1(&data), 13);
    }

    #[test]
    fn challange2_example() {
        let data = parse(EXAMPLE_TEXT);
        assert_eq!(challange2(&data), 1);
    }

    #[test]
    fn challange2_example2() {
        let data = parse(EXAMPLE2_TEXT);
        assert_eq!(challange2(&data), 36);
    }
}

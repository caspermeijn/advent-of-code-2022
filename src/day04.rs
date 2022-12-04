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

use std::ops::RangeInclusive;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Assignment {
    first: RangeInclusive<u32>,
    second: RangeInclusive<u32>,
}

pub fn parse(text: &str) -> Vec<Assignment> {
    text.lines().map(parse_assignment).collect()
}

fn parse_assignment(text: &str) -> Assignment {
    let (first_range, second_range) = text.split_once(',').unwrap();
    Assignment {
        first: parse_range(first_range),
        second: parse_range(second_range),
    }
}

fn parse_range(text: &str) -> RangeInclusive<u32> {
    let (first, second) = text.split_once('-').unwrap();

    RangeInclusive::new(first.parse().unwrap(), second.parse().unwrap())
}

impl Assignment {
    fn has_fully_overlap(&self) -> bool {
        (self.first.contains(self.second.start()) && self.first.contains(self.second.end()))
            || (self.second.contains(self.first.start()) && self.second.contains(self.first.end()))
    }

    fn has_any_overlap(&self) -> bool {
        self.first.contains(self.second.start())
            || self.first.contains(self.second.end())
            || self.second.contains(self.first.start())
            || self.second.contains(self.first.end())
    }
}

pub fn challange1(assignments: &[Assignment]) -> u32 {
    assignments
        .iter()
        .filter(|&assignment| assignment.has_fully_overlap())
        .count()
        .try_into()
        .unwrap()
}

pub fn challange2(assignments: &[Assignment]) -> u32 {
    assignments
        .iter()
        .filter(|&assignment| assignment.has_any_overlap())
        .count()
        .try_into()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_TEXT: &str = "\
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
";

    #[test]
    fn parse_example() {
        let assignments = parse(EXAMPLE_TEXT);
        assert_eq!(
            assignments,
            vec!(
                Assignment {
                    first: 2..=4,
                    second: 6..=8,
                },
                Assignment {
                    first: 2..=3,
                    second: 4..=5,
                },
                Assignment {
                    first: 5..=7,
                    second: 7..=9,
                },
                Assignment {
                    first: 2..=8,
                    second: 3..=7,
                },
                Assignment {
                    first: 6..=6,
                    second: 4..=6,
                },
                Assignment {
                    first: 2..=6,
                    second: 4..=8,
                },
            )
        );
    }

    #[test]
    fn has_fully_overlap_example() {
        let assignments = parse(EXAMPLE_TEXT);
        let mut fully_overlap = assignments.iter().map(Assignment::has_fully_overlap);
        assert_eq!(fully_overlap.next().unwrap(), false);
        assert_eq!(fully_overlap.next().unwrap(), false);
        assert_eq!(fully_overlap.next().unwrap(), false);
        assert_eq!(fully_overlap.next().unwrap(), true);
        assert_eq!(fully_overlap.next().unwrap(), true);
        assert_eq!(fully_overlap.next().unwrap(), false);
    }

    #[test]
    fn challange1_example() {
        let data = parse(EXAMPLE_TEXT);
        assert_eq!(challange1(&data), 2);
    }

    #[test]
    fn challange2_example() {
        let data = parse(EXAMPLE_TEXT);
        assert_eq!(challange2(&data), 4);
    }
}

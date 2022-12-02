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

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Elf {
    total_calories: u32,
}

pub fn parse(text: &str) -> Vec<Elf> {
    text.split("\n\n").map(parse_elf).collect()
}

fn parse_elf(text: &str) -> Elf {
    let total_calories = text
        .split('\n')
        .map(|calories_text| calories_text.parse::<u32>().unwrap())
        .sum();
    Elf { total_calories }
}

pub fn challange1(elves: &[Elf]) -> u32 {
    let most_calorie_elf = elves.iter().max_by_key(|elf| elf.total_calories).unwrap();

    most_calorie_elf.total_calories
}

pub fn challange2(elves: &[Elf]) -> u32 {
    let mut sorted_elves = elves.to_vec();
    sorted_elves.sort_by_key(|elf| elf.total_calories);
    sorted_elves.reverse();

    let top3_elves = sorted_elves.iter().take(3);
    top3_elves.map(|elf| elf.total_calories).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_TEXT: &str = "\
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn parse_example() {
        let elves = parse(EXAMPLE_TEXT);
        assert_eq!(
            elves,
            vec!(
                Elf {
                    total_calories: 6000,
                },
                Elf {
                    total_calories: 4000,
                },
                Elf {
                    total_calories: 11000,
                },
                Elf {
                    total_calories: 24000,
                },
                Elf {
                    total_calories: 10000,
                },
            )
        );
    }

    #[test]
    fn challange1_example() {
        let elves = parse(EXAMPLE_TEXT);
        assert_eq!(challange1(&elves), 24000);
    }

    #[test]
    fn challange2_example() {
        let elves = parse(EXAMPLE_TEXT);
        assert_eq!(challange2(&elves), 45000);
    }
}

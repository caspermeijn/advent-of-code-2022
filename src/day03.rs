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
pub struct Rucksack {
    all_items: String,
    first_compartment: String,
    second_compartment: String,
}

pub fn parse(text: &str) -> Vec<Rucksack> {
    text.lines().map(parse_rucksack).collect()
}

fn parse_rucksack(text: &str) -> Rucksack {
    let mid = text.len() / 2;
    let (first, second) = text.split_at(mid);
    Rucksack {
        all_items: text.to_string(),
        first_compartment: first.to_string(),
        second_compartment: second.to_string(),
    }
}

impl Rucksack {
    fn find_packing_failure(&self) -> char {
        for first_item in self.first_compartment.chars() {
            if self.second_compartment.contains(first_item) {
                return first_item;
            }
        }
        panic!()
    }
}

fn calc_item_priority(item: char) -> u32 {
    match item {
        'a'..='z' => item as u32 - 'a' as u32 + 1,
        'A'..='Z' => item as u32 - 'A' as u32 + 27,
        _ => panic!(),
    }
}

pub fn challange1(rucksacks: &[Rucksack]) -> u32 {
    rucksacks
        .iter()
        .map(Rucksack::find_packing_failure)
        .map(calc_item_priority)
        .sum()
}

fn find_common_item(group: &[Rucksack]) -> char {
    for item in group[0].all_items.chars() {
        if group[1].all_items.contains(item) && group[2].all_items.contains(item) {
            return item;
        }
    }
    panic!()
}

pub fn challange2(rucksacks: &[Rucksack]) -> u32 {
    let groups = rucksacks.chunks(3);

    groups.map(find_common_item).map(calc_item_priority).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_TEXT: &str = "\
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn parse_example() {
        let rucksacks = parse(EXAMPLE_TEXT);
        assert_eq!(
            rucksacks,
            vec!(
                Rucksack {
                    all_items: "vJrwpWtwJgWrhcsFMMfFFhFp".to_string(),
                    first_compartment: "vJrwpWtwJgWr".to_string(),
                    second_compartment: "hcsFMMfFFhFp".to_string(),
                },
                Rucksack {
                    all_items: "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL".to_string(),
                    first_compartment: "jqHRNqRjqzjGDLGL".to_string(),
                    second_compartment: "rsFMfFZSrLrFZsSL".to_string(),
                },
                Rucksack {
                    all_items: "PmmdzqPrVvPwwTWBwg".to_string(),
                    first_compartment: "PmmdzqPrV".to_string(),
                    second_compartment: "vPwwTWBwg".to_string(),
                },
                Rucksack {
                    all_items: "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn".to_string(),
                    first_compartment: "wMqvLMZHhHMvwLH".to_string(),
                    second_compartment: "jbvcjnnSBnvTQFn".to_string(),
                },
                Rucksack {
                    all_items: "ttgJtRGJQctTZtZT".to_string(),
                    first_compartment: "ttgJtRGJ".to_string(),
                    second_compartment: "QctTZtZT".to_string(),
                },
                Rucksack {
                    all_items: "CrZsJsPPZsGzwwsLwLmpwMDw".to_string(),
                    first_compartment: "CrZsJsPPZsGz".to_string(),
                    second_compartment: "wwsLwLmpwMDw".to_string(),
                },
            )
        );
    }

    #[test]
    fn packing_failure_example() {
        let data = parse(EXAMPLE_TEXT);
        let mut failures = data.iter().map(Rucksack::find_packing_failure);
        assert_eq!(failures.next(), Some('p'));
        assert_eq!(failures.next(), Some('L'));
        assert_eq!(failures.next(), Some('P'));
        assert_eq!(failures.next(), Some('v'));
        assert_eq!(failures.next(), Some('t'));
        assert_eq!(failures.next(), Some('s'));
    }

    #[test]
    fn calc_item_priority_example() {
        assert_eq!(calc_item_priority('p'), 16);
        assert_eq!(calc_item_priority('L'), 38);
        assert_eq!(calc_item_priority('P'), 42);
        assert_eq!(calc_item_priority('v'), 22);
        assert_eq!(calc_item_priority('t'), 20);
        assert_eq!(calc_item_priority('s'), 19);
    }

    #[test]
    fn challange1_example() {
        let data = parse(EXAMPLE_TEXT);
        assert_eq!(challange1(&data), 157);
    }

    #[test]
    fn challange2_example() {
        let data = parse(EXAMPLE_TEXT);
        assert_eq!(challange2(&data), 70);
    }
}

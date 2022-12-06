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

use std::collections::VecDeque;

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct Ship {
    layout: [VecDeque<char>; 9],
}

impl Ship {
    fn execute_challange1(&mut self, instruction: Instruction) {
        for _ in 0..instruction.amount {
            let crane = self.layout[instruction.from - 1].pop_front().unwrap();
            self.layout[instruction.to - 1].push_front(crane)
        }
    }

    fn execute_challange2(&mut self, instruction: Instruction) {
        let mut crane = VecDeque::new();
        for _ in 0..instruction.amount {
            crane.push_front(self.layout[instruction.from - 1].pop_front().unwrap());
        }
        for _ in 0..instruction.amount {
            self.layout[instruction.to - 1].push_front(crane.pop_front().unwrap());
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Instruction {
    amount: usize,
    from: usize,
    to: usize,
}

pub fn parse(text: &str) -> (Ship, Vec<Instruction>) {
    let (ship_text, instruction_text) = text.split_once("\n\n").unwrap();

    (parse_ship(ship_text), parse_instructions(instruction_text))
}

fn parse_ship(text: &str) -> Ship {
    let mut ship = Ship::default();
    for line in text.lines() {
        let container_row =
            line.chars()
                .enumerate()
                .filter_map(|(i, c)| if i % 4 == 1 { Some(c) } else { None });
        for (position, container) in container_row.enumerate() {
            if container.is_alphabetic() {
                ship.layout[position].push_back(container)
            }
        }
    }
    ship
}

fn parse_instructions(text: &str) -> Vec<Instruction> {
    text.lines().map(parse_instruction).collect()
}

fn parse_instruction(text: &str) -> Instruction {
    let mut parts = text.split_whitespace();
    assert_eq!(parts.next().unwrap(), "move");
    let amount = parts.next().unwrap().parse().unwrap();
    assert_eq!(parts.next().unwrap(), "from");
    let from = parts.next().unwrap().parse().unwrap();
    assert_eq!(parts.next().unwrap(), "to");
    let to = parts.next().unwrap().parse().unwrap();
    Instruction { amount, from, to }
}

pub fn challange1((ship, instructions): (Ship, Vec<Instruction>)) -> String {
    let mut ship = ship;
    for instruction in instructions {
        ship.execute_challange1(instruction);
    }
    ship.layout
        .iter()
        .map(|vec| vec.front().unwrap_or(&' '))
        .collect()
}

pub fn challange2((ship, instructions): (Ship, Vec<Instruction>)) -> String {
    let mut ship = ship;
    for instruction in instructions {
        ship.execute_challange2(instruction);
    }
    ship.layout
        .iter()
        .map(|vec| vec.front().unwrap_or(&' '))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_TEXT: &str = "\
.   [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
";

    #[test]
    fn parse_example() {
        let (ship, instructions) = parse(EXAMPLE_TEXT);
        assert_eq!(
            ship,
            Ship {
                layout: [
                    ['N', 'Z',].into(),
                    ['D', 'C', 'M',].into(),
                    ['P',].into(),
                    [].into(),
                    [].into(),
                    [].into(),
                    [].into(),
                    [].into(),
                    [].into()
                ]
            }
        );
        assert_eq!(
            instructions,
            vec!(
                Instruction {
                    amount: 1,
                    from: 2,
                    to: 1,
                },
                Instruction {
                    amount: 3,
                    from: 1,
                    to: 3,
                },
                Instruction {
                    amount: 2,
                    from: 2,
                    to: 1,
                },
                Instruction {
                    amount: 1,
                    from: 1,
                    to: 2,
                },
            )
        );
    }

    #[test]
    fn challange1_example() {
        let data = parse(EXAMPLE_TEXT);
        assert_eq!(challange1(data), "CMZ      ");
    }

    #[test]
    fn challange2_example() {
        let data = parse(EXAMPLE_TEXT);
        assert_eq!(challange2(data), "MCD      ");
    }
}

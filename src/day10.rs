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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Instruction {
    Noop,
    AddX(i32),
}

pub fn parse(text: &str) -> Vec<Instruction> {
    text.lines().map(parse_instruction).collect()
}

pub fn parse_instruction(text: &str) -> Instruction {
    if text == "noop" {
        Instruction::Noop
    } else {
        let (_instruction_text, value_text) = text.split_once(' ').unwrap();
        Instruction::AddX(value_text.parse().unwrap())
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Processor {
    instructions_to_go: VecDeque<Instruction>,
    instruction_in_progress: Option<Instruction>,
    x_register: i32,
    cycle_count: usize,
}

impl Processor {
    fn new(instructions_to_go: &[Instruction]) -> Self {
        Self {
            instructions_to_go: VecDeque::from(instructions_to_go.to_vec()),
            instruction_in_progress: None,
            x_register: 1,
            cycle_count: 0,
        }
    }

    fn execute_1_cycle(&mut self) {
        if let Some(instruction_in_progress) = self.instruction_in_progress.take() {
            match instruction_in_progress {
                Instruction::Noop => panic!(),
                Instruction::AddX(value) => self.x_register += value,
            }
        } else {
            let next_instruction = self.instructions_to_go.pop_front().unwrap();
            match next_instruction {
                Instruction::Noop => {}
                Instruction::AddX(value) => {
                    self.instruction_in_progress = Some(Instruction::AddX(value));
                }
            }
        }
    }

    fn execute_cycles(&mut self, cycles: usize) {
        for _ in 0..cycles {
            self.execute_1_cycle()
        }
    }

    pub fn x_register(&self) -> i32 {
        self.x_register
    }
}

pub fn challange1(instructions: &[Instruction]) -> i32 {
    let mut processor = Processor::new(instructions);
    let mut total = 0;
    processor.execute_cycles(19);
    total += 20 * processor.x_register();
    for i in 1..=5 {
        processor.execute_cycles(40);
        total += (20 + i * 40) * processor.x_register();
    }
    total
}

pub fn challange2(instructions: &[Instruction]) -> String {
    let mut processor = Processor::new(instructions);
    let mut result = String::with_capacity(6 * 41);

    for _ in 0..6 {
        for x in 0..40 {
            if processor.x_register() - 1 <= x && x <= processor.x_register() + 1 {
                result.push('#');
            } else {
                result.push('.');
            }
            processor.execute_1_cycle();
        }
        result.push('\n')
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_TEXT: &str = "\
noop
addx 3
addx -5";

    const EXAMPLE2_TEXT: &str = "\
addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

    #[test]
    fn parse_example() {
        let data = parse(EXAMPLE_TEXT);
        assert_eq!(
            data,
            vec![
                Instruction::Noop,
                Instruction::AddX(3),
                Instruction::AddX(-5),
            ]
        );
    }

    #[test]
    fn processor_example() {
        let data = parse(EXAMPLE_TEXT);
        let mut processor = Processor::new(&data);
        assert_eq!(processor.x_register(), 1);
        processor.execute_1_cycle();
        assert_eq!(processor.x_register(), 1);
        processor.execute_1_cycle();
        assert_eq!(processor.x_register(), 1);
        processor.execute_1_cycle();
        assert_eq!(processor.x_register(), 4);
        processor.execute_1_cycle();
        assert_eq!(processor.x_register(), 4);
        processor.execute_1_cycle();
        assert_eq!(processor.x_register(), -1);
    }

    #[test]
    fn processor_example2() {
        let data = parse(EXAMPLE2_TEXT);
        let mut processor = Processor::new(&data);
        assert_eq!(processor.x_register(), 1);
        processor.execute_cycles(19);
        assert_eq!(processor.x_register(), 21);
        processor.execute_cycles(40);
        assert_eq!(processor.x_register(), 19);
        processor.execute_cycles(40);
        assert_eq!(processor.x_register(), 18);
        processor.execute_cycles(40);
        assert_eq!(processor.x_register(), 21);
        processor.execute_cycles(40);
        assert_eq!(processor.x_register(), 16);
        processor.execute_cycles(40);
        assert_eq!(processor.x_register(), 18);
    }

    #[test]
    fn challange1_example2() {
        let data = parse(EXAMPLE2_TEXT);
        assert_eq!(challange1(&data), 13140);
    }

    #[test]
    fn challange2_example2() {
        let data = parse(EXAMPLE2_TEXT);
        assert_eq!(
            challange2(&data),
            "\
##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
"
        );
    }
}

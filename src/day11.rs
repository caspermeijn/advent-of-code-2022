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

use std::{
    cell::RefCell,
    collections::{BTreeMap, VecDeque},
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Operation {
    OldMultiplyOld,
    OldMultiplyValue(usize),
    OldAddValue(usize),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Test {
    divisible_by: usize,
    if_true_throw_to_monkey: usize,
    if_false_throw_to_monkey: usize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Monkey {
    items: RefCell<VecDeque<usize>>,
    operation: Operation,
    test: Test,
}

pub fn parse(text: &str) -> Vec<Monkey> {
    text.split("\n\n").map(parse_monkey).collect()
}

pub fn parse_monkey(text: &str) -> Monkey {
    let mut lines = text.lines();
    let _header = lines.next().unwrap();
    let starting_items = parse_starting_items(lines.next().unwrap());
    let operation = parse_operation(lines.next().unwrap());
    let test = parse_test(
        lines.next().unwrap(),
        lines.next().unwrap(),
        lines.next().unwrap(),
    );
    Monkey {
        items: VecDeque::from(starting_items).into(),
        operation,
        test,
    }
}

pub fn parse_starting_items(text: &str) -> Vec<usize> {
    let (_header, items) = text.split_once(": ").unwrap();
    items.split(", ").map(|t| t.parse().unwrap()).collect()
}

pub fn parse_operation(text: &str) -> Operation {
    let (_header, operation) = text.split_once(": ").unwrap();
    let end = operation.strip_prefix("new = old ").unwrap();
    if end == "* old" {
        Operation::OldMultiplyOld
    } else if end.starts_with('*') {
        let value = end.strip_prefix("* ").unwrap();
        Operation::OldMultiplyValue(value.parse().unwrap())
    } else {
        let value = end.strip_prefix("+ ").unwrap();
        Operation::OldAddValue(value.parse().unwrap())
    }
}

pub fn parse_test(text_divisible: &str, text_if_true: &str, text_if_false: &str) -> Test {
    Test {
        divisible_by: text_divisible
            .strip_prefix("  Test: divisible by ")
            .unwrap()
            .parse()
            .unwrap(),
        if_true_throw_to_monkey: text_if_true
            .strip_prefix("    If true: throw to monkey ")
            .unwrap()
            .parse()
            .unwrap(),
        if_false_throw_to_monkey: text_if_false
            .strip_prefix("    If false: throw to monkey ")
            .unwrap()
            .parse()
            .unwrap(),
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Throw {
    monkey: usize,
    item: usize,
}

impl Monkey {
    fn execute_turn(&self, divisor_product: Option<usize>) -> Vec<Throw> {
        let items = self.items.borrow().clone();
        self.items.borrow_mut().clear();
        items
            .iter()
            .map(|&item| {
                let worry_level = self.operation.calculate(item);
                let worry_level = if let Some(divisor_product) = divisor_product {
                    worry_level % divisor_product
                } else {
                    worry_level / 3
                };
                let monkey = self.test.test(worry_level);
                Throw {
                    monkey,
                    item: worry_level,
                }
            })
            .collect()
    }

    fn receive_item(&self, item: usize) {
        self.items.borrow_mut().push_back(item)
    }
}

impl Operation {
    fn calculate(&self, worry_level: usize) -> usize {
        match self {
            Operation::OldMultiplyOld => worry_level * worry_level,
            Operation::OldMultiplyValue(value) => worry_level * value,
            Operation::OldAddValue(value) => worry_level + value,
        }
    }
}

impl Test {
    fn test(&self, item: usize) -> usize {
        if item % self.divisible_by == 0 {
            self.if_true_throw_to_monkey
        } else {
            self.if_false_throw_to_monkey
        }
    }
}

pub fn challange1(monkeys: &[Monkey]) -> usize {
    let monkeys = monkeys.to_vec();
    let mut inspections = BTreeMap::new();
    for _round in 0..20 {
        for (i, monkey) in monkeys.iter().enumerate() {
            let throws = monkey.execute_turn(None);
            let inspection: &mut usize = inspections.entry(i).or_default();
            *inspection += throws.len();

            for throw in throws {
                monkeys[throw.monkey].receive_item(throw.item)
            }
        }
    }
    let mut business = inspections.values().collect::<Vec<_>>();
    business.sort();
    business.reverse();
    business[0] * business[1]
}

pub fn challange2(monkeys: &[Monkey]) -> usize {
    let monkeys = monkeys.to_vec();
    let mut inspections = BTreeMap::new();
    let divisor_product = monkeys
        .iter()
        .map(|m| m.test.divisible_by)
        .product::<usize>();
    for _round in 0..10000 {
        for (i, monkey) in monkeys.iter().enumerate() {
            let throws = monkey.execute_turn(Some(divisor_product));
            monkey
                .items
                .borrow_mut()
                .iter_mut()
                .for_each(|item| *item %= divisor_product);
            let inspection: &mut usize = inspections.entry(i).or_default();
            *inspection += throws.len();

            for throw in throws {
                monkeys[throw.monkey].receive_item(throw.item)
            }
        }
    }
    let mut business = inspections.values().collect::<Vec<_>>();
    business.sort();
    business.reverse();
    business[0] * business[1]
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_TEXT: &str = "\
Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

    #[test]
    fn parse_example() {
        let data = parse(EXAMPLE_TEXT);
        assert_eq!(
            data,
            vec![
                Monkey {
                    items: VecDeque::from([79, 98]).into(),
                    operation: Operation::OldMultiplyValue(19),
                    test: Test {
                        divisible_by: 23,
                        if_true_throw_to_monkey: 2,
                        if_false_throw_to_monkey: 3
                    }
                },
                Monkey {
                    items: VecDeque::from([54, 65, 75, 74]).into(),
                    operation: Operation::OldAddValue(6),
                    test: Test {
                        divisible_by: 19,
                        if_true_throw_to_monkey: 2,
                        if_false_throw_to_monkey: 0
                    }
                },
                Monkey {
                    items: VecDeque::from([79, 60, 97]).into(),
                    operation: Operation::OldMultiplyOld,
                    test: Test {
                        divisible_by: 13,
                        if_true_throw_to_monkey: 1,
                        if_false_throw_to_monkey: 3
                    }
                },
                Monkey {
                    items: VecDeque::from([74]).into(),
                    operation: Operation::OldAddValue(3),
                    test: Test {
                        divisible_by: 17,
                        if_true_throw_to_monkey: 0,
                        if_false_throw_to_monkey: 1
                    }
                },
            ]
        );
    }

    #[test]
    fn round1_example() {
        let monkeys = parse(EXAMPLE_TEXT);
        let throws = monkeys[0].execute_turn(None);
        assert_eq!(
            throws,
            vec![
                Throw {
                    monkey: 3,
                    item: 500
                },
                Throw {
                    monkey: 3,
                    item: 620
                },
            ]
        );
        for throw in throws {
            monkeys[throw.monkey].receive_item(throw.item)
        }

        let throws = monkeys[1].execute_turn(None);
        assert_eq!(
            throws,
            vec![
                Throw {
                    monkey: 0,
                    item: 20
                },
                Throw {
                    monkey: 0,
                    item: 23
                },
                Throw {
                    monkey: 0,
                    item: 27
                },
                Throw {
                    monkey: 0,
                    item: 26
                },
            ]
        );
        for throw in throws {
            monkeys[throw.monkey].receive_item(throw.item)
        }

        let throws = monkeys[2].execute_turn(None);
        assert_eq!(
            throws,
            vec![
                Throw {
                    monkey: 1,
                    item: 2080
                },
                Throw {
                    monkey: 3,
                    item: 1200
                },
                Throw {
                    monkey: 3,
                    item: 3136
                },
            ]
        );
        for throw in throws {
            monkeys[throw.monkey].receive_item(throw.item)
        }

        let throws = monkeys[3].execute_turn(None);
        assert_eq!(
            throws,
            vec![
                Throw {
                    monkey: 1,
                    item: 25
                },
                Throw {
                    monkey: 1,
                    item: 167
                },
                Throw {
                    monkey: 1,
                    item: 207
                },
                Throw {
                    monkey: 1,
                    item: 401
                },
                Throw {
                    monkey: 1,
                    item: 1046
                },
            ]
        );
        for throw in throws {
            monkeys[throw.monkey].receive_item(throw.item)
        }
    }

    #[test]
    fn challange1_example() {
        let data = parse(EXAMPLE_TEXT);
        assert_eq!(challange1(&data), 10605);
    }

    #[test]
    fn challange2_example() {
        let data = parse(EXAMPLE_TEXT);
        assert_eq!(challange2(&data), 2713310158);
    }
}

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
enum Shape {
    Rock,
    Paper,
    Scissors,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Winner {
    Opponent,
    Myself,
    Draw,
}

fn parse_shape(text: &str) -> Shape {
    match text.chars().next().unwrap() {
        'A' => Shape::Rock,
        'B' => Shape::Paper,
        'C' => Shape::Scissors,
        'X' => Shape::Rock,
        'Y' => Shape::Paper,
        'Z' => Shape::Scissors,
        _ => panic!(),
    }
}

pub mod challange1 {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub struct Round {
        opponent: Shape,
        mine: Shape,
    }

    impl Round {
        fn score(&self) -> u32 {
            let shape_score = match self.mine {
                Shape::Rock => 1,
                Shape::Paper => 2,
                Shape::Scissors => 3,
            };
            let result_score = match self.result() {
                Winner::Opponent => 0,
                Winner::Myself => 6,
                Winner::Draw => 3,
            };
            shape_score + result_score
        }

        fn result(&self) -> Winner {
            match self.opponent {
                Shape::Rock => match self.mine {
                    Shape::Rock => Winner::Draw,
                    Shape::Paper => Winner::Myself,
                    Shape::Scissors => Winner::Opponent,
                },
                Shape::Paper => match self.mine {
                    Shape::Rock => Winner::Opponent,
                    Shape::Paper => Winner::Draw,
                    Shape::Scissors => Winner::Myself,
                },
                Shape::Scissors => match self.mine {
                    Shape::Rock => Winner::Myself,
                    Shape::Paper => Winner::Opponent,
                    Shape::Scissors => Winner::Draw,
                },
            }
        }
    }

    pub fn parse(text: &str) -> Vec<Round> {
        text.lines().map(parse_round).collect()
    }

    fn parse_round(text: &str) -> Round {
        let (opponent_text, mine_text) = text.split_once(' ').unwrap();
        Round {
            opponent: parse_shape(opponent_text),
            mine: parse_shape(mine_text),
        }
    }

    pub fn challange1(rounds: &[Round]) -> u32 {
        rounds.iter().map(Round::score).sum()
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        const EXAMPLE_TEXT: &str = "\
A Y
B X
C Z";

        #[test]
        fn parse_example() {
            let rounds = parse(EXAMPLE_TEXT);
            assert_eq!(
                rounds,
                vec!(
                    Round {
                        opponent: Shape::Rock,
                        mine: Shape::Paper,
                    },
                    Round {
                        opponent: Shape::Paper,
                        mine: Shape::Rock,
                    },
                    Round {
                        opponent: Shape::Scissors,
                        mine: Shape::Scissors,
                    },
                )
            );
        }

        #[test]
        fn challange1_example() {
            let rounds = parse(EXAMPLE_TEXT);
            assert_eq!(challange1(&rounds), 15);
        }
    }
}

pub mod challange2 {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub struct Round {
        opponent: Shape,
        result: Winner,
    }

    impl Round {
        fn score(&self) -> u32 {
            let shape_score = match self.mine() {
                Shape::Rock => 1,
                Shape::Paper => 2,
                Shape::Scissors => 3,
            };
            let result_score = match self.result {
                Winner::Opponent => 0,
                Winner::Myself => 6,
                Winner::Draw => 3,
            };
            shape_score + result_score
        }

        fn mine(&self) -> Shape {
            match self.opponent {
                Shape::Rock => match self.result {
                    Winner::Opponent => Shape::Scissors,
                    Winner::Myself => Shape::Paper,
                    Winner::Draw => Shape::Rock,
                },
                Shape::Paper => match self.result {
                    Winner::Opponent => Shape::Rock,
                    Winner::Myself => Shape::Scissors,
                    Winner::Draw => Shape::Paper,
                },
                Shape::Scissors => match self.result {
                    Winner::Opponent => Shape::Paper,
                    Winner::Myself => Shape::Rock,
                    Winner::Draw => Shape::Scissors,
                },
            }
        }
    }

    pub fn parse(text: &str) -> Vec<Round> {
        text.lines().map(parse_round).collect()
    }

    fn parse_round(text: &str) -> Round {
        let (opponent_text, winner_text) = text.split_once(' ').unwrap();
        Round {
            opponent: parse_shape(opponent_text),
            result: parse_winner(winner_text),
        }
    }

    fn parse_winner(text: &str) -> Winner {
        match text.chars().next().unwrap() {
            'X' => Winner::Opponent,
            'Y' => Winner::Draw,
            'Z' => Winner::Myself,
            _ => panic!(),
        }
    }

    pub fn challange2(rounds: &[Round]) -> u32 {
        rounds.iter().map(Round::score).sum()
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        const EXAMPLE_TEXT: &str = "\
A Y
B X
C Z";

        #[test]
        fn parse_example() {
            let rounds = parse(EXAMPLE_TEXT);
            assert_eq!(
                rounds,
                vec!(
                    Round {
                        opponent: Shape::Rock,
                        result: Winner::Draw,
                    },
                    Round {
                        opponent: Shape::Paper,
                        result: Winner::Opponent,
                    },
                    Round {
                        opponent: Shape::Scissors,
                        result: Winner::Myself,
                    },
                )
            );
        }

        #[test]
        fn challange2_example() {
            let rounds = parse(EXAMPLE_TEXT);
            assert_eq!(challange2(&rounds), 12);
        }
    }
}

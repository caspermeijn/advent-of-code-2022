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

use advent_of_code_2022::*;

fn main() {
    let text = include_str!("../data/day01.txt");
    let elves = day01::parse(text);
    let challange1 = day01::challange1(&elves);
    println!("Day 1, challange 1: {}", challange1);
    let challange2 = day01::challange2(&elves);
    println!("Day 1, challange 2: {}", challange2);

    let text = include_str!("../data/day02.txt");
    let data = day02::challange1::parse(text);
    let challange1 = day02::challange1::challange1(&data);
    println!("Day 2, challange 1: {}", challange1);
    let data = day02::challange2::parse(text);
    let challange2 = day02::challange2::challange2(&data);
    println!("Day 2, challange 2: {}", challange2);

    let text = include_str!("../data/day03.txt");
    let data = day03::parse(text);
    let challange1 = day03::challange1(&data);
    println!("Day 3, challange 1: {}", challange1);
    let challange2 = day03::challange2(&data);
    println!("Day 3, challange 2: {}", challange2);
}

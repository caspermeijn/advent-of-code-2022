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

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct Datasteam {
    data: String,
}

pub fn parse(text: &str) -> Datasteam {
    Datasteam {
        data: text.to_string(),
    }
}

pub fn is_unique(data: &[u8]) -> bool {
    let mut set = HashSet::new();
    for c in data {
        set.insert(c);
    }
    set.len() == data.len()
}

pub fn find_start_of_stream(datastream: &Datasteam, marker_len: usize) -> usize {
    datastream
        .data
        .as_bytes()
        .windows(marker_len)
        .enumerate()
        .find(|(_, window)| is_unique(window))
        .unwrap()
        .0
        + marker_len
}

pub fn challange1(datastream: &Datasteam) -> usize {
    find_start_of_stream(datastream, 4)
}
pub fn challange2(datastream: &Datasteam) -> usize {
    find_start_of_stream(datastream, 14)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_TEXT1: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
    const EXAMPLE_TEXT2: &str = "bvwbjplbgvbhsrlpgdmjqwftvncz";
    const EXAMPLE_TEXT3: &str = "nppdvjthqldpwncqszvftbrmjlhg";
    const EXAMPLE_TEXT4: &str = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
    const EXAMPLE_TEXT5: &str = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";

    #[test]
    fn challange1_example() {
        let data = parse(EXAMPLE_TEXT1);
        assert_eq!(challange1(&data), 7);
        let data = parse(EXAMPLE_TEXT2);
        assert_eq!(challange1(&data), 5);
        let data = parse(EXAMPLE_TEXT3);
        assert_eq!(challange1(&data), 6);
        let data = parse(EXAMPLE_TEXT4);
        assert_eq!(challange1(&data), 10);
        let data = parse(EXAMPLE_TEXT5);
        assert_eq!(challange1(&data), 11);
    }

    #[test]
    fn challange2_example() {
        let data = parse(EXAMPLE_TEXT1);
        assert_eq!(challange2(&data), 19);
        let data = parse(EXAMPLE_TEXT2);
        assert_eq!(challange2(&data), 23);
        let data = parse(EXAMPLE_TEXT3);
        assert_eq!(challange2(&data), 23);
        let data = parse(EXAMPLE_TEXT4);
        assert_eq!(challange2(&data), 29);
        let data = parse(EXAMPLE_TEXT5);
        assert_eq!(challange2(&data), 26);
    }

    // #[test]
    // fn challange2_example() {
    //     let data = parse(EXAMPLE_TEXT);
    //     assert_eq!(challange2(data), "MCD      ");
    // }
}

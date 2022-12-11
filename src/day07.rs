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

use std::{collections::VecDeque, path::PathBuf};

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct Directory {
    path: PathBuf,
    files: Vec<(String, usize)>,
}

impl Directory {
    fn sum_size_files(&self) -> usize {
        self.files.iter().map(|(_, size)| size).sum()
    }

    fn total_size(&self, all_directories: &Vec<Directory>) -> usize {
        let size_childeren: usize = all_directories
            .iter()
            // .filter(|other_directory| other_directory.path != self.path)
            .filter(|child| {
                if let Some(parent_path) = child.path.parent() {
                    parent_path == self.path
                } else {
                    false
                }
            })
            .map(|child| child.total_size(all_directories))
            .sum();

        size_childeren + self.sum_size_files()
    }
}

pub fn parse(text: &str) -> Vec<Directory> {
    let mut directories = Vec::new();
    let mut lines = text.lines().collect::<VecDeque<_>>();
    let mut current_path = PathBuf::new();
    while !lines.is_empty() {
        let command = lines.pop_front().unwrap();
        if command.starts_with("$ cd") {
            let directory = command.strip_prefix("$ cd ").unwrap();
            if directory == "/" {
                current_path.clear();
                current_path.push("/");
            } else if directory == ".." {
                current_path.pop();
            } else {
                current_path.push(directory);
            }
        } else if command.starts_with("$ ls") {
            let mut new_directory = Directory {
                path: current_path.clone(),
                ..Default::default()
            };
            while !lines.front().unwrap_or(&"$").starts_with('$') {
                let entry = lines.pop_front().unwrap();
                if !entry.starts_with("dir") {
                    let (size, name) = entry.split_once(' ').unwrap();
                    new_directory
                        .files
                        .push((name.to_string(), size.parse().unwrap()))
                }
            }
            directories.push(new_directory);
        }
    }
    directories
}

pub fn challange1(directories: &Vec<Directory>) -> usize {
    directories
        .iter()
        .map(|directory| directory.total_size(directories))
        .filter(|&size| size <= 100000)
        .sum()
}

pub fn challange2(directories: &Vec<Directory>) -> usize {
    let total_disk = 70000000;
    let update_size = 30000000;
    let used_disk = directories
        .iter()
        .find(|dir| dir.path == PathBuf::from("/"))
        .unwrap()
        .total_size(directories);
    let free_disk = total_disk - used_disk;
    let to_be_freeed = update_size - free_disk;

    directories
        .iter()
        .map(|directory| directory.total_size(directories))
        .filter(|&size| size >= to_be_freeed)
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_TEXT: &str = "\
$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    #[test]
    fn parse_example() {
        let directories = parse(EXAMPLE_TEXT);
        assert_eq!(
            directories,
            vec![
                Directory {
                    path: PathBuf::from("/"),
                    files: vec![
                        ("b.txt".to_string(), 14848514),
                        ("c.dat".to_string(), 8504156),
                    ]
                },
                Directory {
                    path: PathBuf::from("/a"),
                    files: vec![
                        ("f".to_string(), 29116),
                        ("g".to_string(), 2557),
                        ("h.lst".to_string(), 62596),
                    ]
                },
                Directory {
                    path: PathBuf::from("/a/e"),
                    files: vec![("i".to_string(), 584),]
                },
                Directory {
                    path: PathBuf::from("/d"),
                    files: vec![
                        ("j".to_string(), 4060174),
                        ("d.log".to_string(), 8033020),
                        ("d.ext".to_string(), 5626152),
                        ("k".to_string(), 7214296),
                    ]
                },
            ]
        );
    }

    #[test]
    fn total_size_example() {
        let directories = parse(EXAMPLE_TEXT);
        let mut total_sizes = directories
            .iter()
            .map(|directory| directory.total_size(&directories));
        assert_eq!(total_sizes.next().unwrap(), 48381165);
        assert_eq!(total_sizes.next().unwrap(), 94853);
        assert_eq!(total_sizes.next().unwrap(), 584);
        assert_eq!(total_sizes.next().unwrap(), 24933642);
    }

    #[test]
    fn challange1_example() {
        let data = parse(EXAMPLE_TEXT);
        assert_eq!(challange1(&data), 95437);
    }

    #[test]
    fn challange2_example() {
        let data = parse(EXAMPLE_TEXT);
        assert_eq!(challange2(&data), 24933642);
    }
}

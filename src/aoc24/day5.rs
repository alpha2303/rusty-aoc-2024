/*
 * Day 5: Print Queue
 * Question URL: https://adventofcode.com/2024/day/5
 *
*/

use std::{collections::HashSet, fs};

pub struct Day5 {}

impl Day5 {
    pub fn get_results() -> [String; 2] {
        let (adj_map, inputs) = Self::parse_input_file("./inputs/day5/day5.txt");
        return [
            Self::part1(&adj_map, &inputs),
            Self::part2(&adj_map, &inputs),
        ];
    }

    fn part1(adj_map: &HashSet<(i32, i32)>, inputs: &Vec<Vec<i32>>) -> String {
        let mut res: i32 = 0;
        for input in inputs {
            if Self::is_valid_update(input, adj_map) {
                res += input[input.len() / 2];
            }
        }

        return res.to_string();
    }

    fn part2(adj_map: &HashSet<(i32, i32)>, inputs: &Vec<Vec<i32>>) -> String {
        let mut res: i32 = 0;
        for input in inputs {
            if !Self::is_valid_update(input, adj_map) {
                let sorted_input = Self::sort_input(input, adj_map);
                res += sorted_input[sorted_input.len() / 2];
            }
        }
        return res.to_string();
    }

    fn is_valid_update(input: &Vec<i32>, adj_map: &HashSet<(i32, i32)>) -> bool {
        for i in 0..(input.len() - 1) {
            for j in (i + 1)..input.len() {
                if !adj_map.contains(&(input[i], input[j])) {
                    return false;
                }
            }
        }

        return true;
    }

    fn sort_input(input: &Vec<i32>, adj_map: &HashSet<(i32, i32)>) -> Vec<i32> {
        let mut sorted_input: Vec<i32> = input.clone();
        let mut swap = true;
        while swap {
            swap = false;
            for i in 0..(sorted_input.len() - 1) {
                if adj_map.contains(&(sorted_input[i], sorted_input[i + 1])) {
                    swap = true;
                    sorted_input.swap(i, i + 1);
                }
            }
        }

        return sorted_input;
    }

    fn parse_input_file(filepath: &str) -> (HashSet<(i32, i32)>, Vec<Vec<i32>>) {
        let mut adj_map: HashSet<(i32, i32)> = HashSet::new();
        let mut inputs: Vec<Vec<i32>> = vec![];

        match fs::read_to_string(filepath) {
            Ok(fp) => {
                let mut input_flag: bool = false;

                for line in fp.lines() {
                    if line.is_empty() {
                        input_flag = true;
                        continue;
                    }

                    if input_flag {
                        inputs.push(
                            line.split(",")
                                .map(|val| val.parse::<i32>().unwrap())
                                .collect::<Vec<i32>>(),
                        );
                    } else {
                        let page_nos: Vec<i32> = line
                            .split("|")
                            .map(|val| val.parse::<i32>().unwrap())
                            .collect::<Vec<i32>>();
                        adj_map.insert((page_nos[0], page_nos[1]));
                    }
                }
            }
            Err(err) => println!("Error while reading input: {}", err),
        }

        return (adj_map, inputs);
    }
}

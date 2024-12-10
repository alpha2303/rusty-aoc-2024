/*
 * Day 1: Historian Hysteria
 * Question URL: https://adventofcode.com/2024/day/1
 * 
 */

use std::collections::HashMap;
use std::fs;

pub struct Day1 {}

impl Day1 {
    pub fn get_results() -> [String; 2] {
        let (left, right) = Self::get_input().unwrap();
        return [Self::part1(&left, &right), Self::part2(&left, &right)];
    }

    fn part1(left_list: &Vec<i32>, right_list: &Vec<i32>) -> String {
        let mut sorted_left: Vec<i32> = left_list.clone();
        sorted_left.sort();
        let mut sorted_right: Vec<i32> = right_list.clone();
        sorted_right.sort();
        let mut result: i32 = 0;
        for i in 0..sorted_left.len() {
            if sorted_left[i] >= sorted_right[i] {
                result += sorted_left[i] - sorted_right[i];
            } else {
                result += sorted_right[i] - sorted_left[i];
            }
        }

        return result.to_string();
    }

    fn part2(left_list: &Vec<i32>, right_list: &Vec<i32>) -> String {
        let mut freq_map: HashMap<i32, i32> = HashMap::new();
        for id in right_list.iter() {
            *freq_map.entry(*id).or_insert(0) += 1
        }

        let mut sim_score: i32 = 0;
        for id in left_list.iter() {
            sim_score += id * freq_map.get(id).unwrap_or(&0);
        }

        return sim_score.to_string();
    }

    fn get_input() -> Result<(Vec<i32>, Vec<i32>), std::io::Error> {
        return match fs::read_to_string("./inputs/day1/day1.txt") {
            Ok(file_contents) => {
                let mut left_list: Vec<i32> = Vec::new();
                let mut right_list: Vec<i32> = Vec::new();

                for line in file_contents.lines() {
                    let line_split: Vec<&str> = line.split_whitespace().collect();
                    left_list.push(line_split[0].parse::<i32>().unwrap());
                    right_list.push(line_split[1].parse::<i32>().unwrap());
                }

                Ok((left_list, right_list))
            }
            Err(err) => Err(err),
        };
    }
}

/*
 * Day 3: Mull It Over
 * Question URL: https://adventofcode.com/2024/day/3
 * 
*/

use regex::Regex;
use std::fs;

pub struct Day3 {}

impl Day3 {
    pub fn get_results() -> [String; 2] {
        let file_contents: String = fs::read_to_string("./inputs/day3/day3.txt").unwrap();
        return [Self::part1(&file_contents), Self::part2(&file_contents)];
    }

    fn part1(input: &String) -> String {
        let mut res: i32 = 0;
        let re: Regex = Regex::new(r"mul\((?<i>\d+),(?<j>\d+)\)").unwrap();

        for (_, [i, j]) in re.captures_iter(input).map(|c| c.extract()) {
            res += i.parse::<i32>().unwrap() * j.parse::<i32>().unwrap();
        }

        return res.to_string();
    }

    fn part2(input: &String) -> String {
        let mut res: i32 = 0;
        let mut is_compute: bool = true;
        let re: Regex =
            Regex::new(r#"(mul)\((\d+),(\d+)\)|(do)\(()()\)|(don\'t)\(()()\)"#).unwrap();

        for (_, [cmd, i, j]) in re.captures_iter(input).map(|c| c.extract()) {
            match cmd {
                "do" => is_compute = true,
                "don't" => is_compute = false,
                _ => {
                    if is_compute {
                        res += i.parse::<i32>().unwrap() * j.parse::<i32>().unwrap();
                    }
                }
            }
        }

        return res.to_string();
    }
}

/*
 * Day 2: Red-Nosed Reports
 * Question URL: https://adventofcode.com/2024/day/2
 * 
*/

use std::fs;

pub struct Day2 {}

impl Day2 {
    pub fn get_results() -> [String; 2] {
        let file_contents: String = fs::read_to_string("./inputs/day2/day2.txt").unwrap();
        return [Self::part1(&file_contents), Self::part2(&file_contents)];
    }

    fn part1(input: &String) -> String {
        let mut safe_count: i32 = 0;
        for line in input.lines() {
            let mut line_iter: std::str::SplitWhitespace<'_> = line.split_whitespace();
            let mut prev_level: i32 = line_iter.next().unwrap().parse::<i32>().unwrap();
            let mut asc_flag: i32 = 0;
            let mut is_safe: bool = true;

            for level in line_iter {
                // println!("{}", level);
                let level_int: i32 = level.parse::<i32>().unwrap();
                let diff: i32 = level_int - prev_level;

                if asc_flag == 0 {
                    asc_flag = diff;
                }

                if diff.abs() < 1
                    || diff.abs() > 3
                    || (asc_flag > 0 && diff < 0)
                    || (asc_flag < 0 && diff > 0)
                {
                    is_safe = false;
                    break;
                }

                prev_level = level_int;
            }

            if is_safe {
                safe_count += 1;
            }
        }
        return safe_count.to_string();
    }

    fn part2(input: &String) -> String {
        let mut safe_count: i32 = 0;
        for line in input.lines() {
            let mut line_iter: std::str::SplitWhitespace<'_> = line.split_whitespace();
            let mut prev_level: i32 = line_iter.next().unwrap().parse::<i32>().unwrap();
            let mut asc_flag: i32 = 0;
            let mut is_safe: i32 = 0;

            for level in line_iter {
                // println!("{}", level);
                let level_int: i32 = level.parse::<i32>().unwrap();
                let diff: i32 = level_int - prev_level;

                if asc_flag == 0 {
                    asc_flag = diff;
                }

                if diff.abs() < 1
                    || diff.abs() > 3
                    || (asc_flag > 0 && diff < 0)
                    || (asc_flag < 0 && diff > 0)
                {
                    is_safe += 1;
                    if is_safe > 1 {
                        break;
                    }
                }

                prev_level = level_int;
            }

            if is_safe <= 1 {
                safe_count += 1;
            }
        }
        return safe_count.to_string();
    }
}

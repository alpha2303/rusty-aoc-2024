/*
 * Day 4: Ceres Search
 * Question URL: https://adventofcode.com/2024/day/4
 *
*/

use std::collections::hash_map::{HashMap, Iter};
use std::fs;

#[derive(Debug, PartialEq, Eq, Hash)]
enum Directions {
    NW,
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
}

struct DirectionMap {
    map: HashMap<Directions, [i32; 2]>,
}

impl DirectionMap {
    pub fn new() -> Self {
        Self {
            map: HashMap::from([
                (Directions::NW, [-1, -1]),
                (Directions::N, [0, -1]),
                (Directions::NE, [1, -1]),
                (Directions::E, [1, 0]),
                (Directions::SE, [1, 1]),
                (Directions::S, [0, 1]),
                (Directions::SW, [-1, 1]),
                (Directions::W, [-1, 0]),
            ]),
        }
    }

    pub fn get_offset(&self, dir: &Directions) -> &[i32; 2] {
        return self.map.get(dir).unwrap();
    }

    pub fn get_opp(&self, dir: &Directions) -> Directions {
        return match dir {
            Directions::E => Directions::W,
            Directions::W => Directions::E,
            Directions::N => Directions::S,
            Directions::S => Directions::N,
            Directions::NE => Directions::SW,
            Directions::SW => Directions::NE,
            Directions::NW => Directions::SE,
            Directions::SE => Directions::NW,
        };
    }

    pub fn get_iter(&self) -> Iter<Directions, [i32; 2]> {
        return self.map.iter();
    }
}

pub struct Day4 {}

impl Day4 {
    pub fn get_results() -> [String; 2] {
        let input_matrix: Vec<Vec<char>> = fs::read_to_string("./inputs/day4/day4.txt")
            .unwrap()
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();

        return [Self::part1(&input_matrix), Self::part2(&input_matrix)];
    }

    fn part1(input: &Vec<Vec<char>>) -> String {
        let mut res: i32 = 0;

        for row_idx in 0..input.len() {
            for col_idx in 0..input[row_idx].len() {
                if input[row_idx][col_idx] == 'X' {
                    res += Self::find_xmas_count_at_index(input, &row_idx, &col_idx);
                }
            }
        }

        return res.to_string();
    }

    fn part2(input: &Vec<Vec<char>>) -> String {
        let mut res: i32 = 0;

        for row_idx in 1..(input.len() - 1) {
            for col_idx in 1..(input[row_idx].len() - 1) {
                if input[row_idx][col_idx] == 'A'
                    && Self::is_x_mas_at_index(input, &row_idx, &col_idx)
                {
                    res += 1;
                }
            }
        }

        return res.to_string();
    }

    fn find_xmas_count_at_index(input: &Vec<Vec<char>>, row_idx: &usize, col_idx: &usize) -> i32 {
        let mut count: i32 = 0;
        let remaining: [char; 2] = ['A', 'S'];
        let direction_map: DirectionMap = DirectionMap::new();

        for (_, [col_offset, row_offset]) in direction_map.get_iter() {
            let mut offset_row_idx: i32 = *row_idx as i32 + row_offset;
            let mut offset_col_idx: i32 = *col_idx as i32 + col_offset;

            if is_out_of_bounds(&offset_row_idx, input.len())
                || is_out_of_bounds(&offset_col_idx, input[offset_row_idx as usize].len())
            {
                continue;
            }

            if input[offset_row_idx as usize][offset_col_idx as usize] == 'M' {
                let mut ptr: usize = 0;
                while ptr < 2 {
                    offset_row_idx += row_offset;
                    offset_col_idx += col_offset;

                    if is_out_of_bounds(&offset_row_idx, input.len())
                        || is_out_of_bounds(&offset_col_idx, input[offset_row_idx as usize].len())
                        || input[offset_row_idx as usize][offset_col_idx as usize] != remaining[ptr]
                    {
                        break;
                    }

                    ptr += 1;
                }

                if ptr == 2 {
                    count += 1;
                }
            }
        }

        return count;
    }

    fn is_x_mas_at_index(input: &Vec<Vec<char>>, row_idx: &usize, col_idx: &usize) -> bool {
        let direction_map: DirectionMap = DirectionMap::new();

        if is_mas(
            input,
            *row_idx as i32,
            *col_idx as i32,
            &direction_map,
            &Directions::NW,
        ) && is_mas(
            input,
            *row_idx as i32,
            *col_idx as i32,
            &direction_map,
            &Directions::NE,
        ) {
            return true;
        }

        return false;
    }
}

fn is_out_of_bounds(val: &i32, max_len: usize) -> bool {
    return *val < 0 || *val >= max_len as i32;
}

fn is_mas(
    input: &Vec<Vec<char>>,
    row_idx: i32,
    col_idx: i32,
    dir_map: &DirectionMap,
    dir: &Directions,
) -> bool {
    let [col_offset, row_offset] = dir_map.get_offset(dir);
    let [opp_col_offset, opp_row_offset] = dir_map.get_offset(&dir_map.get_opp(dir));

    let offset_row_idx: i32 = row_idx + row_offset;
    let offset_col_idx: i32 = col_idx + col_offset;

    let opp_offset_row_idx: i32 = row_idx + opp_row_offset;
    let opp_offset_col_idx: i32 = col_idx + opp_col_offset;

    if (input[offset_row_idx as usize][offset_col_idx as usize] == 'M'
        && input[opp_offset_row_idx as usize][opp_offset_col_idx as usize] == 'S')
        || (input[opp_offset_row_idx as usize][opp_offset_col_idx as usize] == 'M'
            && input[offset_row_idx as usize][offset_col_idx as usize] == 'S')
    {
        return true;
    }

    return false;
}

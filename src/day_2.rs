use std::{
    collections::VecDeque,
    fs::File,
    io::{BufRead, BufReader},
    u32,
};

fn find_divisible(line: Vec<u32>) -> (u32, u32) {
    // Assume that the line isn't sorted
    let mut temp = line.clone();
    temp.sort();
    let mut temp: VecDeque<u32> temp.clone().into();

    todo!()
}
fn day_2_part_2(fname: &str) -> u32 {
    let lines = parse_file_to_entries(fname);
    lines.iter().map(|x| {
        let mut temp = x.clone();
        let mut out: (u32, u32);
        temp.sort();
        let mut temp: VecDeque<u32> = temp.clone().into();
        let front = temp.pop_front().unwrap();
        for v in temp {
            match (v % front) == 0 {
                true => {
                    out = (v, front);
                    break;
                }
                false => {
                    // println!("{} and {} -- remainder: {}", front, v, (front % v));
                }
            }
        }
        return out
    }
    ).collect::<Vec<u32>>().iter().sum()
}
pub fn parse_file_to_entries(fname: &str) -> Vec<Vec<u32>> {
    let file = File::open(fname).unwrap();
    let bufreader = BufReader::new(file);
    bufreader
        .lines()
        .map(|x| match x {
            Ok(line) => line
                .split_whitespace()
                .map(|x| match x.parse::<u32>() {
                    Ok(n) => n,
                    Err(_) => todo!(),
                })
                .collect::<Vec<u32>>(),
            Err(_) => todo!(),
        })
        .collect()
}
pub fn min_max(line: Vec<u32>) -> (u32, u32) {
    let mut min = u32::MAX;
    let mut max = u32::MIN;
    line.iter().for_each(|x| {
        match *x < min {
            true => min = *x,
            false => {}
        }
        match *x > max {
            true => max = *x,
            false => {}
        }
    });
    (min, max)
}

pub fn checksum(min_max: (u32, u32)) -> u32 {
    min_max.1 - min_max.0
}
pub fn day_2_part_1(fname: &str) -> u32 {
    parse_file_to_entries("input/day_2.txt")
        .iter()
        .map(|x| min_max(x.clone()))
        .collect::<Vec<(u32, u32)>>()
        .iter()
        .map(|x| checksum(*x))
        .collect::<Vec<u32>>()
        .iter()
        .sum::<u32>()
}

#[cfg(test)]
mod tests {
    use crate::day_1::day_1_part_1;

    use super::*;
    #[test]
    fn test_parse_file_to_entries() {
        let contents = parse_file_to_entries("input/day_2.txt");
        assert_eq!(contents[0].len(), 16);
    }
    #[test]
    fn test_day_2_part_1() {
        println!("Day 2 Part 1 Results: {}", day_2_part_1("input/day_2.txt"));
    }
    #[test]
    fn test_day_2_part_2() {
        println!("Day 2 Part 2 Results: {}", day_2_part_2("input/day_2.txt"));
    }
}

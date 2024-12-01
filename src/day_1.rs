use std::fs::File;
use std::io::Read;

pub fn day_1_part_1(fname: &str) -> u32 {
    let numbers = parse_file_to_numbers(fname);
    println!("{}", numbers.len());
    let mut sum: u32 = 0;
    for i in 0..numbers.len() - 1 {
        match check_neighbor(&numbers[i..=i + 1]) {
            true => sum += numbers[i],
            false => {}
        }
    }
    if numbers[0] == numbers[numbers.len() - 1] {
        sum += numbers[0]
    }
    println!("the sum is {}", sum);
    sum
}
pub fn day_1_part_2(fname: &str) -> u32 {
    let numbers = parse_file_to_numbers(fname);
    let mut sum = 0;
    for i in 0..numbers.len() - 1 {
        match numbers[i] == numbers[calc_half_neighbor(i, numbers.len())] {
            true => sum += numbers[i],
            false => {}
        }
    }
    println!("part 2 sum: {}", sum);
    sum
}

fn parse_file_to_numbers(fname: &str) -> Vec<u32> {
    let mut buffer = String::new();
    let _ = File::open(fname)
        .unwrap()
        .read_to_string(&mut buffer)
        .unwrap();
    buffer
        .chars()
        .map(|x| x.to_digit(10).map(|d| d))
        .filter(|x| x.is_some())
        .flatten()
        .collect()
}
fn calc_half_neighbor(current_address: usize, total_length: usize) -> usize {
    let normal_length = current_address + total_length / 2;
    match normal_length > total_length - 1 {
        true => (normal_length - total_length),
        false => normal_length,
    }
}
fn check_neighbor(slice: &[u32]) -> bool {
    slice[0] == slice[1]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs::File, io::Read};
    #[test]
    fn test_day_1_parse_1() {
        day_1_part_1("input/day_1.txt");
    }
    #[test]
    fn test_day_1_parse_2() {
        day_1_part_2("input/day_1.txt");
    }
    #[test]
    fn test_open_file() {
        let fname = "input/day_1.txt";
        let mut buffer = String::new();
        let _ = File::open(fname)
            .unwrap()
            .read_to_string(&mut buffer)
            .unwrap();
        assert_ne!(buffer.len(), 0);
    }
    #[test]
    fn test_wrap_around() {
        let total_length = 1000;
        let current_position = 600;
        assert_eq!(calc_half_neighbor(current_position, total_length), 100);
    }
    #[test]
    fn test_mid() {
        let total_length = 1000;
        let current_position = 0;
        assert_eq!(calc_half_neighbor(current_position, total_length), 500);
    }
    #[test]
    fn test_zero() {
        let total_length = 1000;
        let current_position = 500;
        assert_eq!(calc_half_neighbor(current_position, total_length), 0);
    }
}

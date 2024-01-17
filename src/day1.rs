
//use std::env;
use std::fs;

pub fn part1() {
    let contents = fs::read_to_string("input/day1.txt").expect("Error while reading file.");
    let lines = contents.split('\n');

    let mut start_digit;
    let mut end_digit;
    let mut sum = 0;
    for line in lines {
        start_digit = '\0';
        end_digit = '\0';
        for char in line.chars() {
            if char.is_ascii_digit() {
                if start_digit == '\0' {
                    start_digit = char;
                    end_digit = char;
                }
                else {
                    end_digit = char;
                }
            }
        }
        sum += start_digit.to_digit(10).unwrap_or(0) * 10 + end_digit.to_digit(10).unwrap_or(0);
    }
    println!("The answer is: {}", sum);
}

use std::fs;

pub fn parts() {
    println!("Results: {} {}", part1(), part2());
}

fn part1() -> u32 {
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
    sum
}

fn check_number(text: &str, index: usize, num: &str) -> bool {
    if text.len() > index && text.len() >= index + num.len() {
        text.as_bytes()[index..(index + num.len())] == num.as_bytes()[..]
    }
    else {
        false
    }
}

fn part2() -> u32 {
    let contents = fs::read_to_string("input/day1.txt").expect("Error while reading file.");
    let lowercase = contents.to_lowercase();
    let lines = lowercase.split('\n');

    let mut digit;
    let mut start_digit;
    let mut end_digit;
    let mut sum = 0;
    let mut char_index;

    for line in lines {
        char_index = 0;
        digit = None;
        start_digit = None;
        end_digit = None;
        while char_index < line.chars().count() {
            match line.as_bytes()[char_index] {
                b'o' => {
                    if check_number(line, char_index, "one") {
                        digit = Some(1);
                    }
                }
                b't' => {
                    if check_number(line, char_index, "two") {
                        digit = Some(2);
                    }
                    else if check_number(line, char_index, "three") {
                        digit = Some(3);
                    }
                }
                b'f' => {
                    if check_number(line, char_index, "four") {
                        digit = Some(4);
                    }
                    else if check_number(line, char_index, "five") {
                        digit = Some(5);
                    }
                }
                b's' => {
                    if check_number(line, char_index, "six") {
                        digit = Some(6);
                    }
                    else if check_number(line, char_index, "seven") {
                        digit = Some(7);
                    }
                }
                b'e' => {
                    if check_number(line, char_index, "eight") {
                        digit = Some(8);
                    }
                }
                b'n' => {
                    if check_number(line, char_index, "nine") {
                        digit = Some(9);
                    }
                }
                n if (b'1'..=b'9').contains(&n) => {
                    digit = (n as char).to_digit(10);
                }
                _ => {}
            }
            char_index += 1;
            if digit.is_some() {
                if start_digit.is_none() {
                    start_digit = digit;
                    end_digit = digit;
                }
                else {
                    end_digit = digit;
                }
            }
        }
        sum += start_digit.unwrap_or(0) * 10 + end_digit.unwrap_or(0);
    }
    sum
}
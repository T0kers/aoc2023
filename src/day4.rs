
use std::fs;

struct MatchInfo {
    gives: u32,
    take: u32,
}

pub fn parts() {
    let contents = fs::read_to_string("input/day4.txt").expect("Error while reading file.");
    let lines = contents.split('\n');

    let mut current_number: u32;
    let mut winning: Vec<u32>;
    let mut iter;
    let mut matches: u32;
    let mut part1_total: u32 = 0;
    let mut part2_total: u32 = 0;
    let mut all_matches: Vec<MatchInfo> = vec![];

    for (line_index, line) in lines.enumerate() {
        if line == "" {continue;}
        iter = line.chars().skip(8);
        winning = vec![];
        current_number = 0;
        while let Some(ch) = iter.next() {
            if ch == ' ' && current_number != 0 {
                winning.push(current_number);
                current_number = 0;
            }
            if ch == '|' {
                current_number = 0;
                break;
            }
            if let Some(n) = ch.to_digit(10) {
                current_number = current_number * 10 + n;
            }
        }
        matches = 0;
        while let Some(ch) = iter.next() {
            if ch == ' ' && current_number != 0 {
                if winning.contains(&current_number) {
                    matches += 1;
                }
                current_number = 0;
            }
            if let Some(n) = ch.to_digit(10) {
                current_number = current_number * 10 + n;
            }
        }
        if winning.contains(&current_number) {
            matches += 1;
        }
        
        let mut matches_index = 0;
        let mut cards: u32 = 1;
        while matches_index < line_index {
            if all_matches[matches_index].gives > 0 {
                all_matches[matches_index].gives -= all_matches[matches_index].take;
                cards += all_matches[matches_index].take;
            }
            matches_index += 1;
        }
        let tot_matches = cards * matches;
        all_matches.push(MatchInfo {
            gives: tot_matches,
            take: cards,
        });
        part2_total += cards;


        if matches > 0 {
            part1_total += u32::pow(2, matches - 1);
        }
    }    
    
    println!("Results: {} {}", part1_total, part2_total);
}
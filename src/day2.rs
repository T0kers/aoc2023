
use std::fs;

#[derive(Debug)]
struct GameInfo {
    r: u32,
    g: u32,
    b: u32,
}

pub fn parts() {
    let contents = fs::read_to_string("input/day2.txt").expect("Error while reading file.");
    let lines = contents.split('\n');
    
    let mut seen_colon;
    let mut goto_next = false;
    let mut num: u32;
    let mut game;

    let mut part1: u32 = 0;
    let mut part2: u32 = 0;
    let mut id: u32 = 1;
    for  line in lines {
        if line == "" {continue;}
        seen_colon = false;
        game = GameInfo {r: 0, g: 0, b: 0};
        num = 0;

        for char in line.chars() {
            if !seen_colon {
                if char == ':' {
                    seen_colon = true;
                    goto_next = true;
                }
                continue;
            }
            if goto_next && char != ' ' {
                continue;
            }
            else {
                goto_next = false;
            }

            if let Some(n) = char.to_digit(10) {
                num = num * 10 + n;
            }
            match char {
                'r' => {
                    if num > game.r {game.r = num;}
                    num = 0;
                    goto_next = true;
                },
                'g' => {
                    if num > game.g {game.g = num;}
                    num = 0;
                    goto_next = true;
                },
                'b' => {
                    if num > game.b {game.b = num;}
                    num = 0;
                    goto_next = true;
                },
                _ => {},
            }
        }
        if game.r <= 12 && game.g <= 13 && game.b <= 14 {
            part1 += id;
        }
        part2 += game.r * game.g * game.b;
        id += 1;
    }
    println!("Result: {}, {}", part1, part2);
}
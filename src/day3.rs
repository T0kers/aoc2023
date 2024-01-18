
use std::{fs, io::Empty};

#[derive(Debug)]
enum Part {
    Number(u8),
    Symbol,
    Empty,
}

pub fn parts() {
    let contents = fs::read_to_string("input/day3.txt").expect("Error while reading file.");
    let lines = contents.split('\n');
    let mut engine = vec![];

    for (row, line) in lines.enumerate() {
        if line == "" {continue;}
        engine.push(Vec::new());
        for char in line.chars() {
            engine[row].push({
                match char {
                    '.' => Part::Empty,
                    '0'..='9' => Part::Number(char.to_digit(10).unwrap() as u8),
                    _ => Part::Symbol,
                }
            })
        }
    }
    for row in engine {
        for part in row {

        }
    }
    println!("{:#?}", engine);

}

use std::{fs, io::Empty};

#[derive(Debug)]
enum Part {
    Number(u8),
    Symbol,
    Empty,
}

fn getPart(i: usize, j: usize, eng: &Vec<Vec<Part>>) -> Option<&Part> {
    if eng.len() > i && eng[i].len() > j {
        Some(&eng[i][j])
    }
    else {
        None
    }
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
    for (i, row) in engine.iter().enumerate() {
        for (j, part) in row.iter().enumerate() {
            'check: for i_check in i-1..=i+1 {
                for j_check in j-1..=j+1 {
                    if let Some(Part::Number(n)) = getPart(i_check, j_check, &engine) {
                        
                    }
                }
            }
        }
    }
    println!("{:#?}", engine);

}
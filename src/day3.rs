
use std::fs;

#[derive(Debug)]
enum Part {
    Number(u8),
    Symbol,
    Gear,
    Empty,
}

fn get_part(i: usize, j: usize, eng: &Vec<Vec<Part>>) -> Option<&Part> {
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
        if line.is_empty() {continue;}
        engine.push(Vec::new());
        for char in line.chars() {
            engine[row].push({
                match char {
                    '.' => Part::Empty,
                    '0'..='9' => Part::Number(char.to_digit(10).unwrap() as u8),
                    '*' => Part::Gear,
                    _ => Part::Symbol,
                }
            })
        }
    }
    let mut current_number: u32 = 0;
    let mut include_number = false;
    let mut sum: u32 = 0;

    let mut gear_sum: u32 = 0;
    let mut gear_ratio: u32;
    let mut gear_number_space;
    let mut gear_number_count: u8;
    let mut gear_check;
    let mut num_finder: usize;
    let mut num_value: u32;

    for (i, row) in engine.iter().enumerate() {
        for (j, _part) in row.iter().enumerate() {
            gear_check = false;
            match get_part(i, j, &engine) {
                Some(Part::Gear) => {
                    if include_number {
                        sum += current_number;
                    }
                    current_number = 0;
                    include_number = false;
                    gear_check = true;
                }
                Some(Part::Symbol) | Some(Part::Empty) | None => {
                    if include_number {
                        sum += current_number;
                    }
                    current_number = 0;
                    include_number = false;

                    continue;
                }
                Some(Part::Number(n)) => {
                    current_number = current_number * 10 + *n as u32;
                }
            }
            gear_ratio = 1;
            gear_number_count = 0;
            'check: for i_check in if i != 0 {i-1..=i+1} else {i..=i+1} {
                gear_number_space = true;
                for j_check in if j != 0 {j-1..=j+1} else {j..=j+1} {
                    if !include_number {
                        if let Some(Part::Symbol) | Some(Part::Gear) = get_part(i_check, j_check, &engine) {
                            include_number = true;
                        }
                    }
                    if gear_check {
                        if gear_number_space {
                            if let Some(Part::Number(_)) = get_part(i_check, j_check, &engine) {
                                gear_number_space = false;

                                gear_number_count += 1;
                                num_finder = j_check;
                                loop {
                                    if num_finder > 0 {
                                        num_finder -= 1;
                                        if let Some(Part::Number(_)) = get_part(i_check, num_finder, &engine) {}
                                        else {
                                            num_finder += 1;
                                            break;
                                        }
                                    }
                                    else {
                                        break;
                                    }
                                }
                                num_value = 0;
                                while let Some(Part::Number(n)) = get_part(i_check, num_finder, &engine) {
                                    num_value = num_value * 10 + *n as u32;
                                    num_finder += 1;
                                }
                                gear_ratio *= num_value;
                            }
                        }
                        else if let Some(Part::Number(_)) = get_part(i_check, j_check, &engine) {}
                        else {
                            gear_number_space = true;
                        }
                    }
                    if include_number && gear_number_count >= 2 {
                        gear_sum += gear_ratio;
                        break 'check;
                    }
                }
            }
        }
    }
    println!("Results: {} {}", sum, gear_sum);

}
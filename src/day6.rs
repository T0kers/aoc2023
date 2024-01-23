
use std::fs;

pub fn parts() {
    let content = fs::read_to_string("input/day6.txt").expect("Error while reading file.");
    let mut lines = content.trim().split('\n');

    let times_p1 = lines.next().unwrap()[12..].split(' ').flat_map(str::parse::<u32>).collect::<Vec<u32>>();
    let distances_p1 = lines.next().unwrap()[12..].split(' ').flat_map(str::parse::<u32>).collect::<Vec<u32>>();



    let mut part1: u32 = 1;
    for (time, distance) in times_p1.iter().zip(distances_p1.iter()) {
        let mut beaten_p1: u32 = 0;
        for vel in 0..*time {
            if (time - vel) * vel > *distance {
                beaten_p1 += 1;
            }
        }
        if beaten_p1 != 0 {part1 *= beaten_p1};
    }

    let nums: Vec<u64> = content.trim().split('\n').map(|line| {
        line[11..].chars()
            .filter(|c| !c.is_whitespace()).collect::<String>().parse::<u64>().unwrap_or(0)
    }).collect();

    for num in &nums {
        println!("{:?}", num);
    }

    let mut part2: u32 = 0;
    for vel in 0..nums[0] {
        if (nums[0] - vel) * vel > nums[1] {
            part2 += 1;
        }
    }

    println!("Results: {} {}", part1, part2);
}
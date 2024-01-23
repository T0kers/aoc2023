
use std::{fs, vec};

#[derive(Debug)]
struct AlmanacRange {
    dest_start: u64,
    source_start: u64,
    range: u64,
}

impl AlmanacRange {
    fn source_end(&self) -> u64 {
        self.source_start + self.range
    }
}

#[derive(Debug)]
#[derive(Clone)]
struct Range {
    start: u64,
    range: u64,
}

impl Range {
    fn new(start: u64, range: u64) -> Range {
        Range {
            start, range
        }
    }

    fn from_end(start: u64, end: u64) -> Range {
        Range {
            start,
            range: end - start,
        }
    }

    fn end(&self) -> u64 {
        self.start + self.range
    }
}


fn eval_rule(ranges_left: &mut Vec<Range>, rule: &AlmanacRange, new_ranges: &mut Vec<Range>) {
    let mut extra_ranges = vec![];
    for range_left in &mut *ranges_left {
        if range_left.range == 0 {continue;}
        if rule.source_start <= range_left.start {
            if range_left.end() <= rule.source_end() {
                new_ranges.push(Range::new((range_left.start - rule.source_start) + rule.dest_start, range_left.range));
                range_left.range = 0;
                continue;
            }
            else if rule.source_end() > range_left.start {
                new_ranges.push(Range::new((range_left.start - rule.source_start) + rule.dest_start, rule.source_end() - range_left.start));
                range_left.range -= rule.source_end() - range_left.start;
                range_left.start = rule.source_end();
            }
        }
        else if rule.source_start < range_left.end() {
            if range_left.end() <= rule.source_end() {
                new_ranges.push(Range::new(rule.dest_start, rule.source_start - range_left.start));
                range_left.range = rule.source_start - range_left.start;
            }
            else {
                new_ranges.push(Range::new(rule.dest_start, rule.range));

                extra_ranges.push(Range::from_end(rule.source_end(), range_left.end()));

                range_left.range = rule.source_start - range_left.start;

            }
        }
    }
    if !extra_ranges.is_empty() {
        eval_rule(&mut extra_ranges, rule, new_ranges);
        ranges_left.append(&mut extra_ranges);
    }
}


pub fn parts() {
    let contents = fs::read_to_string("input/day5.txt").expect("Error while reading file.");
    let mut maps = contents.split("\n\n");

    let seeds = maps.next().unwrap()[7..].trim().split(' ').flat_map(str::parse::<u64>).collect::<Vec<u64>>();
    
    let mut almanac: Vec<Vec<AlmanacRange>> = vec![];
    
    for (i, map) in maps.enumerate() {
        almanac.push(vec![]);
        for line in map.split('\n').skip(1) {
            if line.is_empty() {continue;}
            almanac[i].push({
                let line_nums: Vec<u64> = line.trim().split(' ').flat_map(str::parse::<u64>).collect::<Vec<u64>>();
                if line_nums.len() == 3 {
                    AlmanacRange {
                        dest_start: line_nums[0],
                        source_start: line_nums[1],
                        range: line_nums[2],
                    }
                }
                else {
                    panic!("Error!");
                }
            });
        }
    }
    let mut lowest_part1: u64 = u64::MAX;
    for seed in &seeds {
        let mut value = *seed;
        let mut new_value: u64;
        for map in &almanac {
            new_value = value;
            for line in map {
                if line.source_start <= value && value <= line.source_end() {
                    new_value = (value - line.source_start) + line.dest_start;
                    break;
                }
            }
            value = new_value;
        }
        if value < lowest_part1 {
            lowest_part1 = value;
        }
    }
    let mut ranges: Vec<Range> = seeds.chunks(2).filter_map(|chunk| {
        if chunk.len() == 2 {
            Some(Range{start: chunk[0], range:chunk[1]})
        }
        else {
            None
        }
    }).collect();

    let mut new_ranges: Vec<Range> = vec![];

    for map in &almanac {
        let mut ranges_left: Vec<Range>;
        for range in &ranges {
            ranges_left = vec![range.clone()];
            for rule in map {
                eval_rule(&mut ranges_left, rule, &mut new_ranges);
            }
            new_ranges.append(&mut ranges_left);
        }
        ranges.clear();
        ranges.append(&mut new_ranges);
    }

    let mut lowest_part2: u64 = u64::MAX;
    for range in &ranges {
        if range.range != 0 && range.start != 0 && range.start < lowest_part2  {
            lowest_part2 = range.start;
        }
    }

    println!("Results: {} {}", lowest_part1, lowest_part2);

}
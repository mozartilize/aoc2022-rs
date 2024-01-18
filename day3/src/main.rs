use std::io;

fn part1(lines: impl Iterator<Item = String>) {
    let mut result = vec![];
    lines.for_each(|line| {
        let compartment1 = &line[..line.len() / 2];
        let compartment2 = &line[line.len() / 2..];
        let mut inner_result = vec![];

        for c in compartment1.chars() {
            if compartment2.contains(c) && !inner_result.contains(&c) {
                result.push(c);
                inner_result.push(c);
            }
        }
    });
    let arr: u32 = result
        .iter()
        .map(|c| {
            if c.is_lowercase() {
                (*c as u32) - 96
            } else {
                (*c as u32) - 38
            }
        })
        .sum();
    dbg!(arr);
}

fn part2(lines: impl Iterator<Item = String>) {
    let mut result = vec![];
    for group in lines.collect::<Vec<String>>().chunks(3) {
        let compartment1 = &group[0];
        let compartment2 = &group[1];
        let compartment3 = &group[2];
        let mut inner_result = vec![];

        for c in compartment1.chars() {
            if compartment2.contains(c) && compartment3.contains(c) && !inner_result.contains(&c) {
                result.push(c);
                inner_result.push(c);
            }
        }
    }
    let arr: u32 = result
        .iter()
        .map(|c| {
            if c.is_lowercase() {
                (*c as u32) - 96
            } else {
                (*c as u32) - 38
            }
        })
        .sum();
    dbg!(arr);
}

fn main() {
    let lines = io::stdin().lines();
    let some_day = std::env::args().last();
    match some_day {
        Some(day) => {
            if day == "1" {
                part1(lines.map(|l| l.unwrap()));
            } else if day == "2" {
                part2(lines.map(|l| l.unwrap()));
            }
        }
        None => (),
    }
}

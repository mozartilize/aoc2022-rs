use regex::Regex;
use std::io;

fn main() {
    let lines = io::stdin().lines();
    let some_part = std::env::args().last().unwrap();
    let mut stacks: Vec<Vec<String>> = vec![];
    let mut number_of_stacks = 0;
    let re = Regex::new(r"^move (?<quantity>\d+) from (?<from>\d+) to (?<to>\d+)$").unwrap();
    let mut step = 0;
    for line in lines.map(|l| l.unwrap()) {
        if line == "" {
            step = 1;
            continue;
        }
        if step == 0 {
            // build stacks
            if number_of_stacks == 0 {
                number_of_stacks = line.len() / 4 + 1;
            }
            for i in 0..number_of_stacks {
                if stacks.len() <= i {
                    stacks.push(vec![]);
                }
                let v = &mut stacks[i];
                let idx = i * 3 + i * 1;
                if &line[idx..idx + 3] != "   " && &line[idx + 1..idx + 2] != format!("{}", i + 1) {
                    v.insert(0, line[idx + 1..idx + 2].to_string());
                }
            }
        } else {
            // moving crates
            let Some(caps) = re.captures(&line) else {
                println!("no match {}", line);
                continue;
            };
            let from = (str::parse::<usize>(&caps["from"])).unwrap();
            let to = (str::parse::<usize>(&caps["to"])).unwrap();
            let quantity = (str::parse::<usize>(&caps["quantity"])).unwrap();
            if some_part == "1" {
                part1(&mut stacks, quantity, from, to);
            } else if some_part == "2" {
                part2(&mut stacks, quantity, from, to);
            }
        }
    }
    for s in stacks {
        print!("{}", s[s.len() - 1]);
    }
}

fn part1(stacks: &mut Vec<Vec<String>>, quantity: usize, from: usize, to: usize) {
    for _ in 0..quantity {
        let val = stacks[from - 1].pop().unwrap();
        stacks[to - 1].push(val);
    }
}

fn part2(stacks: &mut Vec<Vec<String>>, quantity: usize, from: usize, to: usize) {
    let mut vals = vec![];
    for _ in 0..quantity {
        let val = stacks[from - 1].pop().unwrap();
        vals.insert(0, val);
    }
    stacks[to - 1].append(&mut vals);
}

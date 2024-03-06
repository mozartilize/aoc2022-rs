use std::{collections::HashSet, io};

#[derive(Debug)]
enum Ops {
    Addx(i32),
    Noop,
}

struct CPU {
    cycle: usize,
    milestones: Vec<usize>,
    idx: usize,
    x: i32,
    saves: Vec<i32>,
    draws: HashSet<usize>,
}

impl CPU {
    fn new(milestones: Vec<usize>) -> Self {
        Self {
            cycle: 0,
            milestones,
            x: 1,
            idx: 0,
            saves: vec![],
            draws: HashSet::new(),
        }
    }
    fn execute(&mut self, ops: Ops) {
        match ops {
            Ops::Addx(val) => {
                self.increase_cycle();
                self.increase_cycle();
                self.x += val;
            }
            Ops::Noop => {
                self.increase_cycle();
            }
        }
    }

    fn increase_cycle(&mut self) {
        self.cycle += 1;
        let sprite = vec![self.x - 1, self.x, self.x + 1];
        let pos = if (self.cycle as i32) % 40 == 0 {
            40
        } else {
            (self.cycle as i32) % 40
        } - 1;
        if sprite.iter().any(|p| pos == *p) {
            self.draws.insert(self.cycle - 1);
        };
        if self.idx <= self.milestones.len() - 1 {
            if self.cycle == self.milestones[self.idx] {
                let val = (self.cycle as i32) * self.x;
                self.saves.push(val);
            } else if self.cycle > self.milestones[self.idx] {
                self.idx += 1;
            }
        };
    }
}

fn main() {
    let lines = io::stdin().lines();
    let mut cpu = CPU::new(vec![20, 60, 100, 140, 180, 220]);
    for ops in lines.map(|l| l.unwrap()).map(|l| {
        let mut split = l.split(" ");
        let ops = split.next().unwrap();
        let m = match ops {
            "addx" => Ops::Addx(str::parse::<i32>(split.next().unwrap()).unwrap()),
            "noop" => Ops::Noop,
            &_ => panic!("invalid char {}", ops),
        };
        m
    }) {
        cpu.execute(ops);
    }
    let signal_strength: i32 = cpu.saves.iter().sum();
    dbg!(signal_strength);
    dbg!(cpu.cycle);
    for i in 0..cpu.cycle {
        if cpu.draws.contains(&i) {
            print!("#");
        } else {
            print!(" ");
        }
        if (i+1) % 40 == 0 {
            println!();
        }
    }
}

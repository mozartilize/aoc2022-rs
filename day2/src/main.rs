use std::{collections::HashMap, io};

use phf::phf_map;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
enum RPS {
    Rock,
    Paper,
    Sissor,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
enum STATE {
    WIN,
    LOSE,
    DRAW,
}

static MAP1: phf::Map<&'static str, RPS> = phf_map! {
    "A" => RPS::Rock,
    "B" => RPS::Paper,
    "C" => RPS::Sissor,
    "X" => RPS::Rock,
    "Y" => RPS::Paper,
    "Z" => RPS::Sissor,
};

static MAP2: phf::Map<&'static str, STATE> = phf_map! {
    "X" => STATE::LOSE,
    "Y" => STATE::DRAW,
    "Z" => STATE::WIN,
};

fn cal_1(c: Vec<RPS>) -> u32 {
    let y: HashMap<(RPS, RPS), u32> = HashMap::from([
        ((RPS::Rock, RPS::Rock), 3 + 1),
        ((RPS::Rock, RPS::Paper), 6 + 2),
        ((RPS::Rock, RPS::Sissor), 0 + 3),
        ((RPS::Paper, RPS::Rock), 0 + 1),
        ((RPS::Paper, RPS::Paper), 3 + 2),
        ((RPS::Paper, RPS::Sissor), 6 + 3),
        ((RPS::Sissor, RPS::Rock), 6 + 1),
        ((RPS::Sissor, RPS::Paper), 0 + 2),
        ((RPS::Sissor, RPS::Sissor), 3 + 3),
    ]);
    y.get(&(c[0].clone(), c[1].clone())).unwrap().clone()
}

fn cal_2(c: (RPS, STATE)) -> (RPS, RPS) {
    let y: HashMap<(RPS, STATE), (RPS, RPS)> = HashMap::from([
        ((RPS::Rock, STATE::WIN), (RPS::Rock, RPS::Paper)),
        ((RPS::Rock, STATE::DRAW), (RPS::Rock, RPS::Rock)),
        ((RPS::Rock, STATE::LOSE), (RPS::Rock, RPS::Sissor)),
        ((RPS::Paper, STATE::WIN), (RPS::Paper, RPS::Sissor)),
        ((RPS::Paper, STATE::DRAW), (RPS::Paper, RPS::Paper)),
        ((RPS::Paper, STATE::LOSE), (RPS::Paper, RPS::Rock)),
        ((RPS::Sissor, STATE::WIN), (RPS::Sissor, RPS::Rock)),
        ((RPS::Sissor, STATE::DRAW), (RPS::Sissor, RPS::Sissor)),
        ((RPS::Sissor, STATE::LOSE), (RPS::Sissor, RPS::Paper)),
    ]);
    y.get(&(c.0.clone(), c.1.clone())).unwrap().clone()
}

fn main() {
    let some_day = std::env::args().last();
    let lines = io::stdin().lines();
    match some_day {
        Some(day) => {
            if day == "1" {
                let sum: u32 = lines
                    .map(|l| l.unwrap())
                    .map(|l| l.split(" ").map(|c| MAP1.get(c).unwrap().clone()).collect())
                    .map(|pair| cal_1(pair))
                    .sum();
                dbg!(sum);
            } else if day == "2" {
                let sum: u32 = lines
                    .map(|l| l.unwrap())
                    .map(|l| {
                        let vals: Vec<&str> = l.split(" ").collect();
                        (
                            MAP1.get(vals[0]).unwrap().clone(),
                            MAP2.get(vals[1]).unwrap().clone(),
                        )
                    })
                    .map(|c| cal_2(c))
                    .map(|c| cal_1(vec![c.0, c.1]))
                    .sum();
                dbg!(sum);
            }
        },
        None => ()
    }
}

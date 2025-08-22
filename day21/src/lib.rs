use std::{
    collections::{HashMap, VecDeque},
    io::{Lines, StdinLock},
};

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[derive(Debug)]
enum Op {
    ADD,
    SUB,
    MUL,
    DIV,
}

#[derive(Debug)]
pub struct Equation {
    pub left: usize,
    pub right: usize,
    op: Op,
}

fn calculate(val1: f64, val2: f64, op: &Op) -> f64 {
    match op {
        Op::ADD => val1 + val2,
        Op::SUB => val1 - val2,
        Op::MUL => val1 * val2,
        Op::DIV => val1 / val2,
    }
}

#[derive(Debug)]
pub enum MonkeyJob {
    EQUATION(Equation),
    LITERAL(f64),
}

pub fn parse(lines: Lines<StdinLock<'static>>) -> (Vec<MonkeyJob>, usize, usize) {
    let mut v = Vec::new();
    let mut name_idxes = HashMap::new();
    lines.enumerate().for_each(|(lineno, lr)| {
        let l = lr.unwrap();
        let name = String::from(&l[0..4]);
        let job_desc = &l[6..];
        let literal_parse_result = str::parse::<u32>(job_desc);
        if literal_parse_result.is_ok() {
            if let Some(name_idx) = name_idxes.get(&name) {
                v[*name_idx] = MonkeyJob::LITERAL(literal_parse_result.unwrap().into());
            } else {
                v.push(MonkeyJob::LITERAL(literal_parse_result.unwrap().into()));
                name_idxes.insert(name, v.len() - 1);
            };
        } else {
            let name1 = String::from(&job_desc[0..4]);
            let left = if let Some(left) = name_idxes.get(&name1) {
                left.to_owned()
            } else {
                v.push(MonkeyJob::LITERAL(0.0));
                name_idxes.insert(name1, v.len() - 1);
                v.len() - 1
            };
            let op_desc = &job_desc[5..6];
            let op = match op_desc {
                "+" => Op::ADD,
                "-" => Op::SUB,
                "*" => Op::MUL,
                "/" => Op::DIV,
                _ => panic!(),
            };
            let name2 = String::from(&job_desc[7..]);
            let right = if let Some(right) = name_idxes.get(&name2) {
                right.to_owned()
            } else {
                v.push(MonkeyJob::LITERAL(0.0));
                name_idxes.insert(name2, v.len() - 1);
                v.len() - 1
            };

            if let Some(name_idx) = name_idxes.get(&name) {
                v[*name_idx] = MonkeyJob::EQUATION(Equation { left, right, op });
            } else {
                v.push(MonkeyJob::EQUATION(Equation { left, right, op }));
                name_idxes.insert(name, v.len() - 1);
            };
        }
    });
    let root_idx = name_idxes.get(&"root".to_string()).unwrap();
    let humn_idx = name_idxes.get(&"humn".to_string()).unwrap();
    // dbg!(&name_idxes);
    (v, root_idx.to_owned(), humn_idx.to_owned())
}

pub fn part_1(jobs: &Vec<MonkeyJob>) -> HashMap<usize, f64> {
    let mut results = HashMap::new();

    let mut v = VecDeque::new();
    for (idx, job) in jobs.iter().enumerate() {
        match job {
            MonkeyJob::EQUATION(eq) => {
                v.push_back((idx, eq));
            }
            MonkeyJob::LITERAL(v) => {
                results.insert(idx, *v);
            }
        }
    }

    while let Some((idx, eq)) = v.pop_front() {
        match (results.get(&eq.left), results.get(&eq.right)) {
            (Some(left_val), Some(right_val)) => {
                results.insert(idx, calculate(*left_val, *right_val, &eq.op));
            }
            _ => {
                v.push_back((idx, eq));
            }
        }
    }
    results
}

fn find_pattern(v: &mut Vec<MonkeyJob>, ridx: usize, hidx: usize) {
    let mut hval: f64 = 1.0;
    loop {
        v[hidx] = MonkeyJob::LITERAL(hval);
        let r = part_1(&v);
        let req = &v[ridx];
        match req {
            MonkeyJob::EQUATION(eq) => {
                if r.get(&eq.left).unwrap().fract() == 0.0 {
                    println!(
                        "{}, {}, {}, {}",
                        hval,
                        r.get(&eq.left).unwrap(),
                        r.get(&eq.right).unwrap(),
                        r.get(&eq.left).unwrap() - r.get(&eq.right).unwrap()
                    );
                    println!();
                }
                if (r.get(&eq.left).unwrap() == r.get(&eq.right).unwrap()) {
                    break;
                }
            }
            _ => (),
        };
        hval += 1.0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

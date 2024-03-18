use std::io;

#[derive(Debug, Copy, Clone)]
enum Ops {
    NotSet,
    Add(i64),
    Mul(i64),
    Sub(i64),
    Div(i64),
}

impl Default for Ops {
    fn default() -> Self {
        Self::NotSet
    }
}

#[derive(Debug, Default)]
struct Test {
    div_by: i64,
    truthy_mk_idx: usize,
    falsy_mk_idx: usize,
}

#[derive(Debug, Default)]
struct Monkey {
    items: Vec<i64>,
    tmp_items: Vec<i64>,
    ops: Ops,
    test: Test,
}

fn run(monkeys: &mut Vec<Monkey>, rounds: u8) -> Vec<usize> {
    let mut result = vec![0; monkeys.len()];
    for r in 0..rounds {
        println!("=============round {}=============", r);
        for idx in 0..monkeys.len() {
            let monk = &monkeys[idx];
            let truthy_mk_idx = monk.test.truthy_mk_idx;
            let falsy_mk_idx = monk.test.falsy_mk_idx;
            let ops = monk.ops;
            let div_by = monk.test.div_by;
            dbg!(&monk.items);
            result[idx] += monk.items.len();
            for item in monk.items.clone() {
                let a = match ops {
                    Ops::Add(val) => if val == -1 { item + item } else { item + val },
                    Ops::Sub(val) => if val == -1 { item - item } else { item - val },
                    Ops::Mul(val) => if val == -1 { item * item } else { item * val },
                    Ops::Div(val) => if val == -1 { item / item } else { item / val },
                    Ops::NotSet => panic!("fuck!"),
                };
                let b = a / 3;
                if b % div_by == 0 {
                    if idx < truthy_mk_idx {
                        monkeys[truthy_mk_idx].items.push(b);
                    } else if idx > truthy_mk_idx{
                        monkeys[truthy_mk_idx].tmp_items.push(b);
                    }
                } else {
                    if idx < falsy_mk_idx {
                        monkeys[falsy_mk_idx].items.push(b);
                    } else if idx > falsy_mk_idx{
                       monkeys[falsy_mk_idx].tmp_items.push(b);
                    }
                };
            };
        }
        for idx in 0..monkeys.len() {
            let monk = &mut monkeys[idx];
            monk.items = (*monk.tmp_items).to_vec();
            monk.tmp_items = vec![];
        }
    }
    result
}

fn main() {
    let mut monkeys: Vec<Monkey> = vec![];
    let lines = io::stdin().lines();
    let mut idx = 0;
    for line in lines.map(|l| l.unwrap()) {
        let line = line.trim();
        if line.trim().starts_with("Monkey") {
            let idx_str = line.trim_start_matches("Monkey ").trim_end_matches(":");
            let monkey = Monkey::default();
            monkeys.push(monkey);
            idx = str::parse::<usize>(idx_str).unwrap();
        } else if line.trim().starts_with("Starting items") {
            let monkey = &mut monkeys[idx];
            let items_str = line.trim_start_matches("Starting items: ");
            monkey.items.extend(
                items_str
                    .split(", ")
                    .map(|item_str| str::parse::<i64>(item_str).unwrap())
                    .collect::<Vec<_>>(),
            );
        } else if line.trim().starts_with("Operation") {
            let monkey = &mut monkeys[idx];
            let ops_str = line.trim_start_matches("Operation: new = old ");
            if ops_str.starts_with("*") {
                let val_str = ops_str.trim_start_matches("* ");
                let val: i64;
                if val_str == "old" {
                    val = -1;
                } else {
                    val = str::parse(val_str).unwrap();
                }
                monkey.ops = Ops::Mul(val);
            } else if ops_str.starts_with("/") {
                let val_str = ops_str.trim_start_matches("/ ");
                let val: i64;
                if val_str == "old" {
                    val = -1;
                } else {
                    val = str::parse(val_str).unwrap();
                }
                monkey.ops = Ops::Div(val);
            } else if ops_str.starts_with("+") {
                let val_str = ops_str.trim_start_matches("+ ");
                let val: i64;
                if val_str == "old" {
                    val = -1;
                } else {
                    val = str::parse(val_str).unwrap();
                }
                monkey.ops = Ops::Add(val);
            } else if ops_str.starts_with("-") {
                let val_str = ops_str.trim_start_matches("- ");
                let val: i64;
                if val_str == "old" {
                    val = -1;
                } else {
                    val = str::parse(val_str).unwrap();
                }
                monkey.ops = Ops::Sub(val);
            }
        } else if line.trim().starts_with("Test") {
            let monkey = &mut monkeys[idx];
            let div_by_str = line.trim_start_matches("Test: divisible by ");
            monkey.test.div_by = str::parse(div_by_str).unwrap();
        } else if line.trim().starts_with("If true") {
            let monkey = &mut monkeys[idx];
            let truthy_mk_idx_str = line.trim_start_matches("If true: throw to monkey ");
            monkey.test.truthy_mk_idx = str::parse(truthy_mk_idx_str).unwrap();
        } else if line.trim().starts_with("If false") {
            let monkey = &mut monkeys[idx];
            let falsy_mk_idx_str = line.trim_start_matches("If false: throw to monkey ");
            monkey.test.falsy_mk_idx = str::parse(falsy_mk_idx_str).unwrap();
        }
    }
    dbg!(&monkeys);
    dbg!(run(&mut monkeys, 20));
}

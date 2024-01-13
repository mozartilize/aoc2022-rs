use std::io;

fn main() {
    let mut values: Vec<u32> = vec![];
    let mut newline_cnt = 0;
    let lines = io::stdin().lines();
    for line in lines {
        let curr_line = line.unwrap();
        if curr_line == "" {
            newline_cnt += 1;
            if newline_cnt == 2 {
                break;
            }
            values.push(0);
        } else {
            newline_cnt = 0;
            let value = (str::parse::<u32>(&curr_line)).unwrap();
            let len = values.len();
            match values.last() {
                Some(_) => values[len - 1] += value,
                None => (),
            };
        }
    }
    dbg!(values.iter().max());
    values.sort_by(|a, b| b.cmp(a));
    dbg!(values[..3].iter().sum::<u32>());
}
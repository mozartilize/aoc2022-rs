use std::io;

fn part1(lines: impl Iterator<Item = String>) {
    let cnt = lines
        .map(|l| {
            l.split(",")
                .map(|sections| {
                    sections
                        .split("-")
                        .map(|s| str::parse::<u8>(s).unwrap())
                        .collect()
                })
                .collect()
        })
        .map(|parts: Vec<Vec<u8>>| {
            let section1st = &parts[0];
            let section2nd = &parts[1];
            if (section1st[0] >= section2nd[0] && section1st[1] <= section2nd[1])
                || (section2nd[0] >= section1st[0] && section2nd[1] <= section1st[1])
            {
                true
            } else {
                false
            }
        })
        .filter(|x| *x).count();
    dbg!(cnt);
}

fn part2(lines: impl Iterator<Item = String>) {
    let cnt = lines
        .map(|l| {
            l.split(",")
                .map(|sections| {
                    sections
                        .split("-")
                        .map(|s| str::parse::<u8>(s).unwrap())
                        .collect()
                })
                .collect()
        })
        .map(|parts: Vec<Vec<u8>>| {
            let section1st = &parts[0];
            let section2nd = &parts[1];
            if (section2nd[0] >= section1st[0] && section2nd[0] <= section1st[1])
                || (section1st[0] >= section2nd[0] && section1st[0] <= section2nd[1])
            {
                true
            } else {
                false
            }
        })
        .filter(|x| *x).count();
    dbg!(cnt);
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

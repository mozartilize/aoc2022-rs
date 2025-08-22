use std::io;

use day22::{parse_path, turn, Grid};

fn main() {
    let lines = io::stdin()
        .lines()
        .map(|l| l.unwrap())
        .collect::<Vec<String>>();
    let max_line_len = lines[0..lines.len() - 2]
        .iter()
        .map(|l| l.len())
        .max()
        .unwrap();
    let grid = lines[0..lines.len() - 2]
        .iter()
        .map(|l| {
            format!("{:<width$}", l, width = max_line_len)
                .split("")
                .filter_map(|c| if c != "" { Some(c.to_string()) } else { None })
                .collect::<Vec<String>>()
        })
        // .collect::<Vec<Vec<String>>>();
    // dbg!(grid);
        .flatten()
        .collect::<Vec<String>>();
    let grid = Grid::new(grid, max_line_len);
    let px = grid.arr.iter().position(|x| x.as_str()!=" ").unwrap() as i32;
    dbg!(px);
    let mut p = [px, 0];
    let mut p2 = p.clone();
    let mut dir = 0;
    let path = parse_path(&lines[lines.len()-1]);
    for guide in path.clone() {
        match guide.as_str() {
            "R" => dir = turn(dir, &guide),
            "L" => dir = turn(dir, &guide),
            _ => {
                let steps = str::parse::<u32>(&guide).unwrap();
                p = grid.walk(&p, dir, steps);
            }
        }
    }
    dbg!(p, dir);
    dbg!(1000*(p[1]+1)+4*(p[0]+1)+dir);
    dbg!(&p2);
    grid.part2(&mut p2, &path);
}

use std::io;

use day23::{Grid};

fn main() {
    let lines = io::stdin()
        .lines()
        .map(|l| l.unwrap())
        .collect::<Vec<String>>();
    println!("{:?}", lines);
    let max_line_len = lines[0..lines.len()]
        .iter()
        .map(|l| l.len())
        .max()
        .unwrap();
    println!("{max_line_len}");
    let grid = lines[0..lines.len()]
        .iter()
        .map(|l| {
            format!("{:<width$}", l, width = max_line_len)
                .split("")
                .filter_map(|c| if c != "" { Some(c.to_string()) } else { None })
                .collect::<Vec<String>>()
        })
        .collect::<Vec<Vec<String>>>();
    // dbg!(grid);
        // .flatten()
        // .collect::<Vec<String>>();
    println!("{:?}", grid);
    let mut grid = Grid::new(grid, max_line_len);
    // println!("{:?}", grid.elves);
    let r = grid.run();
    // println!("{:?}", grid.elves);
    println!("{r}");
}

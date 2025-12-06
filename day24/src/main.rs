use day24::{Blizzard, Grid};
use std::io;

fn main() {
    let lines = io::stdin()
        .lines()
        .map(|l| l.unwrap())
        .collect::<Vec<String>>();
    let max_line_len = lines[0..lines.len()].iter().map(|l| l.len()).max().unwrap();
    let grid = lines[0..lines.len()]
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
    let grid = Grid::new(&grid, max_line_len.try_into().unwrap());
    // let mut bz = Blizzard::new(30, ">");
    // grid.blizzard_step(&mut bz);
    // println!("{:?}", bz);
    // let mut bzs = grid.blizzards.clone();
    // grid.print_grid(&bzs);
    // for i in 0..3 {
    //     bzs = grid.blizzards_step(&mut bzs);
    //     grid.print_grid(&bzs);
    // }
    let x = grid.run2();
    println!("{}", x);
}

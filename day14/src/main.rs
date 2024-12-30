use day14::{Grid, Point};

fn main() {
    let mut builder = Grid::builder();

    for line_result in std::io::stdin().lines() {
        let line = line_result.unwrap();
        line.split(" -> ").map(|point_str| {
            let mut xy = point_str.split(",").map(|s| s.parse::<u32>().unwrap());
            let x = xy.next().unwrap();
            let y = xy.next().unwrap();
            Point{x, y}
        }).reduce(|p1, p2| {
            builder.store_rocks_from_range(p1, p2.clone());
            p2
        });
    }
    let mut grid = builder.finish();
    dbg!(&grid.max_y, &grid.floor_y);
    let mut grid2 = grid.clone();
    let cnt = grid.run(Point{x: 500, y: 0});
    dbg!(cnt);
    
    let cnt = grid2.run_part2(Point{x: 500, y: 0});
    dbg!(cnt);
}
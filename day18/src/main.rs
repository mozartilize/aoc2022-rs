use day18::{Database, ThreeDPoint};

fn main() {
    let mut db = Database::new();
    for line_result in std::io::stdin().lines() {
        let line = line_result.unwrap();
        let coords = line.split(",").map(|s| s.parse::<i64>().unwrap()).collect::<Vec<_>>();
        db.add_point(ThreeDPoint{x: coords[0], y: coords[1], z: coords[2]});
    }
    // part 1
    dbg!(db.total_uncovered_sides());
    // part 2
    dbg!(db.total_exterior_sides());
}
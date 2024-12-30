use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}

#[derive(Debug, Clone)]
pub enum Material {
    Rock,
    Sand,
}

#[derive(Debug, Clone)]
pub struct Grid {
    pub points: HashMap<Point, Material>,
    pub max_y: u32,
    pub floor_y: u32,
}

impl Grid {
    pub fn builder() -> GridBuilder {
        GridBuilder::new()
    }

    pub fn run(&mut self, source_of_sand: Point) -> u32 {
        let mut cnt: u32 = 0;
        loop {
            let sand = self.fall_a_sand_until_rest(&source_of_sand);
            if self.points.get(&sand).is_none() {
                break;
            }
            cnt += 1;
        }
        return cnt;
    }

    pub fn run_part2(&mut self, source_of_sand: Point) -> u32 {
        let mut cnt: u32 = 0;
        loop {
            let sand = self.fall_a_sand_until_rest_part2(&source_of_sand);
            cnt += 1;
            if sand == source_of_sand {
                break;
            }
        }
        return cnt;
    }

    fn fall_a_sand_until_rest(&mut self, source_of_sand: &Point) -> Point {
        let mut sand = Point {
            x: source_of_sand.x,
            y: source_of_sand.y + 1,
        };
        loop {
            if sand.y + 1 > self.max_y {
                break;
            }
            if self
                .points
                .get(&Point {
                    x: sand.x,
                    y: sand.y + 1,
                })
                .is_none()
            {
                sand.y += 1;
            } else if self
                .points
                .get(&Point {
                    x: sand.x - 1,
                    y: sand.y + 1,
                })
                .is_none()
            {
                sand.x -= 1;
                sand.y += 1;
            } else if self
                .points
                .get(&Point {
                    x: sand.x + 1,
                    y: sand.y + 1,
                })
                .is_none()
            {
                sand.x += 1;
                sand.y += 1;
            } else {
                self.points.insert(sand.clone(), Material::Sand);
                break;
            }
        }
        return sand;
    }

    fn fall_a_sand_until_rest_part2(&mut self, source_of_sand: &Point) -> Point {
        let mut sand = source_of_sand.clone();
        loop {
            if sand.y == self.floor_y - 1 {
                self.points.insert(sand.clone(), Material::Sand);
                break;
            }
            if self
                .points
                .get(&Point {
                    x: sand.x,
                    y: sand.y + 1,
                })
                .is_none()
            {
                sand.y += 1;
            } else if self
                .points
                .get(&Point {
                    x: sand.x - 1,
                    y: sand.y + 1,
                })
                .is_none()
            {
                sand.x -= 1;
                sand.y += 1;
            } else if self
                .points
                .get(&Point {
                    x: sand.x + 1,
                    y: sand.y + 1,
                })
                .is_none()
            {
                sand.x += 1;
                sand.y += 1;
            } else {
                self.points.insert(sand.clone(), Material::Sand);
                break;
            }
            if sand == *source_of_sand {
                self.points.insert(sand.clone(), Material::Sand);
                break;
            }
        }
        return sand;
    }
}

pub struct GridBuilder {
    grid: Grid,
}

impl GridBuilder {
    pub fn new() -> Self {
        GridBuilder {
            grid: Grid {
                points: HashMap::new(),
                max_y: u32::default(),
                floor_y: u32::default(),
            },
        }
    }
    pub fn store_rocks_from_range(&mut self, p1: Point, p2: Point) -> &Self {
        for p in yeild_points_in_range(p1, p2) {
            if p.y > self.grid.max_y {
                self.grid.max_y = p.y;
                self.grid.floor_y = p.y + 2;
            }
            self.grid.points.insert(p, Material::Rock);
        }
        return self;
    }

    pub fn finish(self) -> Grid {
        let _grid = self.grid.clone();
        drop(self);
        return _grid;
    }
}

fn yeild_points_in_range(p1: Point, p2: Point) -> Vec<Point> {
    if (p1.x != p2.x) && (p1.y != p2.y) {
        return vec![];
    } else if p1.x == p2.x {
        if p1.y > p2.y {
            return (p2.y..=p1.y).map(|yy| Point { x: p1.x, y: yy }).collect();
        } else {
            return (p1.y..=p2.y).map(|yy| Point { x: p1.x, y: yy }).collect();
        }
    } else {
        if p1.x > p2.x {
            return (p2.x..=p1.x).map(|xx| Point { x: xx, y: p1.y }).collect();
        } else {
            return (p1.x..=p2.x).map(|xx| Point { x: xx, y: p1.y }).collect();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder() {
        let mut builder = Grid::builder();
        builder.store_rocks_from_range(Point{x:498, y:4}, Point{x:498, y:6});
        builder.store_rocks_from_range(Point{x:498, y:6}, Point{x:496, y:6});
        builder.store_rocks_from_range(Point{x:503, y:4}, Point{x:502, y:4});
        builder.store_rocks_from_range(Point{x:502, y:4}, Point{x:502, y:9});
        builder.store_rocks_from_range(Point{x:502, y:9}, Point{x:494, y:9});
        let grid = builder.finish();
        assert!(grid.points.contains_key(&Point{x:502, y:9}));
        assert!(grid.points.contains_key(&Point{x:500, y:9}));
    }

    #[test]
    fn test_part_1() {
        let mut builder = Grid::builder();
        builder.store_rocks_from_range(Point{x:498, y:4}, Point{x:498, y:6});
        builder.store_rocks_from_range(Point{x:498, y:6}, Point{x:496, y:6});
        builder.store_rocks_from_range(Point{x:503, y:4}, Point{x:502, y:4});
        builder.store_rocks_from_range(Point{x:502, y:4}, Point{x:502, y:9});
        builder.store_rocks_from_range(Point{x:502, y:9}, Point{x:494, y:9});
        let mut grid = builder.finish();
        let cnt = grid.run(Point{x: 500, y: 0});
        assert_eq!(cnt, 24);
    }

    #[test]
    fn test_part_2() {
        let mut builder = Grid::builder();
        builder.store_rocks_from_range(Point{x:498, y:4}, Point{x:498, y:6});
        builder.store_rocks_from_range(Point{x:498, y:6}, Point{x:496, y:6});
        builder.store_rocks_from_range(Point{x:503, y:4}, Point{x:502, y:4});
        builder.store_rocks_from_range(Point{x:502, y:4}, Point{x:502, y:9});
        builder.store_rocks_from_range(Point{x:502, y:9}, Point{x:494, y:9});
        let mut grid = builder.finish();
        let cnt = grid.run_part2(Point{x: 500, y: 0});
        assert_eq!(cnt, 93);
    }
}
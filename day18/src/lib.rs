use std::{
    cmp::{max, min},
    collections::{HashMap, HashSet, VecDeque},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ThreeDPoint {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

impl ThreeDPoint {
    fn neighbors(&self) -> Vec<ThreeDPoint> {
        vec![
            ThreeDPoint {
                x: self.x + 1,
                y: self.y,
                z: self.z,
            },
            ThreeDPoint {
                x: self.x - 1,
                y: self.y,
                z: self.z,
            },
            ThreeDPoint {
                x: self.x,
                y: self.y + 1,
                z: self.z,
            },
            ThreeDPoint {
                x: self.x,
                y: self.y - 1,
                z: self.z,
            },
            ThreeDPoint {
                x: self.x,
                y: self.y,
                z: self.z + 1,
            },
            ThreeDPoint {
                x: self.x,
                y: self.y,
                z: self.z - 1,
            },
        ]
    }
}

struct Sides {
    // x
    left: Option<usize>,
    right: Option<usize>,
    // y
    up: Option<usize>,
    down: Option<usize>,
    // z
    front: Option<usize>,
    behind: Option<usize>,
}

impl Sides {
    fn uncovered_sides_cnt(&self) -> usize {
        [
            self.left,
            self.right,
            self.up,
            self.down,
            self.front,
            self.behind,
        ]
        .iter()
        .filter(|side| side.is_none())
        .count()
    }
}

#[derive(Default)]
pub struct Database {
    points: Vec<ThreeDPoint>,
    xyz_map: HashMap<(i64, i64, i64), usize>,
    sides_map: HashMap<usize, Sides>,
    xmin: i64,
    xmax: i64,
    ymin: i64,
    ymax: i64,
    zmin: i64,
    zmax: i64,
}

impl Database {
    pub fn new() -> Self {
        let mut db = Database::default();
        // Determine the bounding box with padding
        (db.xmin, db.ymin, db.zmin) = (i64::MAX, i64::MAX, i64::MAX);
        (db.xmax, db.ymax, db.zmax) = (i64::MIN, i64::MIN, i64::MIN);
        db
    }

    pub fn total_exterior_sides(&self) -> usize {
        let xmin = self.xmin - 1;
        let xmax = self.xmax + 1;
        let ymin = self.ymin - 1;
        let ymax = self.ymax + 1;
        let zmin = self.zmin - 1;
        let zmax = self.zmax + 1;

        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back(ThreeDPoint {
            x: xmin,
            y: ymin,
            z: zmin,
        });

        // Flood fill to mark reachable outside air
        while let Some(point) = queue.pop_front() {
            if visited.contains(&point) || self.points.contains(&point) {
                continue;
            }
            visited.insert(point);

            for neighbor in point.neighbors() {
                if neighbor.x < xmin
                    || neighbor.x > xmax
                    || neighbor.y < ymin
                    || neighbor.y > ymax
                    || neighbor.z < zmin
                    || neighbor.z > zmax
                {
                    continue;
                }
                if !visited.contains(&neighbor) && !self.points.contains(&neighbor) {
                    queue.push_back(neighbor);
                }
            }
        }

        // Count point faces that are adjacent to external air
        let mut surface_area = 0;
        for point in &self.points {
            for neighbor in point.neighbors() {
                if visited.contains(&neighbor) {
                    surface_area += 1;
                }
            }
        }

        surface_area
    }

    pub fn total_uncovered_sides(&self) -> usize {
        let mut total = 0;
        for p_idx in 0..self.points.len() {
            total += self.sides_map.get(&p_idx).unwrap().uncovered_sides_cnt();
        }
        total
    }

    pub fn add_point(&mut self, p: ThreeDPoint) {
        let idx = self.points.len();
        self.xmin = min(self.xmin, p.x);
        self.xmax = max(self.xmax, p.x);
        self.ymin = min(self.ymin, p.y);
        self.ymax = max(self.ymax, p.y);
        self.zmin = min(self.zmin, p.z);
        self.zmax = max(self.zmax, p.z);
        self.xyz_map.insert((p.x, p.y, p.z), idx);
        let sides = Sides {
            left: self.get_side_left(&p),
            right: self.get_side_right(&p),
            up: self.get_side_up(&p),
            down: self.get_side_down(&p),
            front: self.get_side_front(&p),
            behind: self.get_side_behind(&p),
        };
        if let Some(other_idx) = sides.left {
            let other_sides = self.sides_map.get_mut(&other_idx).unwrap();
            other_sides.right = Some(idx);
        }
        if let Some(other_idx) = sides.right {
            let other_sides = self.sides_map.get_mut(&other_idx).unwrap();
            other_sides.left = Some(idx);
        }
        if let Some(other_idx) = sides.up {
            let other_sides = self.sides_map.get_mut(&other_idx).unwrap();
            other_sides.down = Some(idx);
        }
        if let Some(other_idx) = sides.down {
            let other_sides = self.sides_map.get_mut(&other_idx).unwrap();
            other_sides.up = Some(idx);
        }
        if let Some(other_idx) = sides.front {
            let other_sides = self.sides_map.get_mut(&other_idx).unwrap();
            other_sides.behind = Some(idx);
        }
        if let Some(other_idx) = sides.behind {
            let other_sides = self.sides_map.get_mut(&other_idx).unwrap();
            other_sides.front = Some(idx);
        }
        self.sides_map.insert(idx, sides);
        self.points.push(p);
    }

    fn get_side_left(&self, p: &ThreeDPoint) -> Option<usize> {
        return self.xyz_map.get(&(p.x - 1, p.y, p.z)).copied();
    }

    fn get_side_right(&self, p: &ThreeDPoint) -> Option<usize> {
        return self.xyz_map.get(&(p.x + 1, p.y, p.z)).copied();
    }

    fn get_side_up(&self, p: &ThreeDPoint) -> Option<usize> {
        return self.xyz_map.get(&(p.x, p.y + 1, p.z)).copied();
    }

    fn get_side_down(&self, p: &ThreeDPoint) -> Option<usize> {
        return self.xyz_map.get(&(p.x, p.y - 1, p.z)).copied();
    }

    fn get_side_front(&self, p: &ThreeDPoint) -> Option<usize> {
        return self.xyz_map.get(&(p.x, p.y, p.z + 1)).copied();
    }

    fn get_side_behind(&self, p: &ThreeDPoint) -> Option<usize> {
        return self.xyz_map.get(&(p.x, p.y, p.z - 1)).copied();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut db = Database::new();
        db.add_point(ThreeDPoint { x: 2, y: 2, z: 2 });
        db.add_point(ThreeDPoint { x: 1, y: 2, z: 2 });
        db.add_point(ThreeDPoint { x: 3, y: 2, z: 2 });
        db.add_point(ThreeDPoint { x: 2, y: 1, z: 2 });
        db.add_point(ThreeDPoint { x: 2, y: 3, z: 2 });
        db.add_point(ThreeDPoint { x: 2, y: 2, z: 1 });
        db.add_point(ThreeDPoint { x: 2, y: 2, z: 3 });
        db.add_point(ThreeDPoint { x: 2, y: 2, z: 4 });
        db.add_point(ThreeDPoint { x: 2, y: 2, z: 6 });
        db.add_point(ThreeDPoint { x: 1, y: 2, z: 5 });
        db.add_point(ThreeDPoint { x: 3, y: 2, z: 5 });
        db.add_point(ThreeDPoint { x: 2, y: 1, z: 5 });
        db.add_point(ThreeDPoint { x: 2, y: 3, z: 5 });
        dbg!(db.total_uncovered_sides());
        dbg!(db.total_exterior_sides());
    }
}

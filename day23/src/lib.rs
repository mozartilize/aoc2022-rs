use std::{
    collections::{HashMap, HashSet},
    i32,
};

pub struct Grid {
    pub arr: Vec<Vec<String>>,
    pub elves: HashSet<Point>,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Grid {
    pub fn new(arr: Vec<Vec<String>>, width: usize) -> Self {
        let mut elves: HashSet<Point> = HashSet::new();
        for y in 0..arr.len() {
            for x in 0..arr[y].len() {
                if arr[y][x] == "#" {
                    elves.insert(Point {
                        x: x as i32,
                        y: y as i32,
                    });
                }
            }
        }
        Self {
            arr,
            elves: elves,
        }
    }

    // [1.1 1.2 1.3 1.4]
    // [2.1 2.2 2.3 2.4]
    fn to_dir(self: &Self, p: &Point, dir: [i32; 2]) -> Point {
        // FIXME: expand width & height
        Point {
            x: p.x + dir[0],
            y: p.y + dir[1],
        }
    }

    fn is_elf_at(self: &Self, p: &Point) -> bool {
        self.elves.contains(p)
    }

    fn plan_all(
        self: &Self,
        state: &HashSet<Point>,
        start_idx: usize,
    ) -> HashMap<Point, Vec<Point>> {
        let mut planned_reversed: HashMap<Point, Vec<Point>> = HashMap::new();
        for p in state {
            if let Some(np) = self.planned_dir(p, start_idx) {
                planned_reversed.entry(np).or_default().push(*p);
            }
        }
        planned_reversed
    }

    pub fn run(self: &mut Self) -> i32 {
        let mut start_idx: usize = 0;
        // println!("initial {} {} {}", self.elves.len(), self.width, self.height);
        for i in 0..1000000000 {
            // for y in 0..self.height {
            //     for x in 0..self.width {
            //         if self.is_elf_at(&Point { x: x as i32, y: y as i32 }) {
            //             print!("#");
            //         }
            //         else {
            //             print!(".");
            //         }
            //     }
            //     println!();
            // }
            println!("round={} start_idx={}", i + 1, start_idx % 4);
            let planned_reversed = self.plan_all(&self.elves, start_idx);
            let mut has_new = false;
            for (new, olds) in planned_reversed {
                // println!("new {:?}, olds {:?}", new, olds);
                if olds.len() == 1 {
                    has_new = true;
                    self.elves.remove(&olds[0]);
                    self.elves.insert(new);
                }
            }
            if !has_new {
                println!("ah ah {i}");
                break;
            }
            start_idx += 1;
        }
        // println!(
        //     "hello??? {} {} {} {}",
        //     self.max_x, self.min_x, self.max_y, self.min_y
        // );
        // println!("after {}", self.elves.len());
        let mut min_x = i32::MAX;
        let mut min_y = i32::MAX;
        let mut max_x = i32::MIN;
        let mut max_y = i32::MIN;

        for p in &self.elves {
            min_x = min_x.min(p.x);
            min_y = min_y.min(p.y);
            max_x = max_x.max(p.x);
            max_y = max_y.max(p.y);
        }
        (max_x - min_x + 1) * (max_y - min_y + 1) - (self.elves.len() as i32)
    }

    fn planned_dir(self: &Self, p: &Point, start_idx: usize) -> Option<Point> {
        let dirs = [
            // north
            [[1, -1], [0, -1], [-1, -1]],
            // south
            [[1, 1], [0, 1], [-1, 1]],
            // west
            [[-1, -1], [-1, 0], [-1, 1]],
            // east
            [[1, -1], [1, 0], [1, 1]],
        ];
        let mut planned = None;
        if dirs
            .map(|_dirs| {
                _dirs
                    .map(|dir| self.to_dir(p, dir))
                    .map(|nb| !self.is_elf_at(&nb))
                    .iter()
                    .all(|ok| *ok)
            })
            .iter()
            .all(|ok| *ok)
        {
            // println!("{:?} no points around", p);
            return None;
        }
        for i in start_idx..start_idx + 4 {
            let j = i % 4;
            // println!("{:?} j={j}", p);
            let dir = dirs[j];
            if {
                dir.map(|dir| self.to_dir(p, dir))
                    .map(|nb| !self.is_elf_at(&nb))
                    .iter()
                    .all(|ok| *ok)
            } {
                planned = Some(self.to_dir(p, dir[1]));
                break;
            }
        }
        // if planned==*p {
        //     println!("{:?} surrounded", p);
        // }
        // if p == &(Point{x:7,y:3}) {
        //     println!("planned {:?}", planned);
        // }
        planned
    }
}

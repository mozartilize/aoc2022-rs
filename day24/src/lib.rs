use std::collections::{HashMap, HashSet, VecDeque};

/// gcd(a, b):
///   Euclid algorithm.
///   Dùng để tính LCM.
///   gcd = greatest common divisor = ước chung lớn nhất.
fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

/// lcm(a, b):
///   least common multiple = bội số chung nhỏ nhất.
///
///   Các blizzard di chuyển trong một hình chữ nhật bị chặn bởi tường.
///   Mỗi blizzard di chuyển theo một chu kỳ khép kín:
///       - Blizzard ngang lặp lại sau (width - 2) bước.
///       - Blizzard dọc lặp lại sau  (height - 2) bước.
///   Chu kỳ toàn cục của bản đồ = LCM của hai chu kỳ thành phần.
///
///   Nghĩa là blizzard_state[t] = blizzard_state[t + LCM].
///
///   Khi dùng:
///        visited.contains( (pos, t % cycle) )
///   -> state space hữu hạn.
/// -------------------------------------------------------
fn lcm(a: usize, b: usize) -> usize {
    a / gcd(a, b) * b
}

pub struct Grid {
    width: i32,
    height: i32,
    pub blizzards: HashMap<i32, HashSet<Blizzard>>,
    expedition: i32,
    exit: i32,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
enum Dir {
    UP,
    DOWN,
    RIGHT,
    LEFT,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct Blizzard {
    pos: i32,
    dir: Dir,
}

impl Blizzard {
    pub fn new(pos: i32, dir: &str) -> Self {
        Blizzard {
            pos,
            dir: match dir {
                "<" => Dir::LEFT,
                ">" => Dir::RIGHT,
                "^" => Dir::UP,
                "v" => Dir::DOWN,
                _ => panic!("invalid direction {}", dir),
            },
        }
    }
}

impl Grid {
    pub fn new(grid: &Vec<String>, width: i32) -> Self {
        let width = width;
        let height = grid.len() as i32 / width;
        Self {
            width,
            height,
            blizzards: grid
                .iter()
                .enumerate()
                .filter(|(idx, v)| [">", "<", "v", "^"].contains(&v.as_str()))
                .map(|(idx, v)| (idx as i32, HashSet::from([Blizzard::new(idx as i32, v)])))
                .collect::<HashMap<_, _>>(),
            expedition: 1,
            exit: height * width - 2,
        }
    }

    fn run_once(&self, start: i32, end: i32, t0: usize) -> usize {
        let cycle = lcm((self.width - 2) as usize, (self.height - 2) as usize);

        // Precompute blizzard states for all t mod cycle
        let mut blizzard_cache = Vec::with_capacity(cycle);
        blizzard_cache.push(self.blizzards.clone());
        for i in 1..cycle {
            let next = self.blizzards_step(&blizzard_cache[i - 1]);
            blizzard_cache.push(next);
        }

        let mut queue: VecDeque<(i32, usize)> = VecDeque::new();
        queue.push_back((start, t0));

        let mut visited: HashSet<(i32, usize)> = HashSet::new();
        visited.insert((start, t0 % cycle));

        while let Some((pos, t)) = queue.pop_front() {
            let nt = t + 1;
            let blz = &blizzard_cache[nt % cycle];

            // Candidate moves: up/down/left/right
            let mut moves = self.expedition_next_poss(pos, blz);

            // Allow waiting only if current tile is not occupied at next step
            if !blz.contains_key(&pos) {
                moves.push(pos);
            }

            for np in moves {
                if np == end {
                    return nt; // reached destination
                }
                let key = (np, nt % cycle);
                if visited.insert(key) {
                    queue.push_back((np, nt));
                }
            }
        }

        unreachable!("No path found from {} to {}", start, end);
    }

    /// Part Two: Start -> Goal -> Start -> Goal
    pub fn run2(&self) -> usize {
        let t1 = self.run_once(self.expedition, self.exit, 0);
        println!("{}", t1);
        let t2 = self.run_once(self.exit, self.expedition, t1);
        let t3 = self.run_once(self.expedition, self.exit, t2);
        t3
    }
    fn expedition_next_poss(
        self: &Self,
        pos: i32,
        blizzards: &HashMap<i32, HashSet<Blizzard>>,
    ) -> Vec<i32> {
        [Dir::UP, Dir::DOWN, Dir::RIGHT, Dir::LEFT]
            .iter()
            .map(|d| match d {
                Dir::UP => pos - self.width,
                Dir::DOWN => pos + self.width,
                Dir::RIGHT => pos + 1,
                Dir::LEFT => pos - 1,
            })
            .filter(|p| {
                // println!(
                //     "{p} {} {} {} {} {}",
                //     !blizzards.contains_key(p),
                //     p / self.width != 0,
                //     p / self.width != self.height - 1,
                //     p % self.width != 0,
                //     p % self.width != self.width - 1,
                // );
                !blizzards.contains_key(p)
                    && p / self.width != 0
                    && p / self.width != self.height - 1
                    && p % self.width != 0
                    && p % self.width != self.width - 1
                    || p == &self.expedition
                    || p == &self.exit
            })
            .collect::<Vec<i32>>()
    }

    pub fn blizzard_step(self: &Self, blizzard: &mut Blizzard) {
        match blizzard.dir {
            Dir::UP => blizzard.pos -= self.width,
            Dir::DOWN => blizzard.pos += self.width,
            Dir::RIGHT => blizzard.pos += 1,
            Dir::LEFT => blizzard.pos -= 1,
        };
        if blizzard.pos / self.width == 0 || blizzard.pos / self.width == self.height - 1 {
            let delta = (self.width * (self.height - 2))
                * (if blizzard.pos / self.width == self.height - 1 {
                    -1
                } else {
                    1
                });
            blizzard.pos += delta;
        } else if blizzard.pos % self.width == 0 || blizzard.pos % self.width == self.width - 1 {
            let delta = (self.width - 2)
                * (if blizzard.pos % self.width == self.width - 1 {
                    -1
                } else {
                    1
                });
            blizzard.pos += delta;
        }
        ()
    }

    pub fn blizzards_step(
        self: &Self,
        blizzards: &HashMap<i32, HashSet<Blizzard>>,
    ) -> HashMap<i32, HashSet<Blizzard>> {
        let mut n: HashMap<i32, HashSet<Blizzard>> = HashMap::new();
        for bs in blizzards.values() {
            for b in bs {
                let mut nb = b.clone();
                self.blizzard_step(&mut nb);
                n.entry(nb.pos).or_default().insert(nb);
            }
        }
        n
    }

    pub fn print_grid(self: &Self, blizzards: &HashMap<i32, HashSet<Blizzard>>, e: i32) {
        for y in 0..self.height {
            for x in 0..self.width {
                let pos = self.width * y + x;
                if pos == e {
                    print!("E");
                } else if pos == 1 {
                    print!(".")
                } else if pos == self.height * self.width - 2 {
                    print!(".")
                } else if y == 0 || x == 0 || y == self.height - 1 || x == self.width - 1 {
                    print!("#")
                } else if blizzards.contains_key(&pos) {
                    let bzs = blizzards.get(&pos).unwrap();
                    print!(
                        "{}",
                        match bzs.len() {
                            0 => panic!("invalid blizzard"),
                            1 => {
                                let bz = bzs.iter().next().unwrap();
                                match bz.dir {
                                    Dir::DOWN => "v".to_string(),
                                    Dir::UP => "^".to_string(),
                                    Dir::LEFT => "<".to_string(),
                                    Dir::RIGHT => ">".to_string(),
                                }
                            }
                            _ => {
                                bzs.len().to_string()
                            }
                        }
                    )
                } else {
                    print!(".");
                }
            }
            print!("\n");
        }
    }
}

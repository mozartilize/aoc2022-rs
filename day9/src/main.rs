use std::cell::RefCell;
use std::collections::HashSet;
use std::io;
use std::rc::Rc;

#[derive(Debug)]
struct Coordinate(i32, i32);

impl Coordinate {
    const UP: Self = Self(0, 1);
    const RIGHT: Self = Self(1, 0);
    const DOWN: Self = Self(0, -1);
    const LEFT: Self = Self(-1, 0);
}

#[derive(Debug)]
struct Move<'a> {
    dir: &'a Coordinate,
    step: i32,
}

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn walk(&mut self, coord: &Coordinate) {
        self.x += coord.0;
        self.y += coord.1;
    }
}

struct Rope {
    head: Rc<RefCell<Point>>,
    tail: Rc<RefCell<Point>>,
}

impl Rope {
    fn walk_head(&mut self, m: Move) -> HashSet<(i32, i32)> {
        let mut stops = HashSet::new();
        let mut moves = vec![];
        for _ in 0..m.step {
            let mut head = self.head.borrow_mut();
            let mut tail = self.tail.borrow_mut();
            head.walk(&m.dir);
            if (head.x - tail.x).abs() > 1 {
                if head.y != tail.y {
                    tail.y += (head.y - tail.y) / (head.y - tail.y).abs();
                    moves.push(Move {
                        dir: &m.dir,
                        step: 1,
                    });
                }
                tail.walk(&m.dir);
                stops.insert((tail.x, tail.y));
                moves.push(Move {
                    dir: &m.dir,
                    step: 1,
                });
                dbg!(&self.tail);
            } else if (head.y - tail.y).abs() > 1 {
                if head.x != tail.x {
                    tail.x += (head.x - tail.x) / (head.x - tail.x).abs();
                }
                tail.walk(&m.dir);
                stops.insert((tail.x, tail.y));
                dbg!(&self.tail);
            }
        }
        stops
    }

    fn walk_head_multiple<'a>(
        &'a mut self,
        steps_guide: impl Iterator<Item = String> + 'a,
    ) -> impl Iterator<Item = HashSet<(i32, i32)>> + 'a {
        steps_guide.map(|l| {
            let mut split = l.split(" ");
            let dir_char = split.next().unwrap();
            let step = str::parse::<i32>(split.next().unwrap()).unwrap();
            let m = match dir_char {
                "U" => Move {
                    dir: &Coordinate::UP,
                    step,
                },
                "R" => Move {
                    dir: &Coordinate::RIGHT,
                    step,
                },
                "D" => Move {
                    dir: &Coordinate::DOWN,
                    step,
                },
                "L" => Move {
                    dir: &Coordinate::LEFT,
                    step,
                },
                &_ => panic!("invalid char {}", dir_char),
            };
            dbg!(&m);
            self.walk_head(m)
        })
    }
}

fn main() {
    let head = Rc::new(RefCell::new(Point { x: 0, y: 0 }));
    let knot1 = Rc::new(RefCell::new(Point { x: 0, y: 0 }));
    let knot2 = Rc::new(RefCell::new(Point { x: 0, y: 0 }));

    let mut rope1 = Rope {
        head: Rc::clone(&head),
        tail: Rc::clone(&knot1),
    };
    let mut rope2 = Rope {
        head: Rc::clone(&knot1),
        tail: Rc::clone(&knot2),
    };

    let lines = io::stdin().lines();
    let some_part = std::env::args().last().unwrap();
    if some_part == "1" {
        let moves = rope1
            .walk_head_multiple(lines.map(|l| l.unwrap()))
            .reduce(|mut m1, m2| {
                m1.extend(m2);
                m1
            });
        // dbg!(&moves);
        let mut x = moves.unwrap();
        x.insert((0, 0));
        dbg!(&x.len());
    }
}

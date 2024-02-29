use std::cell::RefCell;
use std::collections::HashSet;
use std::io;
use std::rc::Rc;

#[derive(Debug, PartialEq)]
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
    fn walk_tail<'a>(
        &'a mut self,
        d: &'a Coordinate,
        debug: bool,
    ) -> (HashSet<(i32, i32)>, &Coordinate) {
        let mut stops = HashSet::new();
        let mut newd: Option<&Coordinate> = None;
        let head = self.head.borrow();
        let mut tail = self.tail.borrow_mut();
        if (head.x - tail.x).abs() > 1 || (head.y - tail.y).abs() > 1 {
            if head.x != tail.x && (d == &Coordinate::UP || d == &Coordinate::DOWN) {
                newd = Some(if head.x > tail.x {
                    &Coordinate::RIGHT
                } else {
                    &Coordinate::LEFT
                });
                tail.x += (head.x - tail.x) / (head.x - tail.x).abs();
            }
            if head.y != tail.y && (d == &Coordinate::LEFT || d == &Coordinate::RIGHT) {
                newd = Some(if head.y > tail.y {
                    &Coordinate::UP
                } else {
                    &Coordinate::DOWN
                });
                tail.y += (head.y - tail.y) / (head.y - tail.y).abs();
            }
            if ((head.x - tail.x).abs() > 0 && (d == &Coordinate::LEFT || d == &Coordinate::RIGHT))
                || ((head.y - tail.y).abs() > 0 && (d == &Coordinate::UP || d == &Coordinate::DOWN))
            {
                newd = Some(d);
                tail.walk(d);
            }
            stops.insert((tail.x, tail.y));
            if debug {
                dbg!(&tail);
            }
        }
        (stops, if newd.is_some() { newd.unwrap() } else { d })
    }
}

fn main() {
    let head = Rc::new(RefCell::new(Point { x: 0, y: 0 }));
    let knot1 = Rc::new(RefCell::new(Point { x: 0, y: 0 }));
    let knot2 = Rc::new(RefCell::new(Point { x: 0, y: 0 }));
    let knot3 = Rc::new(RefCell::new(Point { x: 0, y: 0 }));
    let knot4 = Rc::new(RefCell::new(Point { x: 0, y: 0 }));
    let knot5 = Rc::new(RefCell::new(Point { x: 0, y: 0 }));
    let knot6 = Rc::new(RefCell::new(Point { x: 0, y: 0 }));
    let knot7 = Rc::new(RefCell::new(Point { x: 0, y: 0 }));
    let knot8 = Rc::new(RefCell::new(Point { x: 0, y: 0 }));
    let knot9 = Rc::new(RefCell::new(Point { x: 0, y: 0 }));

    let mut rope1 = Rope {
        head: Rc::clone(&head),
        tail: Rc::clone(&knot1),
    };
    let mut rope2 = Rope {
        head: Rc::clone(&knot1),
        tail: Rc::clone(&knot2),
    };
    let mut rope3 = Rope {
        head: Rc::clone(&knot2),
        tail: Rc::clone(&knot3),
    };
    let mut rope4 = Rope {
        head: Rc::clone(&knot3),
        tail: Rc::clone(&knot4),
    };
    let mut rope5 = Rope {
        head: Rc::clone(&knot4),
        tail: Rc::clone(&knot5),
    };
    let mut rope6 = Rope {
        head: Rc::clone(&knot5),
        tail: Rc::clone(&knot6),
    };
    let mut rope7 = Rope {
        head: Rc::clone(&knot6),
        tail: Rc::clone(&knot7),
    };
    let mut rope8 = Rope {
        head: Rc::clone(&knot7),
        tail: Rc::clone(&knot8),
    };
    let mut rope9 = Rope {
        head: Rc::clone(&knot8),
        tail: Rc::clone(&knot9),
    };

    let lines = io::stdin().lines();
    let mut stops1 = HashSet::new();
    let mut stops9 = HashSet::new();
    for step_guides in lines.map(|l| l.unwrap()).map(|l| {
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
        m
    }) {
        dbg!(&step_guides);
        for _ in 0..step_guides.step {
            rope1.head.borrow_mut().walk(step_guides.dir);
            let (_stops1, d) = rope1.walk_tail(step_guides.dir, false);
            stops1.extend(_stops1);
            let (_stops2, d) = rope2.walk_tail(d, false);
            let (_stops3, d) = rope3.walk_tail(d, false);
            let (_stops4, d) = rope4.walk_tail(d, false);
            let (_stops5, d) = rope5.walk_tail(d, false);
            let (_stops6, d) = rope6.walk_tail(d, true);
            let (_stops7, d) = rope7.walk_tail(d, false);
            let (_stops8, d) = rope8.walk_tail(d, false);
            let (_stops9, _) = rope9.walk_tail(d, false);
            stops9.extend(_stops9);
        }
    }
    dbg!(stops1.len());
    dbg!(stops9.len());
}

use std::{collections::HashMap, vec};

use itertools::any;

const WIDTH: usize = 7;

#[derive(Debug, Default)]
struct Coord {
    x: usize,
    y: usize,
}

const SHAPES: [&[Coord]; 5] = [
    // horizontal line
    &[
        Coord { x: 0, y: 0 },
        Coord { x: 1, y: 0 },
        Coord { x: 2, y: 0 },
        Coord { x: 3, y: 0 },
    ],
    // plus
    &[
        Coord { x: 0, y: 1 },
        Coord { x: 1, y: 0 },
        Coord { x: 1, y: 1 },
        Coord { x: 1, y: 2 },
        Coord { x: 2, y: 1 },
    ],
    // J (or backwards L)
    &[
        Coord { x: 0, y: 0 },
        Coord { x: 1, y: 0 },
        Coord { x: 2, y: 0 },
        Coord { x: 2, y: 1 },
        Coord { x: 2, y: 2 },
    ],
    // vertical line
    &[
        Coord { x: 0, y: 0 },
        Coord { x: 0, y: 1 },
        Coord { x: 0, y: 2 },
        Coord { x: 0, y: 3 },
    ],
    // square
    &[
        Coord { x: 0, y: 0 },
        Coord { x: 1, y: 0 },
        Coord { x: 0, y: 1 },
        Coord { x: 1, y: 1 },
    ],
];
pub struct Chamber {
    buffer: Vec<[u8; WIDTH]>,
    shape_gen_index: i64,
    pub pressure: String,
    pressure_gen_index: i64,
    has_shape: bool,
    // clever solution from https://nickymeuleman.netlify.app,
    // only tracking the curr coord than we could add the offset
    // to get the value from the buffer to check if we could move the rocks
    curr: Coord,
    pattern: HashMap<(usize, usize), (usize, usize, usize)>,
}

impl Chamber {
    pub fn new(pressure: &str) -> Self {
        let _pressure = pressure
            .split("")
            .filter_map(|v| {
                if v.is_empty() {
                    None
                } else {
                    Some(v.to_string())
                }
            })
            .collect::<Vec<_>>();
        return Self {
            buffer: vec![],
            shape_gen_index: -1,
            pressure: pressure.to_string(),
            pressure_gen_index: -1,
            has_shape: false,
            pattern: HashMap::new(),
            curr: Coord::default(),
        };
    }

    pub fn run(&mut self, cnt: usize) -> usize {
        let mut _cnt = 0;
        let mut cycle_detected = false;
        let mut real_height = 0;
        loop {
            if !self.has_shape {
                self.spawn_shape();
                self.has_shape = true;
                let p = self.next_pressure();
                // println!("{}   {}", _cnt + 1, p);
                if p == '>' {
                    self.right();
                } else {
                    self.left();
                }
            } else {
                if self.down() {
                    let p = self.next_pressure();
                    // println!("{}   {}", _cnt + 1, p);
                    if p == '>' {
                        self.right();
                    } else {
                        self.left();
                    }
                } else {
                    for coord in SHAPES[self.shape_gen_index as usize] {
                        if coord.y + self.curr.y >= self.buffer.len() {
                            self.buffer.push([0; 7]);
                        }
                        self.buffer[coord.y + self.curr.y][coord.x + self.curr.x] = 1;
                    }
                    self.has_shape = false;
                    self.trim_top();
                    // for line in self.buffer.iter().rev() {
                    //     for val in line {
                    //         if *val == 0 {
                    //             print!(".");
                    //         } else {
                    //             print!("#");
                    //         }
                    //     }
                    //     println!();
                    // }
                    // println!();
                    // println!();
                    let key = (
                        self.shape_gen_index as usize,
                        self.pressure_gen_index as usize,
                    );
                    _cnt += 1;
                    // some shapes drop first then the pattern starts appear
                    // I was felt for the pattern starts with the first shape
                    if let Some((repeat, __cnt, height)) = self.pattern.get_mut(&key) {
                        if !cycle_detected && *repeat == 2 {
                            cycle_detected = true;
                            let shapes_cnt = _cnt - *__cnt;
                            let chunk_height = self.buffer.len() - *height;
                            let chunk_repeat = (cnt - _cnt) / shapes_cnt;
                            dbg!(self.buffer.len(), *height, chunk_height, chunk_repeat, _cnt);
                            _cnt += shapes_cnt * chunk_repeat;
                            real_height = chunk_height * chunk_repeat;
                        } else {
                            *repeat += 1;
                            *__cnt = _cnt;
                            *height = self.buffer.len();
                        }
                    } else {
                        self.pattern.insert(key, (1, _cnt, self.buffer.len()));
                    };
                    if _cnt == cnt {
                        for line in self.buffer.iter() {
                            for val in line {
                                if *val == 0 {
                                    print!(".");
                                } else {
                                    print!("#");
                                }
                            }
                            println!();
                        }
                        println!();
                        println!();
                        println!("{}", _cnt);
                        break;
                    }
                }
            }
        }
        dbg!(real_height + self.buffer.len());
        real_height + self.buffer.len()
    }

    fn next_pressure(&mut self) -> char {
        if self.pressure_gen_index as usize == self.pressure.len() - 1 {
            self.pressure_gen_index = 0;
        } else {
            self.pressure_gen_index += 1;
        }
        self.pressure
            .chars()
            .nth(self.pressure_gen_index as usize)
            .unwrap()
    }

    fn next_shape(&mut self) {
        if self.shape_gen_index as usize == SHAPES.len() - 1 {
            self.shape_gen_index = 0;
        } else {
            self.shape_gen_index += 1;
        }
    }

    fn trim_top(&mut self) {
        let mut i = 0;
        for y in (0..self.buffer.len()).rev() {
            if any(self.buffer[y].iter(), |v| *v != 0) {
                break;
            }
            i = y;
        }
        if i > 0 {
            let _ = self.buffer.split_off(i);
        }
    }

    fn spawn_shape(&mut self) {
        let slice = &[[0u8; WIDTH], [0u8; WIDTH], [0u8; WIDTH]];
        self.buffer.extend_from_slice(slice);
        self.curr.x = 2;
        self.curr.y = self.buffer.len();

        self.next_shape();
    }

    fn left(&mut self) -> bool {
        for coord in SHAPES[self.shape_gen_index as usize] {
            let x = coord.x + self.curr.x;
            // we only add 3 blank lines, but
            // no need to check for the part of the shape above the buffer
            if coord.y + self.curr.y <= self.buffer.len() - 1
                && (x == 0 || self.buffer[coord.y + self.curr.y][x - 1] == 1)
            {
                return false;
            }
        }
        self.curr.x -= 1;
        true
    }

    fn right(&mut self) -> bool {
        for coord in SHAPES[self.shape_gen_index as usize] {
            let x = coord.x + self.curr.x;
            if coord.y + self.curr.y <= self.buffer.len() - 1
                && (x == WIDTH - 1 || self.buffer[coord.y + self.curr.y][x + 1] == 1)
            {
                return false;
            }
        }
        self.curr.x += 1;
        true
    }

    fn down(&mut self) -> bool {
        for coord in SHAPES[self.shape_gen_index as usize] {
            let y = coord.y + self.curr.y;
            if coord.y + self.curr.y <= self.buffer.len() - 1
                && (y == 0 || self.buffer[y - 1][coord.x + self.curr.x] == 1)
            {
                return false;
            }
        }
        self.curr.y -= 1;
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut c = Chamber::new(">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>");
        let height = c.run(2022);
        // dbg!(c.pattern);
        assert_eq!(height, 3068);
    }

    #[test]
    fn test_test() {
        let mut c = Chamber::new("><<<>");
        c.run(30);
        // dbg!(c.pattern);
        assert_eq!(c.buffer.len(), 3068);
    }
}

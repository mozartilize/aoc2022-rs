use itertools::any;

pub struct Chamber {
    pub height: usize,
    buffer: Vec<Vec<u8>>,
    shape_gen_index: i64,
    shapes: Vec<Vec<Vec<u8>>>,
    x: usize,
    y: usize,
    pub pressure: String,
    pressure_gen_index: i64,
    has_shape: bool,
}

impl Chamber {
    pub fn new(pressure: &str) -> Self {
        let shapes = vec![
            // horizontal line
            vec![vec![1, 1, 1, 1]],
            // cross
            vec![vec![0, 1, 0], vec![1, 1, 1], vec![0, 1, 0]],
            // backward L
            vec![vec![1, 1, 1], vec![0, 0, 1], vec![0, 0, 1]],
            // vertical line
            vec![vec![1], vec![1], vec![1], vec![1]],
            // square
            vec![vec![1, 1], vec![1, 1]],
        ];
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
            height: 0,
            buffer: vec![],
            shape_gen_index: -1,
            shapes,
            x: 2,
            y: 3,
            pressure: pressure.to_string(),
            pressure_gen_index: -1,
            has_shape: false,
        };
    }

    pub fn run(&mut self, cnt: usize) {
        let mut cnt_ = 0;
        loop {
            if !self.has_shape {
                self.spawn_shape();
                self.has_shape = true;
                let p = self.next_pressure();
                // println!("{}   {}", cnt_ + 1, p);
                if p == '>' {
                    self.right();
                } else {
                    self.left();
                }
            } else {
                if self.down() {
                    let p = self.next_pressure();
                    // println!("{}   {}", cnt_ + 1, p);
                    if p == '>' {
                        self.right();
                    } else {
                        self.left();
                    }
                } else {
                    self.get_height();
                    // for line in self.buffer.iter().rev() {
                    //     for val in line {
                    //         if *val == 0 {
                    //             print!(".");
                    //         }
                    //         else {
                    //             print!("#");
                    //         }
                    //     }
                    //     println!();
                    // }
                    // println!();
                    // println!();
                    cnt_ += 1;
                    if cnt_ == cnt {
                        break;
                    }
                    self.has_shape = false;
                }
            }
        }
    }

    fn curr_shape(&self) -> &Vec<Vec<u8>> {
        &self.shapes[self.shape_gen_index as usize]
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

    fn next_shape(&mut self) -> Vec<Vec<u8>> {
        if self.shape_gen_index as usize == self.shapes.len() - 1 {
            self.shape_gen_index = 0;
        } else {
            self.shape_gen_index += 1;
        }
        self.shapes[self.shape_gen_index as usize].clone()
    }

    fn padding_shape(&self, shape: &mut Vec<Vec<u8>>) {
        for i in 0..shape.len() {
            let line = shape.get_mut(i).unwrap();
            line.splice(0..0, vec![0, 0]);
            for _ in 0..7 - line.len() {
                line.push(0);
            }
        }
    }

    fn get_height(&mut self) {
        let mut i = 0;
        for y in (0..self.buffer.len()).rev() {
            if any(self.buffer[y].iter(), |v| *v != 0) {
                break;
            }
            i = y;
        }
        let offset = self.buffer.len() - 1 - i;
        if offset > 0 && self.curr_shape().len() + 3 > offset {
            self.height += self.curr_shape().len() + 3 - offset - 1;
        }
        if i > 0 {
            let _ = self.buffer.split_off(i);
        }
    }

    fn spawn_shape(&mut self) {
        // println!("boom");
        let slice = &[vec![0u8; 7], vec![0u8; 7], vec![0u8; 7]];
        self.buffer.extend_from_slice(slice);
        self.x = 2;
        self.y = self.buffer.len();

        let mut ns = self.next_shape();
        self.padding_shape(&mut ns);
        self.buffer.extend_from_slice(&ns);
    }

    fn left(&mut self) -> bool {
        if self.x == 0 {
            return false;
        }
        let mut checks = vec![false; self.y + self.curr_shape().len()];
        let mut vals = vec![0; self.y + self.curr_shape().len()];
        for x in self.x..self.x + self.curr_shape()[0].len() {
            for y in self.y..self.y + self.curr_shape().len() {
                if self.curr_shape()[y - self.y][x - self.x] == 1 && !checks[y] {
                    checks[y] = true;
                    if self.buffer[y][x - 1] == 1 {
                        vals[y] = 1;
                    }
                }
            }
            if any(vals.iter(), |v| *v == 1) {
                return false;
            }
        }
        for y in self.y..self.y + self.curr_shape().len() {
            for x in self.x..self.x + self.curr_shape()[0].len() {
                if self.curr_shape()[y - self.y][x - self.x] != 0 {
                    self.buffer[y][x - 1] = 1;
                    self.buffer[y][x] = 0;
                }
            }
        }
        self.x -= 1;
        true
    }

    fn right(&mut self) -> bool {
        if self.x + self.curr_shape()[0].len() == 7 {
            return false;
        }
        let mut checks = vec![false; self.y + self.curr_shape().len()];
        let mut vals = vec![0; self.y + self.curr_shape().len()];
        for x in (self.x..self.x + self.curr_shape()[0].len()).rev() {
            for y in self.y..self.y + self.curr_shape().len() {
                if self.curr_shape()[y - self.y][x - self.x] == 1 && !checks[y] {
                    checks[y] = true;
                    if self.buffer[y][x + 1] == 1 {
                        vals[y] = 1;
                    }
                }
            }
            if any(vals.iter(), |v| *v == 1) {
                return false;
            }
        }
        for y in self.y..self.y + self.curr_shape().len() {
            for x in (self.x..self.x + self.curr_shape()[0].len()).rev() {
                if self.curr_shape()[y - self.y][x - self.x] != 0 {
                    self.buffer[y][x + 1] = 1;
                    self.buffer[y][x] = 0;
                }
            }
        }
        self.x += 1;
        true
    }

    fn down(&mut self) -> bool {
        if self.y == 0 {
            return false;
        }
        let mut checks = vec![false; 7];
        let mut vals = vec![0; 7];
        for y in self.y..self.y + self.curr_shape().len() {
            for x in self.x..self.x + self.curr_shape()[0].len() {
                if self.curr_shape()[y - self.y][x - self.x] == 1 && !checks[x] {
                    checks[x] = true;
                    if self.buffer[y - 1][x] == 1 {
                        vals[x] = 1;
                    }
                }
            }
            if any(vals.iter(), |v| *v == 1) {
                return false;
            }
        }
        for y in self.y..self.y + self.curr_shape().len() {
            for x in self.x..self.x + self.curr_shape()[0].len() {
                if self.curr_shape()[y - self.y][x - self.x] != 0 {
                    self.buffer[y - 1][x] = 1;
                    self.buffer[y][x] = 0;
                }
            }
        }
        self.y -= 1;
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut c = Chamber::new(">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>");
        c.buffer = vec![
            vec![1, 1, 1, 0, 0, 0, 0],
            vec![1, 1, 1, 0, 0, 0, 0],
            vec![0, 0, 1, 0, 0, 0, 0],
            vec![0, 0, 1, 0, 0, 0, 0],
        ]
        .into_iter()
        .rev()
        .collect::<Vec<_>>();
        c.shape_gen_index = 4;
        c.x = 0;
        c.y = 2;
        assert_eq!(c.left(), false);
        assert_eq!(c.right(), false);
        assert_eq!(c.down(), true);
        assert_eq!(
            c.buffer,
            vec![
                vec![0, 0, 1, 0, 0, 0, 0],
                vec![1, 1, 1, 0, 0, 0, 0],
                vec![1, 1, 1, 0, 0, 0, 0],
                vec![0, 0, 1, 0, 0, 0, 0],
            ]
            .into_iter()
            .rev()
            .collect::<Vec<_>>()
        );
    }

    #[test]
    fn test_2() {
        let mut c = Chamber::new(">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>");
        c.buffer = vec![
            vec![0, 1, 0, 0, 0, 0, 0],
            vec![1, 1, 1, 0, 0, 0, 0],
            vec![0, 1, 0, 1, 0, 0, 0],
            vec![0, 1, 1, 1, 0, 0, 0],
        ]
        .into_iter()
        .rev()
        .collect::<Vec<_>>();
        c.shape_gen_index = 1;
        c.x = 0;
        c.y = 1;
        assert_eq!(c.down(), false);
        assert_eq!(c.left(), false);
        assert_eq!(c.right(), true);
        assert_eq!(c.x, 1);
        assert_eq!(
            c.buffer,
            vec![
                vec![0, 0, 1, 0, 0, 0, 0],
                vec![0, 1, 1, 1, 0, 0, 0],
                vec![0, 0, 1, 1, 0, 0, 0],
                vec![0, 1, 1, 1, 0, 0, 0],
            ]
            .into_iter()
            .rev()
            .collect::<Vec<_>>()
        );
    }

    #[test]
    fn test_3() {
        let mut c = Chamber::new(">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>");
        c.buffer = vec![
            vec![0, 1, 0, 0, 0, 0, 0],
            vec![1, 1, 1, 0, 0, 0, 0],
            vec![0, 1, 1, 0, 0, 0, 0],
            vec![0, 1, 1, 1, 0, 0, 0],
        ]
        .into_iter()
        .rev()
        .collect::<Vec<_>>();
        c.shape_gen_index = 1;
        c.x = 0;
        c.y = 1;
        assert_eq!(c.down(), false);
        assert_eq!(c.left(), false);
        assert_eq!(c.right(), false);
    }

    #[test]
    fn test_4() {
        let mut c = Chamber::new(">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>");
        c.buffer = vec![
            vec![0, 0, 0, 0, 0, 1, 0],
            vec![0, 0, 0, 0, 1, 1, 1],
            vec![0, 0, 0, 0, 1, 1, 0],
            vec![0, 0, 0, 1, 1, 1, 0],
        ]
        .into_iter()
        .rev()
        .collect::<Vec<_>>();
        c.shape_gen_index = 1;
        c.x = 4;
        c.y = 1;
        assert_eq!(c.down(), false);
        assert_eq!(c.right(), false);
        assert_eq!(c.left(), false);
    }

    #[test]
    fn test_5() {
        let mut c = Chamber::new(">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>");
        c.buffer = vec![
            vec![0, 0, 1, 0, 0, 0, 0],
            vec![0, 1, 1, 1, 0, 0, 0],
            vec![0, 0, 1, 1, 0, 0, 0],
            vec![0, 1, 1, 1, 0, 0, 0],
        ]
        .into_iter()
        .rev()
        .collect::<Vec<_>>();
        c.shape_gen_index = 1;
        c.x = 1;
        c.y = 1;
        assert_eq!(c.down(), false);
        assert_eq!(c.right(), false);
        assert_eq!(c.left(), true);
        assert_eq!(c.x, 0);
        assert_eq!(
            c.buffer,
            vec![
                vec![0, 1, 0, 0, 0, 0, 0],
                vec![1, 1, 1, 0, 0, 0, 0],
                vec![0, 1, 0, 1, 0, 0, 0],
                vec![0, 1, 1, 1, 0, 0, 0],
            ]
            .into_iter()
            .rev()
            .collect::<Vec<_>>()
        );
    }

    #[test]
    fn test_6() {
        let mut c = Chamber::new(">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>");
        c.buffer = vec![
            vec![0, 0, 1, 0, 0, 0, 0],
            vec![1, 1, 1, 1, 1, 0, 0],
            vec![1, 0, 1, 0, 1, 0, 0],
            vec![1, 1, 0, 1, 1, 0, 0],
        ]
        .into_iter()
        .rev()
        .collect::<Vec<_>>();
        c.shape_gen_index = 1;
        c.x = 1;
        c.y = 1;
        assert_eq!(c.right(), false);
        assert_eq!(c.left(), false);
        assert_eq!(c.down(), true);
        assert_eq!(c.y, 0);
        assert_eq!(
            c.buffer,
            vec![
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![1, 0, 1, 0, 1, 0, 0],
                vec![1, 1, 1, 1, 1, 0, 0],
                vec![1, 1, 1, 1, 1, 0, 0],
            ]
            .into_iter()
            .rev()
            .collect::<Vec<_>>()
        );
    }

    #[test]
    fn test_7() {
        let mut c = Chamber::new(">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>");
        c.buffer = vec![
            vec![0, 0, 1, 0, 0, 0, 0],
            vec![1, 1, 1, 1, 1, 0, 0],
            vec![1, 1, 1, 1, 1, 0, 0],
            vec![1, 0, 0, 0, 1, 0, 0],
        ]
        .into_iter()
        .rev()
        .collect::<Vec<_>>();
        c.shape_gen_index = 1;
        c.x = 1;
        c.y = 1;
        assert_eq!(c.right(), false);
        assert_eq!(c.left(), false);
        assert_eq!(c.down(), false);
    }

    #[test]
    fn test() {
        let mut c = Chamber::new(">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>");
        c.run(2022);
        assert_eq!(c.height, 3068);
    }
}

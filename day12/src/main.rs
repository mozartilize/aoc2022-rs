use std::{
    collections::HashSet,
    fmt,
    io::{self, Read},
};

#[derive(Debug)]
struct Cell {
    x: usize,
    val: u8,
}

impl Cell {
    fn new(x: usize, val: u8) -> Cell {
        Self { x, val }
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Cell{{({:?}, {:?}), {}}}",
            self.x / 8,
            self.x % 8,
            self.val
        )
    }
}

#[derive(Debug)]
struct Grid {
    cells: Vec<Cell>,
    start_idx: usize,
    end_idx: usize,
    width: usize,
    acells: HashSet<usize>,
}

impl Grid {
    fn walkable_cells_for_part1(&self, cell: &Cell) -> Vec<usize> {
        let mut cell_indies: Vec<usize> = vec![];
        // left_cell
        if (cell.x + 1) % self.width != 1 && (cell.val >= self.cells[cell.x - 1].val - 1) {
            cell_indies.push(cell.x - 1);
        }
        // right_cell
        if (cell.x + 1) % self.width != 0 && (cell.val >= self.cells[cell.x + 1].val - 1) {
            cell_indies.push(cell.x + 1);
        };
        // up_cell
        if cell.x > self.width - 1 && (cell.val >= self.cells[cell.x - self.width].val - 1) {
            cell_indies.push(cell.x - self.width);
        }
        // down_cell
        if cell.x + self.width < self.cells.len()
            && (cell.val >= self.cells[cell.x + self.width].val - 1)
        {
            cell_indies.push(cell.x + self.width);
        }
        cell_indies
    }

    fn walkable_cells_for_part2(&self, cell: &Cell) -> Vec<usize> {
        let mut cell_indies: Vec<usize> = vec![];
        // left_cell
        if (cell.x + 1) % self.width != 1 && (cell.val <= self.cells[cell.x - 1].val + 1) {
            cell_indies.push(cell.x - 1);
        }
        // right_cell
        if (cell.x + 1) % self.width != 0 && (cell.val <= self.cells[cell.x + 1].val + 1) {
            cell_indies.push(cell.x + 1);
        };
        // up_cell
        if cell.x > self.width - 1 && (cell.val <= self.cells[cell.x - self.width].val + 1) {
            cell_indies.push(cell.x - self.width);
        }
        // down_cell
        if cell.x + self.width < self.cells.len()
            && (cell.val <= self.cells[cell.x + self.width].val + 1)
        {
            cell_indies.push(cell.x + self.width);
        }
        cell_indies
    }

    fn get_cell(&self, x: usize) -> &Cell {
        &self.cells[x]
    }

    fn walk(&self, from_cell: &Cell, to_cell: Option<&Cell>) -> usize {
        let mut visited: HashSet<usize> = HashSet::from([from_cell.x]);
        let mut visited_val: HashSet<u8> = HashSet::from([from_cell.val]);
        let mut current = HashSet::from([from_cell.x]);
        let mut steps = 0;
        loop {
            let mut next = HashSet::new();
            for idx in current {
                let next_indices = if to_cell.is_some() {
                    self.walkable_cells_for_part1(self.get_cell(idx))
                } else {
                    self.walkable_cells_for_part2(self.get_cell(idx))
                };
                for next_idx in next_indices {
                    if visited.contains(&next_idx) {
                        continue;
                    }

                    next.insert(next_idx);
                    visited.insert(next_idx);

                    visited_val.insert(self.get_cell(next_idx).val);
                    if to_cell.is_some() && visited.contains(&to_cell.unwrap().x) {
                        std::mem::take(&mut next);
                        break;
                    } else if to_cell.is_none() && visited_val.contains(&97) {
                        std::mem::take(&mut next);
                        break;
                    }
                }
            }
            current = next;
            steps += 1;
            if current.is_empty() {
                break;
            }
        }
        steps
    }

    fn run(&mut self) {
        let steps1 = self.walk(
            self.get_cell(self.start_idx),
            Some(self.get_cell(self.end_idx)),
        );

        let steps2 = self.walk(self.get_cell(self.end_idx), None);
        dbg!(steps1);
        dbg!(steps2);
    }
}

fn main() {
    let mut x: usize = 0;
    let mut grid = Grid {
        cells: vec![],
        start_idx: 0,
        end_idx: 0,
        width: 0,
        acells: HashSet::new(),
    };
    loop {
        let mut buf = vec![0u8; 1];
        let result = io::stdin().read_exact(&mut buf);
        if result.is_ok() {
            match buf[0] as char {
                '\n' => {
                    if grid.width == 0 {
                        grid.width = x;
                    }
                }
                'a'..='z' => {
                    let cell = Cell::new(x, buf[0]);
                    grid.cells.push(cell);
                    if buf[0] == 97 {
                        grid.acells.insert(x);
                    }
                    x += 1;
                }
                'S' => {
                    let cell = Cell::new(x, 97);
                    grid.cells.push(cell);
                    grid.start_idx = x;
                    grid.acells.insert(x);
                    x += 1;
                }
                'E' => {
                    let cell = Cell::new(x, 122);
                    grid.cells.push(cell);
                    grid.end_idx = x;
                    x += 1;
                }
                _ => panic!("invalid char {}", buf[0] as char),
            }
        } else {
            break;
        }
    }
    dbg!(grid.width);
    grid.run();
}

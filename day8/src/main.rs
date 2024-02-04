use std::collections::{HashMap, HashSet};
use std::io;

fn is_visible(line: &[u8], h: &u8) -> bool {
    line.iter().all(|v| v < h)
}

fn part1(lines: impl Iterator<Item = String>) {
    let mut size = 0;
    let mut matrix: Vec<Vec<u8>> = vec![];
    let mut visibles = HashSet::new();
    let mut visibles_right = HashSet::new();
    let mut visibles_down = HashSet::new();
    for line in lines {
        if size == 0 {
            size = line.len();
        }
        let mut row: Vec<u8> = vec![];
        line.split("")
            .filter(|c| c != &"")
            .map(|c| str::parse::<u8>(c).unwrap())
            .for_each(|h| {
                let x = row.len();
                let y = matrix.len();
                if x == 0 || x == size - 1 || y == size - 1 || y == 0 {
                    visibles.insert((x, y));
                }
                if x > 0 {
                    if is_visible(&row[..x], &h) {
                        visibles.insert((x, y));
                    }
                    if !visibles.contains(&(x, y)) {
                        visibles_right.insert((x, y));
                    }
                    (1..x).for_each(|x| {
                        if row[x] <= h {
                            visibles_right.remove(&(x, y));
                        }
                    })
                }
                if y > 0 {
                    let col = (0..y).map(|y| matrix[y][x]).collect::<Vec<u8>>();
                    if is_visible(&col, &h) {
                        visibles.insert((x, y));
                    }
                    if !visibles.contains(&(x, y)) {
                        visibles_down.insert((x, y));
                    }
                    (1..y).for_each(|y| {
                        if matrix[y][x] <= h {
                            visibles_down.remove(&(x, y));
                        }
                    })
                }
                row.push(h);
            });
        matrix.push(row);
    }
    dbg!(&visibles_right);
    dbg!(&visibles_down);
    visibles.extend(visibles_right);
    visibles.extend(visibles_down);
    dbg!(&visibles);
    dbg!(&visibles.len());
}

fn cal_view_points(line: &[i32], h: i32) -> i32 {
    if line.len() == 0 {
        return 0;
    } else if line.len() == 1 {
        return 1;
    }
    let mut cnt = 0;
    for i in (0..(line.len())).rev() {
        if line[i] < h {
            cnt += 1;
        } else if line[i] == h {
            cnt += 1;
            break;
        } else {
            break;
        }
    }
    return cnt;
}

fn part2(lines: impl Iterator<Item = String>) {
    let mut size = 0;
    let mut matrix: Vec<Vec<i32>> = vec![];
    let mut view_points = HashMap::new();
    let mut view_points_right = HashMap::new();
    let mut view_points_down = HashMap::new();
    for line in lines {
        if size == 0 {
            size = line.len();
        }
        let mut row: Vec<i32> = vec![];
        line.split("")
            .filter(|c| c != &"")
            .map(|c| str::parse::<i32>(c).unwrap())
            .for_each(|h| {
                let x = row.len();
                let y = matrix.len();
                *view_points.entry((x, y)).or_insert(1) *= cal_view_points(&row[..x], h);
                let col = (0..y).map(|y| matrix[y][x]).collect::<Vec<i32>>();
                *view_points.entry((x, y)).or_insert(1) *= cal_view_points(&col, h);

                if x == size - 1 || y == size - 1 {
                    *view_points.entry((x, y)).or_insert(0) *= 0;
                }

                (0..x).for_each(|x0| {
                    if view_points_right.get(&(x0, y)).unwrap_or(&-1) < &0 && row[x0] > h {
                        *view_points_right.entry((x0, y)).or_insert(0) -= 1;
                        if x == size - 1 {
                            view_points_right.insert(
                                (x0, y),
                                -1 * view_points_right.get(&(x0, y)).unwrap_or(&-1),
                            );
                            *view_points.entry((x0, y)).or_insert(1) *=
                                view_points_right.get(&(x0, y)).unwrap();
                        }
                    } else if view_points_right.get(&(x0, y)).unwrap_or(&-1) < &0 && row[x0] <= h {
                        *view_points_right.entry((x0, y)).or_insert(0) -= 1;
                        view_points_right
                            .insert((x0, y), -1 * view_points_right.get(&(x0, y)).unwrap_or(&-1));
                        *view_points.entry((x0, y)).or_insert(1) *=
                            view_points_right.get(&(x0, y)).unwrap();
                    }
                });
                (0..y).for_each(|y0| {
                    if view_points_down.get(&(x, y0)).unwrap_or(&-1) < &0 && matrix[y0][x] > h {
                        *view_points_down.entry((x, y0)).or_insert(0) -= 1;
                        if y == size - 1 {
                            view_points_down.insert(
                                (x, y0),
                                -1 * view_points_down.get(&(x, y0)).unwrap_or(&-1),
                            );
                            *view_points.entry((x, y0)).or_insert(1) *=
                                view_points_down.get(&(x, y0)).unwrap();
                        }
                    } else if view_points_down.get(&(x, y0)).unwrap_or(&-1) < &0
                        && matrix[y0][x] <= h
                    {
                        *view_points_down.entry((x, y0)).or_insert(0) -= 1;
                        view_points_down
                            .insert((x, y0), -1 * view_points_down.get(&(x, y0)).unwrap_or(&-1));
                        *view_points.entry((x, y0)).or_insert(1) *=
                            view_points_down.get(&(x, y0)).unwrap();
                    }
                });
                row.push(h);
            });
        matrix.push(row);
    }
    dbg!(view_points.values().max());
}

fn main() {
    let lines = io::stdin().lines();
    let some_part = std::env::args().last().unwrap();
    if some_part == "1" {
        part1(lines.map(|l| l.unwrap()));
    } else if some_part == "2" {
        part2(lines.map(|l| l.unwrap()));
    } else {
        return;
    }
}

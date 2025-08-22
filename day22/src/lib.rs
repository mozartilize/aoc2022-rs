use std::{
    collections::{HashMap, HashSet},
};

use regex::Regex;

pub struct Grid {
    pub arr: Vec<String>,
    width: usize,
    height: usize,
}

impl Grid {
    pub fn new(arr: Vec<String>, width: usize) -> Self {
        Self {
            width,
            height: arr.len() / width,
            arr,
        }
    }

    fn val_at(self: &Self, x: i32, y: i32) -> &str {
        self.arr[(y * (self.width as i32) + x) as usize].as_str()
    }

    fn neighbour8s(self: &Self, pos: &[i32; 2]) -> Vec<[i32; 2]> {
        let mut r = vec![];
        for d in [
            [0, 1],
            [1, 1],
            [1, 0],
            [1, -1],
            [0, -1],
            [-1, -1],
            [-1, 0],
            [-1, 1],
        ] {
            if pos[0] + d[0] >= 0
                && pos[0] + d[0] <= self.width as i32 - 1
                && pos[1] + d[1] >= 0
                && pos[1] + d[1] <= self.height as i32 - 1
                && self.val_at(pos[0] + d[0], pos[1] + d[1]) != " "
            {
                r.push(d);
            }
        }
        r
    }

    fn missed_neighbour8_dirs(self: &Self, pos: &[i32; 2]) -> Vec<[i32; 2]> {
        let mut r = vec![];
        for d in [
            [0, 1],
            [1, 1],
            [1, 0],
            [1, -1],
            [0, -1],
            [-1, -1],
            [-1, 0],
            [-1, 1],
        ] {
            if !(pos[0] + d[0] >= 0
                && pos[0] + d[0] <= self.width as i32 - 1
                && pos[1] + d[1] >= 0
                && pos[1] + d[1] <= self.height as i32 - 1
                && self.val_at(pos[0] + d[0], pos[1] + d[1]) != " ")
            {
                r.push(d);
            }
        }
        r
    }

    fn edge_points(self: &Self) -> Vec<[i32; 2]> {
        let mut r = Vec::new();
        for i in 0..self.arr.len() {
            let y = (i / self.width) as i32;
            let x = (i % self.width) as i32;
            if self.val_at(x, y) == " " {
                continue;
            }
            let n = self.missed_neighbour8_dirs(&[x, y]);
            if n.len() >= 2 {
                r.push([x, y]);
            }
        }
        r
    }

    fn corner_points(self: &Self) -> HashMap<[i32; 2], [i32; 2]> {
        let mut r = HashMap::new();
        for i in 0..self.arr.len() {
            let y = (i / self.width) as i32;
            let x = (i % self.width) as i32;
            let n = self.missed_neighbour8_dirs(&[x, y]);
            if n.len() == 1 {
                r.insert([x, y], n[0]);
            }
        }
        r
    }

    fn points_on_zshape_of_corner(
        self: &Self,
        pos: &[i32; 2],
        dir: &[i32; 2],
    ) -> ([[i32; 2]; 2], Vec<[i32; 2]>) {
        let mut on_x_cnt = 0;
        let mut on_y_cnt = 0;
        let mut max_x_pos = pos.clone().to_owned();
        let mut max_y_pos = pos.clone().to_owned();
        let mut points = vec![];
        while max_x_pos[0] + dir[0] >= 0
            && max_x_pos[0] + dir[0] <= self.width as i32 - 1
            && self.val_at(max_x_pos[0] + dir[0], max_x_pos[1]) != " "
        {
            on_x_cnt += 1;
            max_x_pos = [max_x_pos[0] + dir[0], max_x_pos[1]];
            points.push(max_x_pos);
        }
        while max_y_pos[1] + dir[1] >= 0
            && max_y_pos[1] + dir[1] <= self.height as i32 - 1
            && self.val_at(max_y_pos[0], max_y_pos[1] + dir[1]) != " "
        {
            on_y_cnt += 1;
            max_y_pos = [max_y_pos[0], max_y_pos[1] + dir[1]];
            points.push(max_y_pos);
        }
        if on_x_cnt < on_y_cnt {
            while on_x_cnt != on_y_cnt - 1 {
                on_x_cnt += 1;
                max_x_pos = [max_x_pos[0], max_x_pos[1] - dir[1]];
                points.push(max_x_pos);
            }
        } else if on_x_cnt > on_y_cnt {
            while on_x_cnt - 1 != on_y_cnt {
                on_y_cnt += 1;
                max_y_pos = [max_y_pos[0] - dir[0], max_y_pos[1]];
                points.push(max_y_pos);
            }
        }
        ([max_x_pos, max_y_pos], points)
    }

    fn find_pairs_through_zshape_center(
        self: &Self,
        points: &Vec<[i32; 2]>,
        rect_corner1: &[i32; 2],
        rect_corner2: &[i32; 2],
        result: &mut HashMap<[i32; 2], Vec<[i32; 2]>>,
        groups: &mut Vec<HashSet<[i32; 2]>>,
        points_groups: &mut HashMap<[i32; 2], Vec<usize>>,
    ) {
        let [min_x, min_y] = [
            rect_corner1[0].min(rect_corner2[0]) as f64,
            rect_corner1[1].min(rect_corner2[1]) as f64,
        ];
        let [max_x, max_y] = [
            rect_corner1[0].max(rect_corner2[0]) as f64,
            rect_corner1[1].max(rect_corner2[1]) as f64,
        ];

        let center_x = min_x + (max_x - min_x) / 2.0;
        let center_y = min_y + (max_y - min_y) / 2.0;

        let point_set: std::collections::HashSet<_> = points.iter().cloned().collect();
        let mut group = HashSet::new();
        for &point in &point_set {
            let [x, y] = point;
            let sym_x = (2.0 * center_x - x as f64).round() as i32;
            let sym_y = (2.0 * center_y - y as f64).round() as i32;

            if [sym_x, sym_y] != point && point_set.contains(&[sym_x, sym_y]) {
                let pv = result.entry(point).or_default();
                if !pv.contains(&[sym_x, sym_y]) {
                    pv.push([sym_x, sym_y]);
                    group.insert(point);
                }
            }
        }
        if group.len() > 0 {
            for p in &group {
                points_groups
                    .entry(p.to_owned())
                    .or_default()
                    .push(groups.len());
            }
            groups.push(group);
        }
    }

    fn find_pairs_through_corner<P>(
        self: &Self,
        points: &P,
        c: &[i32; 2],
        dir: &[i32; 2],
        result: &mut HashMap<[i32; 2], Vec<[i32; 2]>>,
        groups: &mut Vec<HashSet<[i32; 2]>>,
        points_groups: &mut HashMap<[i32; 2], Vec<usize>>,
    ) -> Vec<[f64; 2]>
    where
        P: Clone + IntoIterator<Item = [i32; 2]>,
    {
        let point_set: std::collections::HashSet<_> = points.clone().into_iter().collect();
        let mut cut_points = Vec::new();
        let mut group = HashSet::new();
        for &point in &point_set {
            if let Some(sym_point) =
                find_symmetric_points_about_perpendicular_bisector(&point, c, dir)
            {
                if sym_point != point && point_set.contains(&sym_point) {
                    group.insert(point);
                    result.entry(point).or_default().push(sym_point);
                }
                cut_points.push(midpoint(sym_point, point));
            }
        }
        if group.len() > 0 {
            for p in &group {
                points_groups
                    .entry(p.to_owned())
                    .or_default()
                    .push(groups.len());
            }
            groups.push(group);
        }
        cut_points
    }

    fn find_pairs_through_mid_points<P>(
        self: &Self,
        points: &P,
        mid_points: &Vec<[f64; 2]>,
        result: &mut HashMap<[i32; 2], Vec<[i32; 2]>>,
        groups: &mut Vec<HashSet<[i32; 2]>>,
        points_groups: &mut HashMap<[i32; 2], Vec<usize>>,
    ) where
        P: Clone + IntoIterator<Item = [i32; 2]>,
    {
        let mut tmp: HashMap<[i32; 2], Vec<[i32; 2]>> = HashMap::new();
        let point_set: Vec<_> = points.clone().into_iter().collect();
        let mut group = HashSet::new();
        let mut line_groups: HashMap<i32, usize> = HashMap::new();
        for i in 0..point_set.len() {
            for j in i + 1..point_set.len() {
                let point1 = point_set[i];
                let point2 = point_set[j];

                let midpoint = midpoint(point1, point2);
                if mid_points.contains(&midpoint) {
                    tmp.entry(point1).or_default().push(point2);
                    tmp.entry(point2).or_default().push(point1);
                    *line_groups.entry(point1[0]).or_default() += 1;
                    *line_groups.entry(point1[1]).or_default() += 1;
                    *line_groups.entry(point2[0]).or_default() += 1;
                    *line_groups.entry(point2[1]).or_default() += 1;
                }
            }
        }
        if line_groups.len() > 0 {
            let max_mems = line_groups.values().max().unwrap();
            for t in tmp.iter_mut() {
                if line_groups
                    .get(&t.0[0])
                    .unwrap()
                    .max(line_groups.get(&t.0[1]).unwrap())
                    == max_mems
                {
                    let mut rm_idx = vec![];
                    for i in 0..t.1.len() {
                        let l = t.1[i];
                        if line_groups
                            .get(&l[0])
                            .unwrap()
                            .max(line_groups.get(&l[1]).unwrap())
                            < max_mems
                        {
                            rm_idx.push(i);
                        }
                    }
                    for i in rm_idx {
                        t.1.remove(i);
                    }
                    group.insert(t.0.clone());
                    result.entry(*t.0).or_default().extend(t.1.clone());
                }
            }
        }
        if group.len() > 0 {
            for p in &group {
                points_groups
                    .entry(p.to_owned())
                    .or_default()
                    .push(groups.len());
            }
            groups.push(group);
        }
    }

    fn find_pairs_on_remaining(
        self: &Self,
        points: &HashSet<[i32; 2]>,
        result: &mut HashMap<[i32; 2], Vec<[i32; 2]>>,
        groups: &mut Vec<HashSet<[i32; 2]>>,
        points_groups: &mut HashMap<[i32; 2], Vec<usize>>,
    ) {
        let mut line_groups = Vec::new();
    
        let mut x_map: HashMap<i32, Vec<[i32; 2]>> = HashMap::new();
        let mut y_map: HashMap<i32, Vec<[i32; 2]>> = HashMap::new();
        
        for &point in points {
            let [x, y] = point;
            x_map.entry(x).or_default().push(point);
            y_map.entry(y).or_default().push(point);
        }
        
        line_groups.extend(x_map.into_values());
        line_groups.extend(y_map.into_values());
        
        line_groups.sort_by_key(|g| std::cmp::Reverse(g.len()));
        
        let mut group1 = line_groups.get(0).cloned().unwrap_or_default();
        group1.sort();
        let mut group2 = line_groups.get(1).cloned().unwrap_or_default();
        group2.sort();

        let mut group = HashSet::new();
        for i in 0..points.len()/2{
            let p1 = group1[i];
            let p2 = group2[i];
            result.entry(p1).or_default().push(p2);
            result.entry(p2).or_default().push(p1);
            group.insert(p1);
            group.insert(p2);
        }
        if group.len() > 0 {
            for p in &group {
                points_groups
                    .entry(p.to_owned())
                    .or_default()
                    .push(groups.len());
            }
            groups.push(group);
        }
    }

    //// So, we start with the corners, in the example image, there's 3 corners
    /// (7,3), (4,8) and (3,11).
    /// # Step 1: The simplest connections are starting from the corners then expanding out.
    /// `find_pairs_through_corner` is for step 1.
    /// 
    /// # Step 2: we can expand to connect more points since one side of the corner could be longer.
    /// So, for the first 2 steps, we need to get those points, `points_on_zshape_of_corner` is for that.
    /// It returns all the points and the bound of the rectangle eg: (0,4) and (0,11).
    /// The pairs for step 2 are ones that connect throught the center of the rectangle using
    /// `find_pairs_through_zshape_center`.
    /// 
    /// # Step 3: This is from my observation, that at least one corner whose perpendicular bisector
    /// has the same distance to 2 points of a pair -- same as step 1.
    /// And we also observe that the segment of the pair also cut at mid point of the corner's
    /// perpendicular bisector, so we have `find_pairs_through_mid_points`.
    /// But note that there's redundant pairs also cut at mid point like (4,0),(3,15) and (6,0),(1,15),
    /// but since (4,0),(3,15) already grouped in step 1 & 2, the remaining isnt enough to form a group.
    /// That's how line_groups used in the function to archive that.
    /// 
    /// # Final step: the remaining points are always (I guess) parallel so we can easyly group them
    /// and pair in order like in `find_pairs_on_remaining`.
    pub fn part2(self: &Self, p: &mut [i32; 2], path: &Vec<String>) {
        let mut edge_points: HashSet<_> = self.edge_points().into_iter().collect();
        let mut groups = Vec::new();
        let mut result = HashMap::new();
        let mut points_groups = HashMap::new();
        let mut mid_points = Vec::new();
        for (c, dir) in self.corner_points() {
            let ([max_x_pos, max_y_pos], points) = self.points_on_zshape_of_corner(&c, &dir);
            let mps = self.find_pairs_through_corner(
                &points,
                &c,
                &dir,
                &mut result,
                &mut groups,
                &mut points_groups,
            );
            mid_points.extend(mps);

            self.find_pairs_through_zshape_center(
                &points,
                &max_x_pos,
                &max_y_pos,
                &mut result,
                &mut groups,
                &mut points_groups,
            );
            for p in &points {
                if self.neighbour8s(p).len() != 3 {
                    edge_points.remove(p);
                } else if self.neighbour8s(p).len() == 3 && result.entry(*p).or_default().len() == 2
                {
                    edge_points.remove(p);
                }
            }
        }
        let gltmp1 = groups.len();
        self.find_pairs_through_mid_points(
            &edge_points,
            &mid_points,
            &mut result,
            &mut groups,
            &mut points_groups,
        );
        let gltmp2 = groups.len();
        if gltmp2 > gltmp1 {
            for p in &groups[groups.len() - 1] {
                if self.neighbour8s(p).len() != 3 {
                    edge_points.remove(p);
                } else if self.neighbour8s(p).len() == 3 && result.entry(*p).or_default().len() == 2
                {
                    edge_points.remove(p);
                }
            }
        }
        assert!(edge_points.len()%2==0, "{:?}", &edge_points);
        self.find_pairs_on_remaining(
            &edge_points,
            &mut result,
            &mut groups,
            &mut points_groups,
        );

        let mut dir = 0;
        for guide in path {
            match guide.as_str() {
                "R" => dir = turn(dir, &guide),
                "L" => dir = turn(dir, &guide),
                _ => {
                    let steps = str::parse::<u32>(&guide).unwrap();
                    (*p, dir) = self.walk3d(p, dir, steps, &result, &groups, &points_groups);
                }
            }
        }
        dbg!(1000*(p[1]+1)+4*(p[0]+1)+dir);
    }

    fn walk3d(
        self: &Self,
        pos: &[i32; 2],
        dir: i32,
        steps: u32,
        edge_mapping: &HashMap<[i32; 2], Vec<[i32; 2]>>,
        groups: &Vec<HashSet<[i32; 2]>>,
        points_groups: &HashMap<[i32; 2], Vec<usize>>,
    ) -> ([i32; 2], i32) {
        let mut new_pos = pos.to_owned();
        let mut new_dir = dir;
        for i in 0..steps {
            let _dir = match new_dir {
                0 => [1, 0],
                1 => [0, 1],
                2 => [-1, 0],
                3 => [0, -1],
                _ => panic!(),
            };
            let (np, dir) = self.step3d(&new_pos, _dir, edge_mapping, groups, points_groups);
            match self.val_at(np[0], np[1]) {
                "." => {
                    new_pos = np;
                    new_dir = dir
                }
                "#" => break,
                _ => (),
            }
        }
        (new_pos, new_dir)
    }

    fn step3d(
        self: &Self,
        pos: &[i32; 2],
        dir: [i32; 2],
        edge_mapping: &HashMap<[i32; 2], Vec<[i32; 2]>>,
        groups: &Vec<HashSet<[i32; 2]>>,
        points_groups: &HashMap<[i32; 2], Vec<usize>>,
    ) -> ([i32; 2], i32) {
        let x = pos[0];
        let y = pos[1];
        if x + dir[0] >= 0
            && y + dir[1] >= 0
            && x + dir[0] <= self.width as i32 - 1
            && y + dir[1] <= self.height as i32 - 1
            && self.val_at(x + dir[0], y + dir[1]) != " "
        {
            return (
                [x + dir[0], y + dir[1]],
                match dir {
                    [1, 0] => 0,
                    [0, 1] => 1,
                    [-1, 0] => 2,
                    [0, -1] => 3,
                    _ => panic!(),
                },
            );
        } else {
            let npv = edge_mapping.get(pos).unwrap();
            match npv {
                npv if npv.len() == 1
                    && self.neighbour8s(pos).len() != 3
                    && self.neighbour8s(&npv[0]).len() != 3 =>
                {
                    let np = npv[0];
                    (
                        npv[0],
                        if np[1] == self.height as i32 - 1 || self.val_at(np[0], np[1] + 1) == " " {
                            3
                        } else if np[1] == 0 || self.val_at(np[0], np[1] - 1) == " " {
                            1
                        } else if np[0] == self.width as i32 - 1
                            || self.val_at(np[0] + 1, np[1]) == " "
                        {
                            2
                        } else if np[0] == 0 || self.val_at(np[0] - 1, np[1]) == " " {
                            0
                        } else {
                            panic!()
                        },
                    )
                }
                npv if npv.len() == 1
                    && self.neighbour8s(pos).len() != 3
                    && self.neighbour8s(&npv[0]).len() == 3 =>
                {
                    let pg = points_groups.get(pos).unwrap()[0];
                    let sp = if points_groups
                        .get(&[x + dir[1], y + dir[0]])
                        .unwrap_or(&vec![])
                        .contains(&pg)
                    {
                        [x + dir[1], y + dir[0]]
                    } else {
                        [x - dir[1], y - dir[0]]
                    };
                    let dir = self.step3d(&sp, dir, edge_mapping, groups, points_groups).1;
                    (npv[0], dir)
                }
                npv => {
                    let sp = if x + dir[1] >= 0
                        && x + dir[1] <= self.width as i32 - 1
                        && y + dir[0] >= 0
                        && y + dir[0] <= self.height as i32 - 1
                        && self.val_at(x + dir[1], y + dir[0]) != " "
                    {
                        [x + dir[1], y + dir[0]]
                    } else {
                        [x - dir[1], y - dir[0]]
                    };
                    let spnpr = self.step3d(&sp, dir, edge_mapping, groups, points_groups);
                    let spnpg = points_groups.get(&spnpr.0).unwrap()[0];
                    let mut np = [-1, -1];
                    for _np in npv {
                        if points_groups.get(_np).unwrap()[0] == spnpg {
                            np = *_np;
                        }
                    }
                    if np == [-1, -1] {
                        np = [
                            ((self.width as i32 - 1) - dir[0] * (self.width as i32 - 1)) / 2,
                            ((self.height as i32 - 1) - dir[1] * (self.height as i32 - 1)),
                        ];
                    }
                    (np, spnpr.1)
                }
            }
        }
    }

    fn right(self: &Self, pos: &[i32; 2]) -> [i32; 2] {
        let mut x = pos[0];
        let y = pos[1];
        x = if x == self.width as i32 - 1 { 0 } else { x + 1 };
        while self.val_at(x, y) == " " {
            x = if x == self.width as i32 - 1 { 0 } else { x + 1 };
        }
        [x, y]
    }

    fn left(self: &Self, pos: &[i32; 2]) -> [i32; 2] {
        let mut x = pos[0];
        let y = pos[1];
        x = if x == 0 { self.width as i32 - 1 } else { x - 1 };
        while self.val_at(x, y) == " " {
            x = if x == 0 { self.width as i32 - 1 } else { x - 1 };
        }
        [x, y]
    }

    fn down(self: &Self, pos: &[i32; 2]) -> [i32; 2] {
        let x = pos[0];
        let mut y = pos[1];
        y = if y == self.height as i32 - 1 {
            0
        } else {
            y + 1
        };
        while self.val_at(x, y) == " " {
            y = if y == self.height as i32 - 1 {
                0
            } else {
                y + 1
            };
        }
        [x, y]
    }

    fn up(self: &Self, pos: &[i32; 2]) -> [i32; 2] {
        let x = pos[0];
        let mut y = pos[1];
        y = if y == 0 {
            self.height as i32 - 1
        } else {
            y - 1
        };
        while self.val_at(x, y) == " " {
            y = if y == 0 {
                self.height as i32 - 1
            } else {
                y - 1
            };
        }
        [x, y]
    }

    pub fn walk(self: &Self, pos: &[i32; 2], dir: i32, steps: u32) -> [i32; 2] {
        let mut new_pos = pos.to_owned();
        for i in 0..steps {
            match dir {
                0 => {
                    let np = self.right(&new_pos);
                    match self.val_at(np[0], np[1]) {
                        "." => new_pos = np,
                        "#" => break,
                        _ => (),
                    }
                }
                1 => {
                    // v
                    let np = self.down(&new_pos);
                    match self.val_at(np[0], np[1]) {
                        "." => new_pos = np,
                        "#" => break,
                        _ => (),
                    }
                }
                2 => {
                    // <
                    let np = self.left(&new_pos);
                    match self.val_at(np[0], np[1]) {
                        "." => new_pos = np,
                        "#" => break,
                        _ => (),
                    }
                }
                3 => {
                    // ^
                    let np = self.up(&new_pos);
                    match self.val_at(np[0], np[1]) {
                        "." => new_pos = np,
                        "#" => break,
                        _ => (),
                    }
                }
                _ => panic!(),
            }
        }
        new_pos
    }
}

pub fn parse_path(path_str: &str) -> Vec<String> {
    let re = Regex::new(r"(\d+|R|L)").unwrap();

    let arr = re
        .captures_iter(path_str)
        .map(|c| {
            let (_, [t]) = c.extract();
            t.to_string()
        })
        .collect::<Vec<String>>();
    arr
}

pub fn turn(dir: i32, to_dir: &str) -> i32 {
    match (dir, to_dir) {
        (0, "R") => 1, // >
        (1, "R") => 2, // v
        (2, "R") => 3, // <
        (3, "R") => 0, // ^
        (0, "L") => 3, // >
        (1, "L") => 0,
        (2, "L") => 1,
        (3, "L") => 2,
        _ => panic!(),
    }
}

fn find_symmetric_points_about_perpendicular_bisector(
    p: &[i32; 2],
    c: &[i32; 2],
    dir: &[i32; 2],
) -> Option<[i32; 2]> {
    let [dx, dy] = *dir;
    let [cx, cy] = *c;
    let [px, py] = *p;

    let [a, b] = [-dy, dx];

    let t = (a as f64 * (px as f64 - cx as f64) + b as f64 * (py as f64 - cy as f64))
        / (a as f64 * a as f64 + b as f64 * b as f64);

    // Điểm đối xứng
    let sym_x = (px as f64 - 2.0 * a as f64 * t).round() as i32;
    let sym_y = (py as f64 - 2.0 * b as f64 * t).round() as i32;

    Some([sym_x, sym_y])
}

fn midpoint<T>(a: [T; 2], b: [T; 2]) -> [f64; 2]
where
    T: Into<f64> + Copy,
{
    [
        (a[0].into() + b[0].into()) / 2.0,
        (a[1].into() + b[1].into()) / 2.0,
    ]
}

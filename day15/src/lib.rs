use std::rc::Rc;

#[derive(Debug)]
pub struct Coord {
    pub x: i64,
    pub y: i64,
}

impl Coord {
    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    pub fn distance_to(&self, c: &Coord) -> i64 {
        (self.x - c.x).abs() + (self.y - c.y).abs()
    }
}

#[derive(Debug)]
pub struct Beacon {
    pub coord: Coord,
}

#[derive(Debug)]
pub struct Sensor {
    pub coord: Coord,
    pub beacon: Rc<Beacon>,
}

impl Sensor {
    pub fn min_distance(&self) -> i64 {
        self.coord.distance_to(&self.beacon.coord)
    }

    pub fn find_cut_xline(&self, y: i64) -> Option<Vec<Coord>> {
        let d = self.min_distance() - (y - self.coord.y).abs();
        if d < 0 {
            return None;
        } else {
            return Some(vec![
                Coord::new(self.coord.x - d, y),
                Coord::new(self.coord.x + d, y),
            ]);
        }
    }
}

#[derive(Debug)]
pub enum Point {
    Sensor(Rc<Sensor>),
    Beacon(Rc<Beacon>),
}

pub struct Grid {
    points: Vec<Point>,
    sensors: Vec<Rc<Sensor>>,
}

impl Grid {
    pub fn new() -> Self {
        Grid {
            points: vec![],
            sensors: vec![],
        }
    }

    pub fn get_beacon_at(&self, x: i64, y: i64) -> Option<Rc<Beacon>> {
        let p = self.points.iter().find(|p| match p {
            Point::Beacon(b) => b.coord.x == x && b.coord.y == y,
            _ => false,
        });
        match p {
            Some(p) => match p {
                Point::Beacon(b) => Some(b.clone()),
                _ => None,
            },
            None => None,
        }
    }

    pub fn get_or_create_beacon_at(&mut self, x: i64, y: i64) -> Rc<Beacon> {
        match self.get_beacon_at(x, y) {
            Some(b) => b,
            None => {
                let b = Rc::new(Beacon {
                    coord: Coord::new(x, y),
                });
                self.points.push(Point::Beacon(b.clone()));
                b
            }
        }
    }

    pub fn add_sensor(&mut self, sensor: Sensor) {
        let s_rc = Rc::new(sensor);
        self.points.push(Point::Sensor(s_rc.clone()));
        self.sensors.push(s_rc.clone());
    }

    pub fn cover_range_on_xline(&self, y: i64) -> Vec<Vec<i64>> {
        let mut xes = vec![];

        self.sensors.iter().for_each(|s| match s.find_cut_xline(y) {
            Some(coords) => xes.push(coords.iter().map(|c| c.x).collect::<Vec<_>>()),
            None => (),
        });
        xes.sort();

        let mut r = vec![];

        for i in 0..xes.len() {
            if r.len() == 0 {
                r.push(xes[i].clone());
                continue;
            }
            let xes1 = r.last_mut().unwrap();
            let xes2 = &xes[i];
            if xes1[1] < xes2[0] {
                r.push(xes2.clone());
            } else if xes1[1] < xes2[1] {
                xes1[1] = xes2[1];
            }
        }
        return r;
    }
}

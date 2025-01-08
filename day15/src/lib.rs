use std::rc::Rc;

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
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

#[derive(Debug, PartialEq, Eq)]
pub struct Beacon {
    pub coord: Coord,
}

#[derive(Debug, PartialEq, Eq)]
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

    pub fn iter_points(&self) -> std::slice::Iter<'_, Point> {
        self.points.iter()
    }

    pub fn iter_sensors(&self) -> std::slice::Iter<'_, Rc<Sensor>> {
        self.sensors.iter()
    }

    pub fn cover_range_on_xline(&self, y: i64) -> Vec<Vec<i64>> {
        let mut xes = vec![];

        self.iter_sensors().for_each(|s| {
            let some_coords = s.find_cut_xline(y);
            if some_coords.is_some() {
                xes.push(
                    some_coords
                        .unwrap()
                        .iter()
                        .map(|c| c.x)
                        .collect::<Vec<i64>>(),
                );
            }
        });
        xes.sort();

        let mut r = vec![];
        xes.into_iter().reduce(|xes1, xes2| {
            let xes_next;
            if xes1[1] < xes2[0] {
                xes_next = xes2.clone();
                if r.len() == 0 {
                    r.push(xes1);
                }
                r.push(xes2);
            } else {
                xes_next = vec![xes1[0], if xes1[1] > xes2[1] { xes1[1] } else { xes2[1] }];
                if r.len() == 0 {
                    r.push(xes_next.clone());
                } else {
                    let a = r.last_mut().unwrap();
                    a[0] = xes1[0];
                    a[1] = xes2[1];
                }
            }
            xes_next
        });
        return r;
    }
}

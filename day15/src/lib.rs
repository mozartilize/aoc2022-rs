use std::vec;
use std::rc::Rc;

#[derive(Debug)]
pub struct Coord {
    pub x: i32,
    pub y: i32,
}

impl Coord {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn distance_to(&self, c: &Coord) -> i32 {
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
    pub fn min_distance(&self) -> i32 {
        self.as_ref().distance_to(&self.beacon.coord)
    }
}

impl AsRef<Coord> for Beacon {
    fn as_ref(&self) -> &Coord {
        &self.coord
    }
}

impl AsRef<Coord> for Sensor {
    fn as_ref(&self) -> &Coord {
        &self.coord
    }
}

#[derive(Debug)]
pub enum Point {
    Sensor(Rc<Sensor>),
    Beacon(Rc<Beacon>),
}

pub struct Grid
{
    points: Vec<Point>,
    sensors: Vec<Rc<Sensor>>
}

impl Grid {
    pub fn new() -> Self {
        Grid { points: vec![], sensors: vec![] }
    }

    pub fn get_beacon_at(&self, x: i32, y: i32) -> Option<Rc<Beacon>> {
        let p = self.points.iter().find(|p| {
            match p {
                Point::Beacon(b) => b.coord.x == x && b.coord.y == y,
                _ => false,
            }
        });
        match p {
            Some(p) => match p {
                Point::Beacon(b) => Some(b.clone()),
                _ => None,
            },
            None => None,
        }
    }

    pub fn get_or_create_beacon_at(&mut self, x: i32, y: i32) -> Rc<Beacon> {
        match self.get_beacon_at(x, y) {
            Some(b) => b,
            None => {
                let b = Rc::new(Beacon{coord: Coord::new(x, y)});
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
}
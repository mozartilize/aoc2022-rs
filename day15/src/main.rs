use day15::{Grid, Coord, Beacon, Sensor, Point};

fn main() {
    let mut g = Grid::new();

    let b = g.get_or_create_beacon_at(2345659, 4354867);
    g.add_sensor(Sensor{coord: Coord::new(2391367, 3787759), beacon: b});
    let b = g.get_or_create_beacon_at(1654342, 3193298);
    g.add_sensor(Sensor{coord: Coord::new(1826659, 2843839), beacon: b});
    let b = g.get_or_create_beacon_at(31358, 2000000);
    g.add_sensor(Sensor{coord: Coord::new(980874, 2369046), beacon: b});
    let b = g.get_or_create_beacon_at(3064453, 2107409);
    g.add_sensor(Sensor{coord: Coord::new(2916267, 2516612), beacon: b});
    let b = g.get_or_create_beacon_at(3064453, 2107409);
    g.add_sensor(Sensor{coord: Coord::new(3304786, 844925), beacon: b});
    let b = g.get_or_create_beacon_at(31358, 2000000);
    g.add_sensor(Sensor{coord: Coord::new(45969, 76553), beacon: b});
    let b = g.get_or_create_beacon_at(2483905, 2123337);
    g.add_sensor(Sensor{coord: Coord::new(2647492, 1985479), beacon: b});
    let b = g.get_or_create_beacon_at(31358, 2000000);
    g.add_sensor(Sensor{coord: Coord::new(15629, 2015720), beacon: b});
    let b = g.get_or_create_beacon_at(3528871, 3361675);
    g.add_sensor(Sensor{coord: Coord::new(3793239, 3203486), beacon: b});
    let b = g.get_or_create_beacon_at(4731853, 1213406);
    g.add_sensor(Sensor{coord: Coord::new(3998240, 15268), beacon: b});
    let b = g.get_or_create_beacon_at(3528871, 3361675);
    g.add_sensor(Sensor{coord: Coord::new(3475687, 3738894), beacon: b});
    let b = g.get_or_create_beacon_at(3528871, 3361675);
    g.add_sensor(Sensor{coord: Coord::new(3993022, 3910207), beacon: b});
    let b = g.get_or_create_beacon_at(31358, 2000000);
    g.add_sensor(Sensor{coord: Coord::new(258318, 2150378), beacon: b});
    let b = g.get_or_create_beacon_at(2483905, 2123337);
    g.add_sensor(Sensor{coord: Coord::new(1615638, 1108834), beacon: b});
    let b = g.get_or_create_beacon_at(1654342, 3193298);
    g.add_sensor(Sensor{coord: Coord::new(1183930, 3997648), beacon: b});
    let b = g.get_or_create_beacon_at(1654342, 3193298);
    g.add_sensor(Sensor{coord: Coord::new(404933, 3377916), beacon: b});
    let b = g.get_or_create_beacon_at(3528871, 3361675);
    g.add_sensor(Sensor{coord: Coord::new(3829801, 2534117), beacon: b});
    let b = g.get_or_create_beacon_at(2483905, 2123337);
    g.add_sensor(Sensor{coord: Coord::new(2360813, 2494240), beacon: b});
    let b = g.get_or_create_beacon_at(1654342, 3193298);
    g.add_sensor(Sensor{coord: Coord::new(2286195, 3134541), beacon: b});
    let b = g.get_or_create_beacon_at(31358, 2000000);
    g.add_sensor(Sensor{coord: Coord::new(15626, 1984269), beacon: b});
    let b = g.get_or_create_beacon_at(3528871, 3361675);
    g.add_sensor(Sensor{coord: Coord::new(3009341, 3849969), beacon: b});
    let b = g.get_or_create_beacon_at(1884716, -881769);
    g.add_sensor(Sensor{coord: Coord::new(1926292, 193430), beacon: b});
    let b = g.get_or_create_beacon_at(3528871, 3361675);
    g.add_sensor(Sensor{coord: Coord::new(3028318, 3091480), beacon: b});

    let p = g.iter_points().find(|p| {
        match p {
            Point::Beacon(b) => b.coord.y == 2000000,
            _ => false
        }
    }).unwrap();
    let b = match p {
        Point::Beacon(b) => Some(b),
        _ => None,
    }.unwrap();
    let mut x_forward = b.coord.x;
    let mut x_backward = b.coord.x;
    loop {
        x_forward += 1;
        if g.iter_sensors().filter(|s| {
            s.as_ref().as_ref().distance_to(&Coord::new(x_forward, b.coord.y)) <= s.min_distance()
        }).peekable().peek().is_none() {
            break;
        }
    }
    loop {
        x_backward -= 1;
        if g.iter_sensors().filter(|s| {
            s.as_ref().as_ref().distance_to(&Coord::new(x_backward, b.coord.y)) <= s.min_distance()
        }).peekable().peek().is_none() {
            break;
        }
    }
    dbg!(x_forward);
    dbg!(x_backward);
    dbg!(x_forward-1 - (x_backward+1));
}

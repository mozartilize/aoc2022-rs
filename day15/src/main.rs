use day15::{Grid, Coord, Sensor};

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

    // part1
    dbg!(g.cover_range_on_xline(2000000));
    
    // part2
    for y in 0..=4000000 {
        let r = g.cover_range_on_xline(y);
        if r.len() > 1 {
            dbg!(y, &r);
        }
    }
}
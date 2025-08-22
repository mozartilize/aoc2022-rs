use day21::{parse, part_1, MonkeyJob};

fn main() {
    let (mut v, ridx, hidx) = parse(std::io::stdin().lines());
    let r = part_1(&v);
    dbg!(r.get(&ridx).unwrap());

    // loop hval to 609823 to find the pattern
    // it starts from 10719 (left hand side value = 87222463338523)
    // and ends at 310123 (left hand side value = 87222457821801)
    // and the value from right hand side is does not change = 31343426392931
    // step count =                      total delta / step delta
    //            = (87222463338523 - 31343426392931)/(87222463338523 - 87222457821801)
    //            = 10124024
    // closest value = 10719 + 10124024 * (310123 - 10719)
    //               = 3032671799667
    let mut hval: f64 = 3032671799667.0;
    loop {
        v[hidx] = MonkeyJob::LITERAL(hval);
        let r = part_1(&v);
        // dbg!(r.get(&ridx));
        // dbg!(&v[ridx]);
        let req = &v[ridx];
        match req {
            MonkeyJob::EQUATION(eq) => {
                if r.get(&eq.left).unwrap().fract() == 0.0 {
                    println!(
                        "{}, {}, {}, {}",
                        hval,
                        r.get(&eq.left).unwrap(),
                        r.get(&eq.right).unwrap(),
                        r.get(&eq.left).unwrap() - r.get(&eq.right).unwrap()
                    );
                    println!();
                }
                if r.get(&eq.left).unwrap() == r.get(&eq.right).unwrap() {
                    break;
                }
            }
            _ => (),
        };
        hval += 1.0;
    }
}

use day20::solver;

fn main() {
    let mut v = Vec::new();
    for line_result in std::io::stdin().lines() {
        let line = line_result.unwrap();
        v.push(line.parse::<i32>().unwrap());
    }
    println!("{}", v.len());
    dbg!(solver(&v, 1, 1));
    dbg!(solver(&v, 811589153, 10));
}

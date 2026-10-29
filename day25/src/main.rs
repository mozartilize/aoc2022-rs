use std::collections::HashMap;

fn to10decimal(snafu: &str, map: &HashMap<char, i64>) -> i64 {
    let mut sum = 0;
    for i in 0..snafu.len() {
        sum += 5_i64.pow((snafu.len() - i - 1) as u32)
            * map.get(&snafu.chars().nth(i).unwrap()).unwrap();
    }
    sum
}

fn decimal2sanfu(d: i64, map: &HashMap<i64, char>) -> String {
    let l = (d as f64).log(5.0).round() as i32;
    let mut c = vec![];
    let mut r = d as f64;
    for i in (0..=l).rev() {
        let a = (r / 5_f64.powi(i)).round();
        r = r - a * 5_f64.powi(i);
        c.push(a);
    }
    return c.iter().map(|v| map.get(&(*v as i64)).unwrap()).collect();
}
fn main() {
    let map = HashMap::from([('2', 2), ('1', 1), ('=', -2), ('-', -1), ('0', 0)]);
    let x = to10decimal("1=-0-2", &map);
    println!("{x}");
    let mut sum = 0;
    for line_result in std::io::stdin().lines() {
        let line = line_result.unwrap();
        let x = to10decimal(&line, &map);
        println!("{}: {}", &line, x);
        sum += x;
    }
    println!("{sum}");
    let inverted_map: HashMap<i64, char> = map.into_iter().map(|(k, v)| (v, k)).collect();

    println!("{}", decimal2sanfu(sum, &inverted_map));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn k() {
        let map = HashMap::from([('2', 2), ('1', 1), ('=', -2), ('-', -1), ('0', 0)]);

        let x = to10decimal("1=-0-2", &map);
        println!("{x}");
        assert_eq!(x, 1747);
        let x = to10decimal("12111", &map);
        assert_eq!(x, 906);

        let x = to10decimal("2=0=", &map);
        assert_eq!(x, 198);

        let x = to10decimal("21", &map);
        assert_eq!(x, 11);

        let x = to10decimal("2=01", &map);
        assert_eq!(x, 201);

        let inverted_map: HashMap<i64, char> = map.into_iter().map(|(k, v)| (v, k)).collect();
        let x = decimal2sanfu(4890, &inverted_map);
        assert_eq!(x, "2=-1=");
    }
}

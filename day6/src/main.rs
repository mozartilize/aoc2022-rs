use std::{io::{self, Read}, collections::HashMap};

fn main() {
    let some_part = std::env::args().last().unwrap();
    let check_len;
    if some_part == "1" {
        check_len = 4;
    } else if some_part == "2" {
        check_len = 14;
    }
    else {
        return
    }
    let mut len = 0;
    let mut v: Vec<u8> = vec![];
    let mut v2 = HashMap::new();
    loop {
        let mut buf = vec![0u8; 1];
        let result = io::stdin().read_exact(&mut buf);
        if result.is_ok() {
            len += 1;
            v.append(&mut buf);
            *v2.entry(v[v.len()-1]).or_insert(0) += 1;
            if v.len() > check_len {
                let x = v.remove(0);
                if v2[&x] == 1 {
                    v2.remove(&x);
                }
                else {
                    *v2.entry(x).or_insert(0) -= 1;
                }
            }
            if v.len() >= check_len && v2.len() == check_len {
                dbg!(len);
                break;
            }
        }
        else {
            break;
        }
    }
}

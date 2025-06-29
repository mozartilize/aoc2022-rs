use std::collections::HashMap;

fn get_new_idx<T: Into<i64>>(curr_idx: usize, step: T, length: usize) -> usize {
    let step = step.into();
    let len = length as i64 - 1;
    let offset = step.rem_euclid(len);
    let mut new_pos = curr_idx as i64 + offset;
    if new_pos >= len {
        new_pos -= len;
    }
    new_pos as usize
}

fn find_x(zero_idx: usize, id_loc: &HashMap<usize, usize>) -> usize {
    let loc_id: HashMap<&usize, &usize> = id_loc.iter().map(|(k, v)| (v, k)).collect();
    let new_zero_idx = id_loc.get(&zero_idx).unwrap();
    let x_idx = new_zero_idx + 1;
    let origin_idx = **loc_id.get(&x_idx).unwrap();
    return origin_idx;
}

fn to_new_list<T: Copy>(l: &Vec<T>, id_loc: &HashMap<usize, usize>) -> Vec<T> {
    let loc_id: HashMap<&usize, &usize> = id_loc.iter().map(|(k, v)| (v, k)).collect();
    (0..l.len())
        .map(|idx| l[**loc_id.get(&idx).unwrap()])
        .collect::<Vec<_>>()
}

pub fn part_1(l: &Vec<i32>) {
    // let mut newl = l.clone();
    let mut id_loc: HashMap<usize, usize> = l.iter().enumerate().map(|(i, _)| (i, i)).collect();
    let length = l.len();
    // let mut loop_idx = 0;
    let zero_idx = l.iter().position(|n| *n == 0).unwrap();
    for i in 0..l.len() {
        let loop_idx = i;
        let curr_idx = id_loc.get(&loop_idx).unwrap().to_owned();
        let v = l[loop_idx];
        if v == 0 {
            continue;
        }
        let new_idx = get_new_idx(curr_idx, v, length);
        // println!("moving {} from {} to {}", v, curr_idx, new_idx);
        let loc_id: HashMap<usize, usize> = id_loc.iter().map(|(k, v)| (*v, *k)).collect();
        if new_idx > curr_idx {
            for idx in curr_idx + 1..=new_idx {
                *id_loc.get_mut(loc_id.get(&idx).unwrap()).unwrap() -= 1;
            }
        } else if new_idx < curr_idx {
            for idx in new_idx..=curr_idx - 1 {
                *id_loc.get_mut(loc_id.get(&idx).unwrap()).unwrap() += 1;
            }
        }
        *id_loc.get_mut(loc_id.get(&curr_idx).unwrap()).unwrap() = new_idx;
    }
    // dbg!(&id_loc);
    let v = to_new_list(l, &id_loc);
    // let loc_id: HashMap<usize, usize> = id_loc.iter().map(|(k, v)| (*v, *k)).collect();
    let new_zero_idx = id_loc.get(&zero_idx).unwrap();
    dbg!(zero_idx, new_zero_idx);
    let idx_1000th = (1000%l.len()+new_zero_idx) % l.len();
    let value_at_idx_1000th = v[idx_1000th];
    dbg!(&value_at_idx_1000th);

    let idx_2000th = (2000%l.len()+new_zero_idx) % l.len();
    let value_at_idx_2000th = v[idx_2000th];
    dbg!(&value_at_idx_2000th);

    let idx_3000th = (3000%l.len()+new_zero_idx) % l.len();
    let value_at_idx_3000th = v[idx_3000th];
    dbg!(&value_at_idx_3000th);

    dbg!(value_at_idx_1000th+value_at_idx_2000th+value_at_idx_3000th);
}

pub fn part_2(l: &Vec<i32>) {
    let l = l.iter().map(|v| (*v as i64)*811589153).collect::<Vec<_>>();
    let mut id_loc: HashMap<usize, usize> = l.iter().enumerate().map(|(i, _)| (i, i)).collect();
    let length = l.len();
    // let mut loop_idx = 0;
    let zero_idx = l.iter().position(|n| *n == 0).unwrap();
    for _ in 0..10 {
        for i in 0..l.len() {
            let loop_idx = i;
            let curr_idx = id_loc.get(&loop_idx).unwrap().to_owned();
            let v = l[loop_idx];
            if v == 0 {
                continue;
            }
            let new_idx = get_new_idx(curr_idx, v, length);
            // println!("moving {} from {} to {}", v, curr_idx, new_idx);
            let loc_id: HashMap<usize, usize> = id_loc.iter().map(|(k, v)| (*v, *k)).collect();
            if new_idx > curr_idx {
                for idx in curr_idx + 1..=new_idx {
                    *id_loc.get_mut(loc_id.get(&idx).unwrap()).unwrap() -= 1;
                }
            } else if new_idx < curr_idx {
                for idx in new_idx..=curr_idx - 1 {
                    *id_loc.get_mut(loc_id.get(&idx).unwrap()).unwrap() += 1;
                }
            }
            *id_loc.get_mut(loc_id.get(&curr_idx).unwrap()).unwrap() = new_idx;
        }
    }
    // dbg!(&id_loc);
    let v = to_new_list(&l, &id_loc);
    // let loc_id: HashMap<usize, usize> = id_loc.iter().map(|(k, v)| (*v, *k)).collect();
    let new_zero_idx = id_loc.get(&zero_idx).unwrap();
    dbg!(zero_idx, new_zero_idx);
    let idx_1000th = (1000%l.len()+new_zero_idx) % l.len();
    let value_at_idx_1000th = v[idx_1000th];
    dbg!(&value_at_idx_1000th);

    let idx_2000th = (2000%l.len()+new_zero_idx) % l.len();
    let value_at_idx_2000th = v[idx_2000th];
    dbg!(&value_at_idx_2000th);

    let idx_3000th = (3000%l.len()+new_zero_idx) % l.len();
    let value_at_idx_3000th = v[idx_3000th];
    dbg!(&value_at_idx_3000th);

    dbg!(value_at_idx_1000th+value_at_idx_2000th+value_at_idx_3000th);
}

pub fn solver(l: &Vec<i32>, decryption_key: i64, mix_cnt: usize) -> i64 {
    let nums: Vec<_> = l.iter().map(|num| *num as i64 * decryption_key).collect();
    // indexes into nums
    let mut mixed: Vec<_> = (0..nums.len()).collect();
    for _ in 0..mix_cnt {
        for (idx, &num) in nums.iter().enumerate() {
            // find mixed that corresponds to the number in nums
            let mixed_idx = mixed.iter().position(|&mix_num| mix_num == idx).unwrap();
            // remove that index from mixed
            mixed.remove(mixed_idx);
            // add num offset to that number and add it back
            let new_mixed_idx = (mixed_idx as i64 + num).rem_euclid(mixed.len() as i64) as usize;
            mixed.insert(new_mixed_idx, idx);
        }
    }

    let zero_idx = nums.iter().position(|&num| num == 0).unwrap();
    let zero_mixed_idx = mixed
        .iter()
        .position(|&mix_num| mix_num == zero_idx)
        .unwrap();

    [1000, 2000, 3000]
        .iter()
        .map(|offset| {
            let mixed_idx = (zero_mixed_idx + offset) % mixed.len();
            let nums_idx = mixed[mixed_idx];
            nums[nums_idx]
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_new_idx() {
        let x = get_new_idx(2, -3, 7);
        assert_eq!(x, 5);

        let x = get_new_idx(6, 4, 7);
        assert_eq!(x, 4);

        let x = get_new_idx(4, 4, 7);
        assert_eq!(x, 2);

        let x = get_new_idx(3, 3, 7);
        assert_eq!(x, 6);

        let x = get_new_idx(3, -2, 7);
        assert_eq!(x, 1);

        let x = get_new_idx(1, -3, 7);
        assert_eq!(x, 4);

        let x = get_new_idx(2, -2, 7);
        assert_eq!(x, 6);
        
        let x = get_new_idx(5, 4, 7);
        assert_eq!(x, 3);
    }

    #[test]
    fn test_part_1() {
        part_1(&vec![1, 2, -3, 3, -2, 0, 4]);
    }

    #[test]
    fn test_part_2() {
        part_2(&vec![1, 2, -3, 3, -2, 0, 4]);
    }
}

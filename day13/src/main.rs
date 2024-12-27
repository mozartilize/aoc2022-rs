use itertools::Itertools;

use day13::Node;

fn main() {
    let mut sum = 0;
    let mut i = 1;
    // let mut data_grouped: Vec<(bool, _)> = Vec::new();
    // for (ok, chunk) in std::io::stdin()
    //     .lines()
    //     .chunk_by(|l|
    //         l.as_ref().is_ok_and(|l| l != "")
    //     ).into_iter() {
    //         data_grouped.push((ok, chunk.collect::<Vec<_>>()));
    //     }
    // println!("{:?}", data_grouped);
    let mut packets = Vec::new();
    let dividers = vec![
        Node::List(vec![Node::Number(2)]),
        Node::List(vec![Node::Number(6)]),
    ];
    for (ok, group) in
        std::io::stdin()
        .lines()
        .chunk_by(|l|
            l.as_ref().is_ok_and(|l| l != "")
        )
        .into_iter()
    {
        if !ok {
            continue;
        }
        let mut nodes = group.map(|line| {
            return serde_json::from_str::<Node>(&line.unwrap()).unwrap();
        });
        let l = nodes.next().unwrap();
        let r = nodes.next().unwrap();
        println!("\n== Pair {i} ==");
        println!("l = {l:?}");
        println!("r = {r:?}");
        println!("l < r = {}", l < r);
        if l < r {
            sum += i;
        }
        i += 1;

        packets.push(l);
        packets.push(r);
    }
    dbg!(sum);

    packets.extend(dividers.clone());
    packets.sort();
    let decoder_key = dividers
        .iter()
        .map(|d| packets.binary_search(d).unwrap() + 1)
        .product::<usize>();
    dbg!(decoder_key);
}

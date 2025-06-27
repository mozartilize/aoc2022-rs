use day19::{part_1, part_2};

fn main() {
    let mut blueprints = Vec::new();

    for line_result in std::io::stdin().lines() {
        let line = line_result.unwrap();
        let mut iter = line.split_ascii_whitespace();

        // ore bots cost ore
        let ore_bot_costs = [iter.nth(6).unwrap().parse().unwrap(), 0, 0, 0];
        // clay bots cost ore
        let clay_bot_costs = [iter.nth(5).unwrap().parse().unwrap(), 0, 0, 0];
        // obsidian bots cost ore and clay
        let obsidian_bot_costs = [
            iter.nth(5).unwrap().parse().unwrap(),
            iter.nth(2).unwrap().parse().unwrap(),
            0,
            0,
        ];
        // geode bots cost ore and obsidian
        let geode_bot_costs = [
            iter.nth(5).unwrap().parse().unwrap(),
            0,
            iter.nth(2).unwrap().parse().unwrap(),
            0,
        ];

        let blueprint = [
            ore_bot_costs,
            clay_bot_costs,
            obsidian_bot_costs,
            geode_bot_costs,
        ];
        blueprints.push(blueprint);
    }

    let x = part_1(&blueprints, 24);
    dbg!(x);

    let x = part_2(&blueprints, 32);
    dbg!(x);
}

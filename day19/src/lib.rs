use std::collections::VecDeque;

struct State {
    // [ore, clay, obsidian, geode]
    inventory: [u16; 4],
    // [ore_bots, clay_bots, obsidian_bots, geode_bots]
    bots: [u16; 4],
    // elapsed time in minutes
    elapsed: u16,
}

pub fn max_geodes(blueprint: &[[u16; 4]; 4], max_time: u16) -> u16 {
    // calculate the maximum amount for every type of bot so that the creation of a new bot of any type is never bottlenecked
    // it doesn't make sense to build more bots than that maximum if the resources a bot type generates are
    // enough to cover that type (ore, clay, obsidian) cost for any possible bot (per question, you can only build 1 bot per turn)
    // for geode bots, there is no logical maximum amount
    // [ore, clay, obsidian, geode]
    let mut max_robots = [u16::MAX; 4];
    for i in 0..3 {
        max_robots[i] = blueprint.iter().map(|cost| cost[i]).max().unwrap();
    }
    let mut max_geodes = 0;

    let mut q = VecDeque::new();
    q.push_back(State {
        inventory: [0, 0, 0, 0],
        bots: [1, 0, 0, 0],
        elapsed: 0,
    });

    while let Some(State {
        inventory,
        bots,
        elapsed,
    }) = q.pop_front()
    {
        // for every bot cost, run simulation
        for i in 0..blueprint.len() {
            // if we already have enough of this bot type, skip
            if bots[i] == max_robots[i] {
                continue;
            }

            let costs = &blueprint[i];

            // Find the limiting resource type for the costs.
            let wait_time = (0..3)
                .map(|idx| {
                    match costs[idx] {
                        // state has enough of current resource in inventory to cover that part of the target bot cost. 0 wait time
                        cost if cost <= inventory[idx] => 0,
                        // no target bot type made yet
                        // we can't build it (it takes more than max_time to build it).
                        _ if bots[idx] == 0 => max_time + 1,
                        // Need to wait for bots to produce enough of the resource
                        // costs[idx] - inventory[idx] = how much more of the resource we need
                        // / bots[idx] = how much we make per minute
                        // + bots[idx] - 1 ensures ceiling division, so we always round up
                        _ => (costs[idx] - inventory[idx] + bots[idx] - 1) / bots[idx],
                    }
                })
                .max()
                .unwrap();

            // if that choice would cause the time limit be to exceeded, skip
            // the + 1 is so the built bot has the chance to do something, it merely being built is not enough
            let new_elapsed = elapsed + wait_time + 1;
            if new_elapsed >= max_time {
                continue;
            }

            // gather ores with previously available bots
            let mut new_inventory = [0; 4];
            for idx in 0..bots.len() {
                new_inventory[idx] = inventory[idx] + bots[idx] * (wait_time + 1) - costs[idx];
            }

            // increase bot type for the bot we just built
            let mut new_bots = bots;
            new_bots[i] += 1;

            // extra optimization:
            // if we theoretically only built geode bots every turn, and we still don't beat the maximum, skip
            let time_left = max_time - new_elapsed;
            if
                // Optimistic number of geodes that could be mined if you built a new geode bot
                // every minute from now on — starting next minute
                // 1 + 2 + 3 + ... + (time_left - 1) = (time_left * (time_left - 1)) / 2
                ((time_left - 1) * time_left) / 2
                    + new_inventory[3]
                    + time_left * new_bots[3]
                    < max_geodes
            {
                continue;
            }

            q.push_back(State {
                inventory: new_inventory,
                bots: new_bots,
                elapsed: new_elapsed,
            })
        }

        let geodes = inventory[3] + bots[3] * (max_time - elapsed);
        max_geodes = geodes.max(max_geodes);
    }

    max_geodes
}

pub fn part_1(blueprints: &Vec<[[u16; 4]; 4]>, max_time: u16) -> u16 {
    blueprints
        .iter()
        .map(|blueprint| max_geodes(blueprint, max_time))
        .enumerate()
        .map(|(idx, geodes)| (idx as u16 + 1) * geodes)
        .sum::<u16>()
}

pub fn part_2(blueprints: &Vec<[[u16; 4]; 4]>, max_time: u16) -> usize {
    blueprints
        .iter()
        .take(3)
        .map(|blueprint| usize::from(max_geodes(blueprint, max_time)))
        .product::<usize>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let blueprints: Vec<[[u16; 4]; 4]> = vec![
            [[4, 0, 0, 0], [2, 0, 0, 0], [3, 14, 0, 0], [2, 0, 7, 0]],
            [[2, 0, 0, 0], [3, 0, 0, 0], [3, 8, 0, 0], [3, 0, 12, 0]],
        ];

        let x = max_geodes(&blueprints[0], 24);
        dbg!(x);
        let x = max_geodes(&blueprints[1], 24);
        dbg!(x);
    }

    #[test]
    fn test_part_2() {
        let blueprints: Vec<[[u16; 4]; 4]> = vec![
            [[4, 0, 0, 0], [2, 0, 0, 0], [3, 14, 0, 0], [2, 0, 7, 0]],
            [[2, 0, 0, 0], [3, 0, 0, 0], [3, 8, 0, 0], [3, 0, 12, 0]],
        ];

        let x = max_geodes(&blueprints[0], 32);
        dbg!(x);
        let x = max_geodes(&blueprints[1], 32);
        dbg!(x);
    }
}

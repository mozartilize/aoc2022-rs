use itertools::Itertools;
use std::{
    collections::{BTreeSet, HashMap, VecDeque},
    hash::Hash,
    vec,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Vault {
    name: String,
    flow_rate: u32,
    links: Vec<usize>,
}

impl Vault {
    pub fn new(name: String, flow_rate: u32) -> Self {
        Self {
            name,
            flow_rate,
            links: vec![],
        }
    }

    fn add_links(&mut self, links: Vec<usize>) {
        for v in links {
            self.links.push(v);
        }
    }
}

pub struct Container {
    vaults: Vec<Vault>,
    pub idxes: HashMap<String, usize>,
    rev_idxes: HashMap<usize, String>,
}

impl Container {
    pub fn new() -> Self {
        Self {
            vaults: vec![],
            idxes: HashMap::new(),
            rev_idxes: HashMap::new(),
        }
    }

    pub fn add_vault(&mut self, v: Vault) {
        let vault_name = v.name.clone();
        self.vaults.push(v);
        let idx = self.vaults.len() - 1;
        self.rev_idxes.insert(idx, vault_name.clone());
        self.idxes.insert(vault_name, idx);
    }

    pub fn has_flow_rate_vaults(&self) -> Vec<&usize> {
        let x = self
            .vaults
            .iter()
            .filter(|v| v.flow_rate > 0)
            .map(|v| self.idxes.get(v.name.as_str()).unwrap())
            .collect::<Vec<_>>();
        x
    }

    fn get_vault_by_name(&self, name: &str) -> &Vault {
        let idx = self.idxes[name];
        &self.vaults[idx]
    }

    fn get_mut_vault_by_name(&mut self, name: &str) -> &mut Vault {
        let idx = self.idxes[name];
        &mut self.vaults[idx]
    }

    pub fn link_vaults(&mut self, vault_name: &str, links: Vec<&str>) {
        let link_idxes = links
            .iter()
            .map(|name| self.idxes.get(*name).unwrap().to_owned())
            .collect::<Vec<usize>>();
        let v = self.get_mut_vault_by_name(vault_name);
        v.add_links(link_idxes);
    }

    fn pressure_gain<'a>(
        &'a self,
        min: i32,
        pressure: u32,
        src: &'a usize,
        dst: &'a usize,
        bfs_cache: &mut HashMap<[&'a usize; 2], Vec<&'a usize>>,
    ) -> (i32, u32) {
        let p = bfs(self, src, dst, bfs_cache);
        let min = min - p.len() as i32;
        let v = self.get_vault_by_name(&self.rev_idxes[&dst]);
        // println!("Open vault {} at min {}", v.name, min);
        (
            min,
            if min > 0 {
                v.flow_rate * min as u32 + pressure
            } else {
                0
            },
        )
    }

    pub fn run3<'a>(
        &'a self,
        min: i32,
        pressure: u32,
        path: Vec<&'a usize>,
        pool: Vec<&'a usize>,
        bfs_cache: &mut HashMap<[&'a usize; 2], Vec<&'a usize>>,
    ) -> u32 {
        // the 3rd function that works for part1
        let start = path.last().unwrap();
        let x = pool
            .iter()
            .filter_map(|v| {
                let (min, pressure) = self.pressure_gain(min, pressure, start, v, bfs_cache);
                if min > 0 {
                    let mut new_path = path.clone();
                    new_path.push(*v);
                    let mut new_pool = pool.clone();
                    new_pool.retain(|nv| nv != v);
                    Some((pressure, min, new_path, new_pool))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        let r = x
            .into_iter()
            .map(|r| self.run3(r.1, r.0, r.2, r.3, bfs_cache))
            .max();
        match r {
            Some(p) => p,
            None => pressure,
        }
    }

    pub fn run_part2(&self) {
        let pool = self
            .has_flow_rate_vaults()
            .iter()
            .map(|v| *v)
            .collect::<Vec<_>>();
        let mut db = HashMap::new();
        let mut db2 = HashMap::new();

        for i in 1..=pool.len()/2+1 {
            for sub_pool in pool.clone().into_iter().combinations(i) {
                // Generate all possible paths for both elephant and human.
                // Because both have same productivity so we could reduce the pool to half for them.
                // For example, with 16 vaults, both should open 8, ideally.
                // But it's possible that human open 5 and elephant could open 11.
                let path = vec![&self.idxes["AA"]];
                let x = self.run3(26, 0, path, sub_pool.clone(), &mut db);
                db2.insert(BTreeSet::from_iter(sub_pool.into_iter()), x);
            }
        }

        let x = db2.iter()
            .tuple_combinations()
            // filtering out pair of path that dont share any common vault
            .filter(|(human, elephant)| human.0.is_disjoint(elephant.0))
            .map(|(human, elephant)| human.1 + elephant.1)
            .max()
            .unwrap();
        dbg!(x);
    }
}

fn bfs<'a>(
    adj: &'a Container,
    src: &'a usize,
    dst: &'a usize,
    bfs_cache: &mut HashMap<[&'a usize; 2], Vec<&'a usize>>,
) -> Vec<&'a usize> {
    if let Some(p) = bfs_cache.get(&[src, dst]) {
        // println!("cached");
        return p.to_vec();
    }

    let mut q: VecDeque<Vec<&usize>> = VecDeque::new();

    let mut path: Vec<&usize> = vec![];
    path.push(src);
    q.push_back(path);

    while q.len() > 0 {
        let path = q.pop_front().unwrap();
        let curr = path.last().unwrap();

        if *curr == dst {
            bfs_cache.insert([src, dst], path.clone());
            return bfs_cache.get(&[src, dst]).unwrap().to_vec();
        }

        let vault_name = &adj.rev_idxes[curr];
        let v = adj.get_vault_by_name(vault_name);
        for x in &v.links {
            if !path.contains(&x) {
                let mut new_path = path.clone();
                new_path.push(x);
                q.push_back(new_path);
            }
        }
    }
    bfs_cache.insert([src, dst], vec![]);
    bfs_cache.get(&[src, dst]).unwrap().to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::Itertools;

    #[test]
    fn test_builder() {
        let mut c = Container::new();
        let aa_v = Vault::new("AA".to_string(), 0);
        let bb_v = Vault::new("BB".to_string(), 13);
        let cc_v = Vault::new("CC".to_string(), 2);
        let dd_v = Vault::new("DD".to_string(), 20);
        let ee_v = Vault::new("EE".to_string(), 3);
        let ff_v = Vault::new("FF".to_string(), 0);
        let gg_v = Vault::new("GG".to_string(), 0);
        let hh_v = Vault::new("HH".to_string(), 22);
        let ii_v = Vault::new("II".to_string(), 0);
        let jj_v = Vault::new("JJ".to_string(), 21);

        c.add_vault(aa_v);
        c.add_vault(bb_v);
        c.add_vault(cc_v);
        c.add_vault(dd_v);
        c.add_vault(ee_v);
        c.add_vault(ff_v);
        c.add_vault(gg_v);
        c.add_vault(hh_v);
        c.add_vault(ii_v);
        c.add_vault(jj_v);

        c.link_vaults("AA", vec!["BB", "DD", "II"]);
        c.link_vaults("BB", vec!["CC", "AA"]);
        c.link_vaults("CC", vec!["DD", "BB"]);
        c.link_vaults("DD", vec!["CC", "AA", "EE"]);
        c.link_vaults("EE", vec!["FF", "DD"]);
        c.link_vaults("FF", vec!["EE", "GG"]);
        c.link_vaults("GG", vec!["FF", "HH"]);
        c.link_vaults("HH", vec!["GG"]);
        c.link_vaults("II", vec!["AA", "JJ"]);
        c.link_vaults("JJ", vec!["II"]);

        let pool = c
            .has_flow_rate_vaults()
            .iter()
            .map(|v| *v)
            .collect::<Vec<_>>();
        // .iter()
        // .map(|v| *v)
        // .collect::<Vec<_>>();
        let path = vec![&c.idxes["AA"]];
        let mut db = HashMap::new();
        let x = c.run3(30, 0, path, pool, &mut db);
        // dbg!(db);
        dbg!(x);

        let mut count = 0;
        for i in 1..=c.has_flow_rate_vaults().len() {
            let x = c.has_flow_rate_vaults().iter().combinations(i).count();
            dbg!(x);
            count += x;
        }
        dbg!(count);
    }
}

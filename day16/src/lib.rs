use itertools::Itertools;
use ouroboros::self_referencing;
use std::{
    collections::{BTreeSet, HashMap, VecDeque}, hash::Hash, str::FromStr, vec
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

#[self_referencing]
pub struct Container {
    vaults: Vec<Vault>,

    #[borrows(vaults)]
    #[covariant]
    pub idxes: HashMap<&'this str, usize>,
    #[borrows(vaults)]
    #[covariant]
    rev_idxes: HashMap<usize, &'this str>,
}

impl Container {
    pub fn build(mut vaults: Vec<Vault>, links: Vec<Vec<&str>>) -> Self {
        let idxes = vaults.iter().enumerate().map(|(idx, v)| { (v.name.clone(), idx) }).collect::<HashMap<_,_>>();
        for (idx, _links) in links.iter().enumerate() {
            // let key = String::from_str(v);
            vaults[idx].add_links(_links.iter().map(|v| idxes[&String::from_str(v).unwrap()] ).collect::<Vec<_>>());
        }
        ContainerBuilder{
            vaults,
            idxes_builder: |vaults| vaults.iter().enumerate().map(|(idx, v)| { (v.name.as_str(), idx) }).collect::<HashMap<_,_>>(),
            rev_idxes_builder: |vaults| vaults.iter().enumerate().map(|(idx, v)| { (idx, v.name.as_str()) }).collect::<HashMap<_,_>>(),
        }.build()
    }
    pub fn has_flow_rate_vaults(&self) -> Vec<&usize> {
        let x = self
            .borrow_vaults()
            .iter()
            .filter(|v| v.flow_rate > 0)
            .map(|v| self.borrow_idxes().get(v.name.as_str()).unwrap())
            .collect::<Vec<_>>();
        x
    }

    fn get_vault_by_name(&self, name: &str) -> &Vault {
        let idx = self.borrow_idxes()[name];
        &self.borrow_vaults()[idx]
    }

    fn get_vault_by_idx(&self, idx: usize) -> &Vault {
        &self.borrow_vaults()[idx]
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
        let v = self.get_vault_by_idx(*dst);
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
                let path = vec![&self.borrow_idxes()["AA"]];
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

        let vault_name = &adj.borrow_rev_idxes()[curr];
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

        let vaults = vec![
            aa_v,
            bb_v,
            cc_v,
            dd_v,
            ee_v,
            ff_v,
            gg_v,
            hh_v,
            ii_v,
            jj_v,
        ];

        let links = vec![
            vec!["BB", "DD", "II"],
            vec!["CC", "AA"],
            vec!["DD", "BB"],
            vec!["CC", "AA", "EE"],
            vec!["FF", "DD"],
            vec!["EE", "GG"],
            vec!["FF", "HH"],
            vec!["GG"],
            vec!["AA", "JJ"],
            vec!["II"],
        ];
        
        let c = Container::build(vaults, links);
        let pool = c
            .has_flow_rate_vaults()
            .iter()
            .map(|v| *v)
            .collect::<Vec<_>>();
        // .iter()
        // .map(|v| *v)
        // .collect::<Vec<_>>();
        let path = vec![&c.borrow_idxes()["AA"]];
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

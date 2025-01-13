use std::{
    collections::{HashMap, VecDeque},
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
    pub fn has_flow_rate_vaults(&self) -> Vec<&usize> {
        let x = self
            .vaults
            .iter()
            .filter(|v| v.flow_rate > 0)
            .map(|v| self.idxes.get(&v.name).unwrap())
            .collect::<Vec<_>>();
        x
    }

    pub fn new() -> Self {
        Self {
            vaults: vec![],
            idxes: HashMap::new(),
            rev_idxes: HashMap::new(),
        }
    }

    fn get_vault_by_name(&self, name: &str) -> &Vault {
        let idx = self.idxes[name];
        &self.vaults[idx]
        // let v = self.vaults.iter().find(|v| &v.name == name).unwrap();
        // v
    }

    fn get_mut_vault_by_name(&mut self, name: &str) -> &mut Vault {
        let idx = self.idxes[name];
        &mut self.vaults[idx]
    }

    pub fn add_vault(&mut self, v: Vault) {
        let vault_name = String::from(&v.name);
        self.vaults.push(v);
        self.idxes
            .insert(String::from(&vault_name), self.vaults.len() - 1);
        self.rev_idxes.insert(self.vaults.len() - 1, vault_name);
    }

    pub fn link_vaults(&mut self, vault_name: &str, links: Vec<&str>) {
        let link_idxes = links
            .iter()
            .map(|name| self.idxes.get(*name).unwrap().to_owned())
            .collect::<Vec<usize>>();
        let v = self.get_mut_vault_by_name(vault_name);
        v.add_links(link_idxes);
    }

    pub fn run2(
        &self,
        start: usize,
        order: &[&usize],
        cache: &mut HashMap<Vec<usize>, Vec<usize>>,
        cache2: &mut HashMap<(usize, Vec<usize>), (i32, u32)>,
    ) -> (i32, u32) {
        if let Some((min, pressure)) =
            cache2.get(&(start, order.iter().map(|x| **x).collect::<Vec<_>>()))
        {
            // println!("cached2");
            return (*min, *pressure);
        }
        let head = &order[0..order.len() - 1];
        let tail = &order[order.len() - 1..];
        let mut min;
        let mut pressure;
        let mut _start = start;
        if head.len() == 0 {
            min = 30;
            pressure = 0;
        } else {
            (min, pressure) = self.run2(start, head, cache, cache2);
            _start = **head.last().unwrap();
        }
        if min < 0 {
            println!("{}, {:?}", min, head);
            // min += min + p.len() as i32;
        } else {
            let p = bfs(&self, _start, *tail[0], cache);
            min -= p.len() as i32;
            let v = self.get_vault_by_name(&self.rev_idxes[tail[0]]);
            // println!("Open vault {} at min {}", v.name, min);
            pressure += v.flow_rate * min as u32;
        }
        cache2.insert(
            (start, order.iter().map(|x| **x).collect::<Vec<_>>()),
            (min, pressure),
        );
        return (min, pressure);
    }

    fn pressure_gain(&self, min: i32, pressure: u32, src: usize, dst: usize) -> (i32, u32) {
        let p = bfs(&self, src, dst, &mut HashMap::new());
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

    pub fn run(
        &self,
        start: usize,
        order: Vec<&usize>,
        cache: &mut HashMap<Vec<usize>, Vec<usize>>,
    ) -> u32 {
        let mut min: i32 = 30;
        let mut _start = start;
        let mut pressure = 0;
        for vidx in order {
            let p = bfs(&self, _start, *vidx, cache);
            min -= p.len() as i32;
            if min < 0 {
                break;
            }
            let v = self.get_vault_by_name(&self.rev_idxes[vidx]);
            // println!("Open vault {} at min {}", v.name, min);
            pressure += v.flow_rate * min as u32;
            _start = *vidx;
        }
        // dbg!(pressure);
        pressure
    }

    pub fn run3<'a>(
        &self,
        min: i32,
        pressure: u32,
        path: Vec<&'a usize>,
        pool: Vec<&'a usize>,
        db: &mut HashMap<Vec<&'a usize>, (i32, u32)>,
    ) {
        let start = path.last().unwrap();
        let x = pool
            .iter()
            .filter(|v| {
                let (min, pressure) = self.pressure_gain(min, pressure, **start, ***v);
                if min > 0 {
                    let mut new_path = path.clone();
                    new_path.push(v);
                    db.insert(new_path, (min, pressure));
                }
                min > 0
            })
            .map(|v| *v)
            .collect::<Vec<_>>();
        for v in x.iter() {
            let mut new_pool = x.clone();
            new_pool.retain(|nv| nv != v);
            let mut new_path = path.clone();
            new_path.push(v);
            let (min, pressure) = db.get(&new_path).unwrap();
            self.run3(*min, *pressure, new_path, new_pool, db);
        }
    }
}

fn bfs(
    adj: &Container,
    src: usize,
    dst: usize,
    cache: &mut HashMap<Vec<usize>, Vec<usize>>,
) -> Vec<usize> {
    if let Some(p) = cache.get(&vec![src, dst]) {
        // println!("cached");
        return (*p).clone();
    }
    let mut q: VecDeque<Vec<usize>> = VecDeque::new();

    let mut path = vec![];
    path.push(src);
    q.push_back(path);

    while q.len() > 0 {
        let path = q.pop_front().unwrap();
        let curr = path.last().unwrap();

        if curr == &dst {
            cache.insert(vec![src, dst], path.clone());
            return path;
        }

        let vault_name = String::from(&adj.rev_idxes[&curr]);
        let v = adj.get_vault_by_name(&vault_name);
        for x in &v.links {
            if !path.contains(x) {
                let mut new_path = path.clone();
                new_path.push(*x);
                q.push_back(new_path);
            }
        }
    }
    cache.insert(vec![src, dst], vec![]);
    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;
    // use itertools::Itertools;

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

        // dbg!(&c.idxes);
        // let mut cache = HashMap::new();
        // dbg!(bfs(&c, 2, 6, &mut cache));

        // let mut cache2 = HashMap::new();
        // let mut max_pressure = 0;
        // for order in c
        // .has_flow_rate_vaults()
        // .iter()
        // .permutations(c.has_flow_rate_vaults().len())
        // {
        //     let (min, pressure) = c.run2(0, &order, &mut cache, &mut cache2);
        //     if min > 0 && pressure > max_pressure {
        //         max_pressure = pressure;
        //     }
        // }

        // dbg!(max_pressure);

        // dbg!(c.run2(0, &[&3, &1, &9, &7, &4, &2], &mut cache, &mut cache2));
        // let max_pressure = c
        //     .has_flow_rate_vaults()
        //     .iter()
        //     .permutations(c.has_flow_rate_vaults().len())
        //     .map(|order| {
        //         c.run(0, order, &mut cache)
        //     })
        //     .max();

        // dbg!(max_pressure);

        let pool = c
            .has_flow_rate_vaults()
            .into_iter()
            .map(|v| v)
            .collect::<Vec<_>>();
        let path = vec![&c.idxes["AA"]];
        let mut db = HashMap::new();
        c.run3(30, 0, path, pool, &mut db);
        // dbg!(db);
        dbg!(db.values().map(|v| { v.1 }).max());
    }
}

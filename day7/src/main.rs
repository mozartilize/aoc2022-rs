use std::collections::HashMap;
use std::io;
use std::rc::Rc;

fn main() {
    let lines = io::stdin().lines();
    let mut curr_dir: Option<Rc<String>> = None;
    let mut dir_size: HashMap<Rc<String>, usize> = HashMap::new();
    for line in lines.map(|l| l.unwrap()) {
        if line.starts_with("$ cd") {
            let mut split = line.split("$ cd ");
            split.next();
            let dir_name = split.next().unwrap();
            let to_path_str: Rc<String>;
            if dir_name == ".." {
                let curr_dir_path_str = &curr_dir.unwrap();
                let i = curr_dir_path_str.rfind("/").unwrap();
                to_path_str = Rc::new(curr_dir_path_str[0..i].to_string());
            } else if dir_name == "/" {
                to_path_str = Rc::new("/".to_string());
                dir_size.insert(Rc::clone(&to_path_str), 0);
            } else {
                to_path_str = Rc::new({
                    let parent_path = curr_dir.clone().unwrap().to_string();
                    if parent_path.ends_with("/") {
                        parent_path + dir_name
                    } else {
                        parent_path + "/" + dir_name
                    }
                });
            }
            curr_dir = Some(to_path_str);
        } else if line.starts_with("$ ls") {
        } else if line.starts_with("dir") {
            let mut split = line.split("dir ");
            split.next();
            let dir_name = split.next().unwrap();
            let to_path_str = Rc::new({
                let parent_path = curr_dir.clone().unwrap().to_string();
                if parent_path.ends_with("/") {
                    parent_path + dir_name
                } else {
                    parent_path + "/" + dir_name
                }
            });
            dir_size.insert(Rc::clone(&to_path_str), 0);
        } else {
            let mut split = line.split(" ");
            let size = str::parse::<usize>(split.next().unwrap()).unwrap();
            let parent_path = curr_dir.clone().unwrap();
            add_size_to_path(&mut dir_size, &parent_path, size);
        }
    }
    let required = 30000000 - (70000000 - dir_size[&Rc::new("/".to_string())]);
    dbg!(dir_size.iter().filter(|e| *e.1 <= 100000).map(|e| *e.1).sum::<usize>());
    dbg!(dir_size.iter().filter(|e| *e.1 >= required).map(|e| *e.1).min());
}

fn add_size_to_path(dir_size: &mut HashMap<Rc<String>, usize>, dir_path: &str, size: usize) {
    if dir_path == "" {
        dir_size.entry(Rc::new("/".to_string())).and_modify(|v| *v += size);
        return;
    } else {
        dir_size.entry(Rc::new(dir_path.to_string())).and_modify(|v| *v += size);
    };
    let idx = dir_path.rfind("/").unwrap();
    add_size_to_path(dir_size, &dir_path[0..idx], size);
}

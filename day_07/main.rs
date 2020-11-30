use std::{io, env};
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::{HashMap, HashSet};
use core::fmt;
use std::process::exit;

#[derive(Debug)]
struct Program {
    name: String,
    weight: Option<u32>,
    parent: Option<String>,
    children: Vec<String>,
    total_weight: u32,
}

impl Program {
    fn new(name: &str, weight: Option<u32>, parent: Option<String>) -> Program {
        println!("  creating {} parent is {:?}", name, parent);
        Program{
            name: name.to_string(),
            weight,
            parent,
            children:Vec::new(),
            total_weight: 0,
        }
    }

    // fn total_weight(&self) -> u32 {
    //     // it's a bug to be None but let's roll
    //     self.weight.unwrap_or(0) + self.total_weight
    // }
}

struct Tower {
    // the bottom of the tower
    root: Option<String>,
    // all the known programs
    programs: HashMap<String, Program>,
}

impl fmt::Debug for Tower {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.root {
            Some(root) => write!(f, "Tower(r={} p={:?})", root, self.programs.keys()),
            None => write!(f, "Tower(r=None p=None)"),
        }
    }
}

impl Tower {
    fn new() -> Tower {
        Tower{
            root: None,
            programs: HashMap::new(),
        }
    }

    fn root_name(&self) -> Option<&str> {
        match &self.root {
            Some(root) => Some(root),
            None => None,
        }
    }

    fn from_file(path: &str) -> io::Result<Tower> {
        let file = File::open(path)?;
        let br = BufReader::new(file);
        let mut rv = Tower::new();

        for (idx, line) in br.lines().enumerate() {
            let line = line?;
            if let Some(data) = Self::data_from_line(&line) {
                rv.add(&data.0, data.1, &data.2);
            } else {
                // empty line, I guess it's fine
                eprintln!("No data from line {}", idx);
            }
        }
        eprintln!("Compute children weight");
        if rv.root.is_some() {
            let root = rv.root.clone().unwrap();
            if let Some(updates) = rv.visit_from(&root) {
                // consumes elements
                for (name, children_weight) in updates {
                    rv.programs.get_mut(&name).unwrap().total_weight = children_weight;
                    eprintln!("{} => {}", name, children_weight);
                }
            }
        }
        Ok(rv)
    }

    fn find_root_from(&self, name: &str) -> Option<String> {
        let base = self.programs.get(name)?;
        match &base.parent {
            Some(parent) => {
                println!("  {} <-", name);
                self.find_root_from(&parent)
            },
            None => Some(base.name.clone())
        }
    }

    // TODO(tr) Recursive and mutable is apparently not working well...
    // so too bad for the children.
    /*fn visit_from(&mut self, name: &str) -> Option<u32> {
        let current = self.programs.get_mut(name)?;
        let mut sum = 0;
        println!("visit_from({}) -> {:?}", name, current.children);
        for child in &current.children {
            if let Some(value) = self.visit_from(child) {
                sum += value;
            }
        }
        current.children_weight = sum;
        Some(sum)
    }*/
    fn visit_from(&self, name: &str) -> Option<HashMap<String, u32>> {
        let mut rv = HashMap::new();
        let current = self.programs.get(name)?;
        let mut sum = current.weight.unwrap_or(0);
        println!("visit_from({}) -> {:?}", name, current.children);
        for child in &current.children {
            if let Some(value) = self.visit_from(child) {
                sum += value[child];
                rv.extend(value);
            }
        }
        rv.insert(name.to_string(), sum);
        Some(rv)
    }

    fn find_unstable_children(&self, name: &str) -> Option<(String, HashMap<String, u32>)> {
        let current = self.programs.get(name)?;
        let mut values : HashSet<u32> = HashSet::new();
        let mut rv = HashMap::new();
        for child in &current.children {
            if let Some(child_rv) = self.find_unstable_children(child) {
                eprintln!("{}->{} is unstable", name, child);
                return Some(child_rv);
            }
            let v = self.programs.get(child).unwrap().total_weight;
            rv.insert(child.clone(), v);
            values.insert(v);
        }

        if values.len() > 1 {
            eprintln!("{} is not stable...", name);
            Some((name.to_string(), rv))
        } else {
            eprintln!("{} is stable", name);
            None
        }
    }

    fn search_unstable(&self) -> Option<(String, HashMap<String, u32>)> {
        // TODO(tr) We could use the result to know which one of the children is bad
        //  and compute bad_child.weight -= diff to return the value bad_child should
        //  have for the tower to be balanced.
        //  At the moment I computed that by hand searching for the bad entry in the
        //  input which is not great...
        self.find_unstable_children(self.root_name().unwrap_or(&""))
    }

    fn add(&mut self, name: &str, weight: u32, children: &[String]) {
        // let mut parent = self.programs.entry(name.to_string())
        //     .or_insert(Program::new(name));

        println!("Adding {} ({}) {:?}",
                 name,
                 weight,
                 children,
        );
        // let parent = self.programs.get_mut(name).unwrap();
        // parent.weight = Some(weight);
        if let Some(parent) = self.programs.get_mut(name) {
            // println!("  {} is known", name);
            parent.weight = Some(weight);
        } else {
            let entry = Program::new(name, Some(weight), None);
            // println!("  {} is new", name);
            // entry.weight = Some(weight);
            self.programs.insert(name.to_string(), entry);
        }

        // let parent = self.programs.get_mut(name).unwrap();
        for child in children {
            // if self.root.is_some() && self.root.as_ref().unwrap() == child {
            //     // change the root to the current entry!
            //     println!("find root from {}", name);
            //     let new_root = self.find_root_from(name).unwrap();
            //     if new_root == name {
            //         println!("  new root is {}", name);
            //     } else {
            //         println!("  new root is {} (root from {})", new_root, name);
            //     }
            //     self.root = Some(new_root);
            // }

            // self.programs.entry(child.to_string())
            //     .or_insert(
            //         Program::new(child, None, Some(name.to_string()))
            //     );
            match self.programs.get_mut(child) {
                Some(child) => {
                    println!("  updating {} parent -> {}", &child.name, name);
                    child.parent = Some(name.to_string());
                },
                None => {
                    self.programs.insert(
                        child.to_string(),
                        Program::new(child, None, Some(name.to_string())),
                    );
                }
            }

            // parent.children.push(child.to_string());
            // sad that we have to get it again every time.
            self.programs.get_mut(name).unwrap().children.push(child.to_string());
            // self.programs[name].children.push(child.clone()); // [] is not mutable...
        }

        println!("Checking if {:?} is still the root...", self.root);
        if self.root.is_none() {
            println!("  no root, new root is {}", name);
            self.root = Some(name.to_string());
        } else if children.contains(&self.root.as_ref().unwrap()) {
            println!("find root from {}", name);
            let new_root = self.find_root_from(name).unwrap();
            if new_root == name {
                println!("  new root is {}", name);
            } else {
                println!("  new root is {} (root from {})", new_root, name);
            }
            self.root = Some(new_root);
        }
    }

    // returns a tuple to simplify testing
    fn data_from_line(line: &str) -> Option<(String, u32, Vec<String>)> {
        // match lines like "{prgm} ({weight}) -> {prgm}"
        // match lines like "{prgm} ({weight}) -> {prgm},{prgm}"
        // or lines like "{prgm} ({weight})"

        let actions: Vec<&str> = line.split_whitespace().collect();
        // why is that a &&str??? or is pycharm confusing me...
        let name = actions.get(0).expect("no name");
        let weight:u32 = actions.get(1).expect("no weight_str")
            .replace("(", "")
            .replace(")", "")
            .parse().expect("invalid weight");
        // now check if we have children
        let mut children = Vec::new();
        let actions: Vec<&str> = line.split(" -> ").collect();

        if let Some(children_string) = actions.get(1) {
            for child_name in children_string.replace(" ", "").split(",") {
                children.push(child_name.to_string());
            }
        }

        Some((name.to_string(), weight, children))
    }

}

fn main() {
    let path = env::args().nth(1).expect("please supply a path");
    let tower = match Tower::from_file(&path) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("failed: {}", e);
            exit(1);
        },
    };

    println!("Root of the tower is {:?}", tower.root_name());

    if let Some(unbalanced) = tower.search_unstable() {
        println!("Unstable: {:?}", unbalanced);
    } else {
        println!("Tower is stable");
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    fn some_data(name: &str, weight: u32, children: Vec<&'static str>) -> Option<(String, u32, Vec<String>)> {
        Some((
            name.to_string(),
            weight,
            children.iter().map(|e| e.to_string()).collect(),
        ))
    }

    #[rstest(line, exp_data,
    case(&"pbga (66)", some_data("pbga", 66, Vec::new())),
    case(&"fwft (72) -> ktlj", some_data("fwft", 72, vec!["ktlj"])),
    case(&"fwft (72) -> ktlj, cntj, xhth", some_data("fwft", 72, vec!["ktlj", "cntj", "xhth"])),
    )]
    fn test_data_from_line(line: &str, exp_data: Option<(String, u32, Vec<String>)>) {
        assert_eq!(Tower::data_from_line(line), exp_data);
    }

    #[rstest(path, root,
    case(&"day_07/test.txt", &"tknk"),
    case(&"day_07/test_2.txt", &"root"),
    )]
    fn test_load_file(path: &str, root: &str) {
        let tower = Tower::from_file(path).unwrap();
        println!("{:?}", tower);
        assert!(tower.root.is_some());
        assert_eq!(tower.root.unwrap(), root);
    }

    #[rstest(name, total_weight, unbalanced,
    case(&"gyxo", 61, None),
    case(&"ugml", 251, None),
    case(&"tknk", 41 + 251 + 243 + 243, Some("tknk".to_string())),
    case(&"root", 20 + 41 + 251 + 243 + 243, Some("tknk".to_string())),
    )]
    fn test_find_balanced(name: &str, total_weight: u32, unbalanced: Option<String>) {
        let tower = Tower::from_file(&"day_07/test_3.txt").unwrap();
        let program = tower.programs.get(name).unwrap();
        println!("Program: {:?}", program);
        assert_eq!(program.total_weight, total_weight);
        let rv = tower.find_unstable_children(name);
        if unbalanced.is_some() {
            assert!(rv.is_some());
            assert_eq!(rv.unwrap().0, unbalanced.unwrap());
        } else {
            assert!(rv.is_none());
        }
    }
}

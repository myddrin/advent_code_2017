use std::{io, env};
use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

fn read(path: &str) -> io::Result<Vec<u32>> {
    let mut file = File::open(path)?;
    let mut content = String::new();
    let mut rv = Vec::new();
    file.read_to_string(&mut content)?;

    for value in content.replace("\t", " ").split_whitespace() {
        rv.push(value.parse().unwrap());
    }
    Ok(rv)
}

fn redistribution_cycle(bank: &mut Vec<u32>) {
    let max_value = bank.iter().max().unwrap_or(&0);
    let mut p = bank.iter().position(|&x| x == *max_value).unwrap();

    let mut entries = bank[p];  // copy
    bank[p] = 0;

    while entries > 0 {
        p = (p + 1) % bank.len();
        bank[p] += 1;
        entries -= 1;
    }
}

fn search_loop(bank: &[u32]) -> (usize, usize) {
    let mut i = 0;
    let mut data = bank.to_vec();
    // let mut history: HashSet<String> = HashSet::new(); // q1 was solved with hashset
    let mut history: HashMap<String, usize> = HashMap::new();

    let mut current = format!("{:?}", data);

    while !history.contains_key(&current) {
        history.insert(current, i);

        redistribution_cycle(&mut data);
        current = format!("{:?}", data);
        i += 1;
    }

    (i, i - history[&current])
}

fn main() {
    let path = env::args().nth(1).expect("please supply a path");
    let contents = read(&path).expect("no content");

    println!("Initial state: {:?}", contents);
    let cycles = search_loop(&contents);
    println!("Looped in {} cycles (loop size is {})", cycles.0, cycles.1);
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[rstest(bank, output,
    case(&[0, 2, 7, 0], &[2, 4, 1, 2]),
    case(&[2, 4, 1, 2], &[3, 1, 2, 3]),
    case(&[0, 2, 3, 4], &[1, 3, 4, 1]),
    case(&[1, 3, 4, 1], &[2, 4, 1, 2]),
    )]
    fn test_redistribution_cycle(bank: &[u32], output: &[u32]) {
        let mut data = bank.to_vec();
        redistribution_cycle(&mut data);
        assert_eq!(data, output);
    }

    #[rstest(bank, expected,
    case(&[0, 2, 7, 0], (5, 4)),
    )]
    fn test_search_loop(bank: &[u32], expected: (usize, usize)) {
        let rv = search_loop(bank);
        assert_eq!(rv, expected);
    }
}

use std::io::{BufReader, BufRead};
use std::fs::File;
use std::{env, io};

fn read(path: &str) -> Result<Vec<Vec<u32>>, io::Error> {
    let file = File::open(path)?;
    let br = BufReader::new(file);
    let mut rv = Vec::new();

    for line in br.lines() {
        let line = line?;
        let mut content = Vec::new();
        for e in line.split_whitespace() {
            content.push(e.parse::<u32>().unwrap());
        }

        rv.push(content);
    }
    Ok(rv)
}


fn check_sum(lines: &[Vec<u32>]) -> u32 {
    let mut sum = 0;
    for l in lines {
        let biggest = l.iter().max().unwrap();
        let smallest = l.iter().min().unwrap();
        sum += biggest - smallest;
    }
    sum
}

fn find_best(line: &[u32]) -> Option<(u32, u32)> {
    for (idx, v0) in line.iter().enumerate() {
        for (_idx2, v1) in line.iter().enumerate().filter(|&(i, _)| i > idx) {
            let d = if v1 > v0 {
                (*v1, *v0)
            } else {
                (*v0, *v1)
            };
            if d.0 % d.1 == 0 {
                return Some(d);
            }
        }
    }
    return None;
}

fn compute(lines: &[Vec<u32>]) -> u32 {
    let mut sum = 0;
    for l in lines {
        let best = find_best(l).unwrap();
        sum += best.0 / best.1;
    }
    sum
}


fn main() {
    let path = env::args().nth(1).expect("please supply a path");
    let contents = read(&path).unwrap();

    let checksum = check_sum(&contents);
    println!("Checksum: {}", checksum);
    let sum = compute(&contents);
    println!("Sum: {}", sum);
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest(input, expected,
    case(&[vec!(5, 1, 9, 5)], 8),
    case(&[vec!(7, 5, 3)], 4),
    case(&[vec!(2, 4, 6, 8)], 6),
    case(&[vec!(5, 1, 9, 5), vec!(7, 5, 3), vec!(2, 4, 6, 8)], 18)
    )]
    fn test_check_sum(input: &[Vec<u32>], expected: u32) {
        assert_eq!(check_sum(input), expected);
    }

    #[rstest(input, expected,
    case(&[5, 9, 2, 8], (8, 2)),
    case(&[9, 4, 7, 3], (9, 3)),
    case(&[3, 8, 6, 5], (6, 3)),
    )]
    fn test_find_best(input: &[u32], expected: (u32, u32)) {
        assert_eq!(find_best(input).unwrap(), expected);
    }

    #[rstest(input, expected,
    case(&[vec!(5, 9, 2, 8)], 4),
    case(&[vec!(9, 4, 7, 3)], 3),
    case(&[vec!(3, 8, 6, 5)], 2),
    case(&[vec!(5, 9, 2, 8), vec!(9, 4, 7, 3), vec!(3, 8, 6, 5)], 9)
    )]
    fn test_compute(input: &[Vec<u32>], expected: u32) {
        assert_eq!(compute(input), expected);
    }
}

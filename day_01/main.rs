use std::{fs, env};

fn sum_similar_to_next(list: &[u32]) -> u32 {
    let mut sum = 0;

    for i in list.iter().enumerate() {
        let same = if i.0 + 1 >= list.len() {
            *i.1 == list[0]
        } else {
            *i.1 == list[i.0 + 1]
        };
        if same {
            sum += i.1;
        }
    }

    sum
}

fn sum_similar_to_halfway(list: &[u32]) -> u32 {
    let halfway = list.len() / 2;
    let mut sum = 0;

    for i in list.iter().enumerate() {
        let pos = (i.0 + halfway) % list.len();
        if *i.1 == list[pos] {
            sum += i.1;
        }
    }

    sum
}


fn read(path: &str) -> Vec<u32> {
    let mut rv = Vec::new();
    let contents = fs::read_to_string(path).expect("Cannot load file");
    for c in contents.trim().chars().map(|c| c.to_digit(10).unwrap()) {
        rv.push(c);
    }
    rv
}


fn main() {
    let path = env::args().nth(1).expect("please supply a path");
    let contents = read(&path);
    let sum = sum_similar_to_next(&contents);
    println!("Found similar to next: {}", sum);
    let sum = sum_similar_to_halfway(&contents);
    println!("Found halfway: {}", sum);
}


#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest(input, expected,
    case(&[1, 1, 2, 2], 3),
    case(&[1, 1, 1, 1], 4),
    case(&[1, 2, 3, 4], 0),
    case(&[9, 1, 2, 1, 2, 1, 2, 9], 9),
    )]
    fn test_count_next(input: &[u32], expected: u32) {
        assert_eq!(sum_similar_to_next(input), expected);
    }

    #[rstest(input, expected,
    case(&[1, 2, 1, 2], 6),
    case(&[1, 2, 2, 1], 0),
    case(&[1, 2, 3, 4, 2, 5], 4),
    case(&[1, 2, 3, 1, 2, 3], 12),
    case(&[1, 2, 1, 3, 1, 4, 1, 5], 4),
    )]
    fn test_count_halfway(input: &[u32], expected: u32) {
        assert_eq!(sum_similar_to_halfway(input), expected);
    }
}

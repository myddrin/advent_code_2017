use std::{io, env};
use std::fs::File;
use std::io::{BufReader, BufRead};

fn read(path: &str) -> io::Result<Vec<i32>> {
    let file = File::open(path)?;
    let br = BufReader::new(file);
    let mut rv = Vec::new();

    for line in br.lines() {
        let line = line?;
        rv.push(line.parse().unwrap());
    }
    Ok(rv)
}

fn iteration(contents: &mut Vec<i32>, current: i32, ge: Option<i32>) -> Option<i32> {
    if current < 0 {
        return None;
    }
    let idx = current as usize;
    let value = contents.get(idx)?;
    let next = current + value;  // to get the value before we change it.

    let dec = if let Some(ge_v) = ge {
        *value >= ge_v
    } else {
        false
    };
    if dec {
        contents[idx] -= 1;
    } else {
        contents[idx] += 1;
    }

    Some(next)
}

fn execute(contents: &[i32], ge: Option<i32>) -> u32 {
    let mut i = 0;
    let mut contents = contents.to_vec();
    let mut current = 0;

    while let Some(res) = iteration(&mut contents, current, ge) {
        i += 1;
        current = res;
    }

    i
}

fn main() {
    let path = env::args().nth(1).expect("please supply a path");
    let contents = read(&path).expect("no content");
    let i = execute(&contents, None);
    println!("Q1: reached the end after {} iterations", i);
    let i = execute(&contents, Some(3));
    println!("Q2: reached the end after {} iterations", i);
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[rstest(input, current, exp_next, new_value,
    case(&[0, 3, 0, 1, -3], 0, Some(0), Some(1)),
    case(&[1, 3, 0, 1, -3], 0, Some(1), Some(2)),
    case(&[2, 3, 0, 1, -3], 1, Some(4), Some(4)),
    case(&[2, 4, 0, 1, -3], 4, Some(1), Some(-2)),
    case(&[2, 4, 0, 1, -2], 1, Some(5), Some(5)),
    case(&[2, 4, 0, 1, -2], 5, None, None),
    )]
    fn test_iteration_q1(input: &[i32], current: i32, exp_next: Option<i32>, new_value: Option<i32>) {
        let mut contents = input.to_vec();
        let rv = iteration(&mut contents, current, None);
        assert_eq!(rv, exp_next);
        if let Some(exp_value) = new_value {
            assert_eq!(contents[current as usize], exp_value);
        }
    }

    #[rstest(input, exp_steps,
    case(&[0, 3, 0, 1, -3], 5),
    )]
    fn test_execute_q1(input: &[i32], exp_steps: u32) {
        assert_eq!(execute(input, None), exp_steps);
    }

    #[rstest(input, exp_steps,
    case(&[0, 3, 0, 1, -3], 10),
    )]
    fn test_execute_q2(input: &[i32], exp_steps: u32) {
        assert_eq!(execute(input, Some(3)), exp_steps);
    }
}

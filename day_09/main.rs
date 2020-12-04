use std::{io, env};
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::iter::FromIterator;


fn read(path: &str) -> io::Result<Vec<String>> {
    let file = File::open(path)?;
    let br = BufReader::new(file);
    let mut rv = Vec::new();

    for line in br.lines() {
        let line = line?;
        rv.push(line);
    }
    Ok(rv)
}

#[derive(Debug)]
struct State {
    in_garbage: bool,
    ignore_next: bool,
    depth: u32,
    groups: Vec<u32>,
    char_in_garbage: Vec<char>,
}

impl State {
    fn new() -> State {
        State{
            in_garbage: false,
            ignore_next: false,
            depth: 0,
            groups: Vec::new(),
            char_in_garbage: Vec::new(),
        }
    }

    fn total_score(&self) -> u32 {
        self.groups.iter().sum()
    }

    fn read_stuff(&mut self, stuff: &str) {
        // eprintln!("init state={:?}", self);
        for c in stuff.chars() {
            if self.ignore_next {
                self.ignore_next = false;
                // eprintln!("Skipping {}", c);
                continue;
            }
            match c {
                '<' => {
                    if self.in_garbage {
                        // self.char_in_garbage += 1;
                        self.char_in_garbage.push(c);
                    }
                    self.in_garbage = true;
                },
                '>' => {
                    self.in_garbage = false;
                },
                '!' => {
                   self.ignore_next = true;
                },
                '{' => {
                    if !self.in_garbage {
                        self.depth += 1;
                        self.groups.push(self.depth);
                    } else {
                        // self.char_in_garbage += 1;
                        self.char_in_garbage.push(c);
                    }
                },
                '}' => {
                    if !self.in_garbage {
                        self.depth -= 1;
                    } else {
                        // self.char_in_garbage += 1;
                        self.char_in_garbage.push(c);
                    }
                },
                _ => {
                    if self.in_garbage {
                        // self.char_in_garbage += 1;
                        self.char_in_garbage.push(c);
                    }
                }
            };
            // eprintln!("'{}' state={:?}", c, self);
        };
    }
}


fn main() {
    let path = env::args().nth(1).expect("please supply a path");
    let contents = read(&path).expect("no content");

    let mut state = State::new();
    for line in contents {
        state.read_stuff(&line);
    }

    println!("Found {} groups with a total score of {}",
             state.groups.len(),
             state.total_score(),
    );
    println!("Found {} char in garbage", state.char_in_garbage.len());
    // The garbage is actually garbage so no need to print it.
    // let s = String::from_iter(&state.char_in_garbage);
    // println!("Garbage: {}", s);
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest(input, exp_groups, exp_score,
    case(&"<>", 0, 0),
    case(&"{{{}}}", 3, 6),
    case(&"{{},{}}", 3, 5),
    case(&"{{{},{},{{}}}}", 6, 16),
    case(&"{<a>,<a>,<a>,<a>}", 1, 1),
    case(&"{{<a>},{<a>},{<a>},{<a>}}", 5, 9),
    case(&"{{<!>},{<!>},{<!>},{<a>}}", 2, 3),
    )]
    fn test_check_score(input: &str, exp_groups: usize, exp_score: u32) {
        let mut state = State::new();
        state.read_stuff(input);
        assert_eq!(state.groups.len(), exp_groups);
        assert_eq!(state.total_score(), exp_score);
    }

    #[rstest(input, exp_count,
    case(&"<>", 0),
    case(&"<random characters>", 17),
    case(&"<{!>}>", 2),
    case(&"<!!>", 0),
    case(&"<!!!>>", 0),
    case(&"<{o\"i!a,<{i<a>", 10),
    )]
    fn test_garbage_count(input: &str, exp_count: usize) {
        let mut state = State::new();
        state.read_stuff(input);
        assert_eq!(state.char_in_garbage.len(), exp_count);
    }
}

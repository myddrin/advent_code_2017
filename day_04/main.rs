use std::{io, env};
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashSet;
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

fn check_duplicates(phrase: &String) -> bool {
    let mut hash = HashSet::new();

    for word in phrase.split_whitespace() {
        if hash.contains(word) {
            return false;
        }
        hash.insert(word);
    }

    true
}

fn check_anagrams(phrase: &String) -> bool {
    let mut hash = HashSet::new();

    for word in phrase.split_whitespace() {
        let mut letters: Vec<char> = word.chars().collect();
        letters.sort_by(|a, b| b.cmp(a));
        let s = String::from_iter(letters);
        if hash.contains(&s) {
            return false;
        }
        hash.insert(s);
    }

    true
}

fn main() {
    let path = env::args().nth(1).expect("please supply a path");
    let contents = read(&path).expect("no content");

    let mut valid = 0;
    for passphrase in &contents {
        if check_duplicates(&passphrase) {
            valid += 1;
        }
    }
    println!("{}/{} passphrase are valid under the old policy", valid, contents.len());

    valid = 0;
    for passphrase in &contents {
        if check_anagrams(&passphrase) {
            valid += 1;
        }
    }
    println!("{}/{} passphrase are valid under the new policy", valid, contents.len());
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest(passphrase,
    case("aa bb cc dd ee".to_string()),
    case("aa bb cc dd aaa".to_string()),
    )]
    fn test_no_duplicates(passphrase: String) {
        assert!(check_duplicates(&passphrase));
    }

    #[rstest(passphrase,
    case("aa bb cc dd aa".to_string()),
    )]
    fn test_has_duplicates(passphrase: String) {
        assert!(!check_duplicates(&passphrase));
    }

    #[rstest(passphrase,
    case("abcde fghij".to_string()),
    case("a ab abc abd abf abj".to_string()),
    case("iiii oiii ooii oooi oooo".to_string()),
    )]
    fn test_no_anagrams(passphrase: String) {
        assert!(check_anagrams(&passphrase));
    }

    #[rstest(passphrase,
    case("abcde xyz ecdab".to_string()),
    case("oiii ioii iioi iiio".to_string()),
    )]
    fn test_has_anagrams(passphrase: String) {
        assert!(!check_anagrams(&passphrase));
    }
}
use regex::Regex;
use std::fs;

use std::io;

fn main() -> Result<(), std::io::Error> {
    let mut words: Vec<&str> = vec![];
    let data = fs::read_to_string("/usr/share/dict/words").expect("Unable to read file");
    for line in data.lines() {
        if line.len() == 5 {
            words.push(line);
        }
    }

    let stdin = io::stdin();
    let input = &mut String::new();

    let mut green = vec!['.'; 5];
    let mut yellow = vec![];
    let mut gray = vec![];

    loop {
        input.clear();
        stdin.read_line(input);
        let subs = parse_input(input);
        for (i, sub) in subs.iter().enumerate() {
            match sub.0 {
                'g' => {
                    green[i] = sub.1;
                }
                'y' => yellow.push(sub.1),
                'r' => gray.push(sub.1),
                _ => {}
            }
        }
        let output = filter(words.clone(), green.clone(), yellow.clone(), gray.clone());
        for line in output {
            println!("{:?}", line);
        }
    }
}

fn parse_input(input: &str) -> Vec<(char, char)> {
    let subs = input
        .as_bytes()
        .chunks(2)
        .map(|s| unsafe { ::std::str::from_utf8_unchecked(s) })
        .map(|s| (s.chars().next().unwrap(), s.chars().next().unwrap()))
        .collect::<Vec<_>>();
    subs
}

fn filter_by_green(words: Vec<&str>, green: Vec<char>) -> Vec<&str> {
    let s: String = green.iter().collect();
    let re = Regex::new(&s).unwrap();

    words.iter().cloned().filter(|x| re.is_match(x)).collect()
}

fn filter_by_yellow_and_gray(words: Vec<&str>, yellow: Vec<char>, gray: Vec<char>) -> Vec<&str> {
    let mut res = vec![];
    'outer: for word in words {
        for y in &yellow {
            if !word.contains(&y.to_string()) {
                continue 'outer;
            }
        }
        for g in &gray {
            if word.contains(&g.to_string()) {
                continue 'outer;
            }
        }
        res.push(word);
    }
    res
}

fn filter(words: Vec<&str>, green: Vec<char>, yellow: Vec<char>, gray: Vec<char>) -> Vec<&str> {
    let f = filter_by_green(words, green);
    filter_by_yellow_and_gray(f, yellow, gray)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_filter_by_green() {
        let words = vec!["audio", "clerk", "bloke"];
        let green = vec!['.', 'l', '.', '.', '.'];
        assert_eq!(super::filter_by_green(words, green), vec!["clerk", "bloke"]);
    }

    #[test]
    fn test_filter_by_yellow_and_gray() {
        let words = vec!["audio", "clerk", "bloke"];
        let yellow = vec!['o', 'e', 'k'];
        let gray = vec!['a', 'u', 'd', 'i', 'c', 'r'];
        assert_eq!(
            super::filter_by_yellow_and_gray(words, yellow, gray),
            vec!["bloke"]
        );
    }
}

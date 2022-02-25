use regex::Regex;
use std::fs;

use std::io;

enum Guess {
    Green,
    Yellow,
    Black,
}

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
    let mut black = vec![];

    loop {
        input.clear();
        stdin.read_line(input)?;
        let subs = parse_input(input);
        for (i, sub) in subs.iter().enumerate() {
            match sub.0 {
                'g' => {
                    green[i] = sub.1;
                }
                'y' => yellow.push((i, sub.1)),
                'b' => black.push(sub.1),
                _ => {}
            }
        }
        let output = filter(words.clone(), green.clone(), yellow.clone(), black.clone());
        for line in output {
            println!("{}", line);
        }
    }
}

fn parse_input(input: &str) -> Vec<(char, char)> {
    let subs = input
        .replace('\n', "")
        .as_bytes()
        .chunks(2)
        .map(|s| unsafe { ::std::str::from_utf8_unchecked(s) })
        .map(|s| {
            let mut chars = s.chars();
            let mark = chars.next().unwrap();
            let c = chars.next().unwrap();
            (mark, c)
        })
        .collect::<Vec<_>>();
    subs
}

fn filter_by_green(words: Vec<&str>, green: Vec<char>) -> Vec<&str> {
    let s: String = green.iter().collect();
    let re = Regex::new(&s).unwrap();

    words.iter().cloned().filter(|x| re.is_match(x)).collect()
}

fn filter_by_yellow(words: Vec<&str>, yellow: Vec<(usize, char)>) -> Vec<&str> {
    let mut res = vec![];
    'outer: for word in words {
        for (i, y) in &yellow {
            if !word.contains(&y.to_string()) {
                continue 'outer;
            }
            // yellow should not be in same spot.
            if word.chars().nth(*i).unwrap() == *y {
                continue 'outer;
            }
        }
        res.push(word);
    }
    res
}

fn filter_by_black(words: Vec<&str>, black: Vec<char>) -> Vec<&str> {
    let mut res = vec![];
    'outer: for word in words {
        for g in &black {
            if word.contains(&g.to_string()) {
                continue 'outer;
            }
        }
        res.push(word);
    }
    res
}

fn filter(
    words: Vec<&str>,
    green: Vec<char>,
    yellow: Vec<(usize, char)>,
    black: Vec<char>,
) -> Vec<&str> {
    let mut filtered = filter_by_green(words, green);
    filtered = filter_by_yellow(filtered, yellow);
    filtered = filter_by_black(filtered, black);
    filtered
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
    fn test_filter_by_yellow() {
        let words = vec!["audio", "clerk", "bloke"];
        let yellow = vec![(1, 'o'), (2, 'e'), (0, 'k')];
        assert_eq!(super::filter_by_yellow(words, yellow), vec!["bloke"]);
    }

    #[test]
    fn test_filter_by_black() {
        let words = vec!["audio", "clerk", "bloke"];
        let black = vec!['a', 'u', 'd', 'i', 'c', 'r'];
        assert_eq!(super::filter_by_black(words, black), vec!["bloke"]);
    }
}

fn main() {}

fn filter_by_green(words: Vec<&str>, green: Vec<char>) -> Vec<&'static str> {
    let mut res = vec![];
    for word in words {
        for (i, c) in word.chars().enumerate() {}
    }
    res
}

fn filter(
    words: Vec<&str>,
    green: Vec<char>,
    yellow: Vec<char>,
    gray: Vec<char>,
) -> Vec<&'static str> {
    let filtered_by_green = filter_by_green(words, green);
    filtered_by_green
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_filter() {
        let words = vec!["audio", "clerk", "bloke"];
        let green = vec!['\0', 'l', '\0', '\0', '\0'];
        let yellow = vec!['o', 'e', 'k'];
        let gray = vec!['a', 'u', 'd', 'i', 'c', 'r'];
        assert_eq!(super::filter(words, green, yellow, gray), vec!["bloke"]);
    }
}

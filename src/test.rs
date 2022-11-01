#[cfg(test)]
use crate::array_strings_are_equal;

#[test]
fn test_1() {
    let word1: Vec<String> = vec!["ab", "c"].iter().map(|str| str.to_string()).collect();
    let word2: Vec<String> = vec!["a", "bc"].iter().map(|str| str.to_string()).collect();

    let result = array_strings_are_equal(word1, word2);
    assert_eq!(result, true);
}

#[test]
fn test_2() {
    let word1 = vec!["a", "cb"].iter().map(|str| str.to_string()).collect();
    let word2 = vec!["ab", "c"].iter().map(|str| str.to_string()).collect();

    let result = array_strings_are_equal(word1, word2);
    assert_eq!(result, false);
}

#[test]
fn test_3() {
    let word1 = vec!["abc", "d", "defg"]
        .iter()
        .map(|str| str.to_string())
        .collect();
    let word2 = vec!["abcddefg"].iter().map(|str| str.to_string()).collect();

    let result = array_strings_are_equal(word1, word2);
    assert_eq!(result, true);
}

mod test;

pub fn array_strings_are_equal(word1: Vec<String>, word2: Vec<String>) -> bool {
    let string1 = word1.join("");
    let string2 = word2.join("");

    string1 == string2
}

fn main() {}

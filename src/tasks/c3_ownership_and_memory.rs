// This chapter is dedicated to the ownership, borrowing and slices

// OWNERSHIP
// ================================================================================================

// ----- 1 --------------------------------------
// Write a function `longest_owned(s1: String, s2: String) -> String` that returns the longer of
// two strings. Check that both original strings are moved into the function, and only the returned
// can still be used.
//
// You can implement the function and use it right inside the `string_ownership` function.
#[allow(dead_code)]
pub fn string_ownership() {
    fn longest_owned(s1: String, s2: String) -> String {
        if s1.len() > s2.len() {
            s1
        } else {
            s2
        }
    }

    let long = String::from("long");
    let short = String::from("short");
    println!("Longest: {}", longest_owned(long, short));
}

// BORROWING
// ================================================================================================

// ----- 2 --------------------------------------
// Write a function `print_length(s: ???)` that takes some string and prints its length without
// taking ownership. First use it with some random (censored) string, and then print this string to
// show that it was not moved and still available.
//
// You can implement the function and use it right inside the `simple_borrowing` function.
#[allow(dead_code)]
pub fn simple_borrowing() {
    fn print_length(s: &str) {
        println!("Length: {}", s.len());
    }
    let value = String::from("value123");
    print_length(&value);
    println!("Value: {}", value);
}

// ----- 3 --------------------------------------
// Implement a function `append_and_return_length(string: ???, suffix: ???) -> usize` that borrows
// some string, appends a suffix to it, and returns the new length. Then call it multiple times
// to check that the string was borrowed, not moved.
//
// You can implement the function and use it right inside the `hard_borrowing` function.
#[allow(dead_code)]
pub fn hard_borrowing() {
    fn append_and_return_length(string: &mut String, suffix: &str) -> usize {
        string.push_str(suffix);
        string.len()
    }

    let mut line = String::from("Some line");
    println!("0. len = {}, line = {}", line.len(), line);
    println!("1. len = {}, line = {}", append_and_return_length(&mut line, "11"), line);
    println!("2. len = {}, line = {}", append_and_return_length(&mut line, "22"), line);
}

// SLICES
// ================================================================================================

// ----- 4 --------------------------------------
// Write a function last_word(s: &str) -> &str that returns the last word from a string slice.
// Assume words are separated by spaces.
pub fn last_word(slice: &str) -> &str {
    slice.split_whitespace().last().unwrap_or("")
}

// ----- 5 --------------------------------------
// Write a function longest_word(sentence: &str) -> &str that returns the longest word in a
// sentence (string slice). If several words have the same maximum length, return the last one.
pub fn longest_word(sentence: &str) -> &str {
    let mut res = "";
    for word in sentence.split_whitespace() {
        if word.len() >= res.len() {
            res = word;
        }
    }
    res
}

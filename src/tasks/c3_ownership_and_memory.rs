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
        return s1
      }
      s2
    }

    let s1 = String::from("1234");
    let s2 = String::from("12345");

    let s_longest = longest_owned(s1, s2);

    println!("longest string is: {s_longest}");
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
  fn print_length(s: &String) {
    let len = s.len();
    println!("Length is : {len}");
  }

  let s1 = String::from("hello");

  print_length(&s1);
  println!("String {s1} is still alive!");
}

// ----- 3 --------------------------------------
// Implement a function `append_and_return_length(string: ???, suffix: ???) -> usize` that borrows
// some string, appends a suffix to it, and returns the new length. Then call it multiple times
// to check that the string was borrowed, not moved.
//
// You can implement the function and use it right inside the `hard_borrowing` function.
#[allow(dead_code)]
pub fn hard_borrowing() {
  fn append_and_return_length(string: &mut String, suffix: &String) -> usize {
    string.push_str(suffix);
    string.len()
  }

  let mut s = String::from("Hello");
  let mut len;
  let suffix = String::from(" world?");

  len = append_and_return_length(&mut s, &suffix); 
  println!("String: {s}, len: {len}");

  len = append_and_return_length(&mut s, &suffix);
  println!("String: {s}, len: {len}");

  len = append_and_return_length(&mut s, &suffix);
  println!("String: {s}, len: {len}");

}

// SLICES
// ================================================================================================

// ----- 4 --------------------------------------
// Write a function last_word(s: &str) -> &str that returns the last word from a string slice.
// Assume words are separated by spaces.
pub fn last_word(slice: &str) -> &str {
  let bytes = slice.as_bytes();
  let mut in_word = false;
  let mut ind = 0;
  let mut len = 0;

  for (i, &item) in bytes.iter().enumerate() {
    if item == b' ' {
      in_word = false;
    } else {
      if !in_word {
        ind = i;
        len = 1;
        in_word = true;
      } else {
        len += 1;
      }
    }
  }

  &slice[ind..ind + len]
}

// ----- 5 --------------------------------------
// Write a function longest_word(sentence: &str) -> &str that returns the longest word in a
// sentence (string slice). If several words have the same maximum length, return the last one.
pub fn longest_word(sentence: &str) -> &str {
  let bytes = sentence.as_bytes();
  let mut in_word = false;
  let mut ind = 0;
  let mut len = 0;
  let mut lg_ind = 0;
  let mut lg_len = 0;

  for (i, &item) in bytes.iter().enumerate() {
    if item == b' ' {
      in_word = false;
    } else {
      if !in_word {
        if len >= lg_len {
          lg_len = len;
          lg_ind = ind;
        }
        ind = i;
        len = 1;
        in_word = true;
      } else {
        len += 1;
      }
    }
  }

  if len >= lg_len {
    lg_len = len;
    lg_ind = ind;
  }
  &sentence[lg_ind..lg_ind + lg_len]
}

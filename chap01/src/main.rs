/// chap01.00 reverse
pub fn reverse(str: &str) -> String {
    str.chars().rev().collect()
}

///chap01.01 concatenate 1st, 3rd, 5th, and 7th character of "schooled" (odd positions)

pub fn cat_even(str: &str) -> String {
    str.chars()
        .enumerate()
        .filter_map(|(idx, elem)| if idx % 2 == 0 { Some(elem) } else { None })
        .collect()
}

///chap01.01.bis concatenate 1st, 3rd, 5th, and 7th character of "schooled" (even positions)

pub fn cat_odd(str: &str) -> String {
    str.chars()
        .enumerate()
        .filter_map(|(idx, elem)| if idx % 2 == 1 { Some(elem) } else { None })
        .collect()
}

///chap01.02 contactenating two str one after the other from head to tail

pub fn join_alt(str1: &str, str2: &str) -> String {
    str1.chars()
        .zip(str2.chars())
        .map(|(ch1, ch2)| format!("{}{}", ch1, ch2))
        .collect::<String>()
}

///chap01.03 convert the sentence : “Now I need a drink, alcoholic of course, after the heavy lectures involving quantum mechanics”
/// into the number of letter in each word.

pub fn convert_sentence() -> Vec<u32> {
    let sentence = "Now I need a drink, alcoholic of course, after the heavy lectures involving quantum mechanics.";
    sentence
        .split_whitespace()
        .map(|s| (s.trim_matches(|c| c == ',' || c == '.').len() % 10) as u32)
        .collect()
}

/// chap01.04
///
/// use for chap01.04
use std::collections::HashMap;

pub fn atomic_symbols<'a>() -> HashMap<&'a str, u32> {
    let sentence = "Hi He Lied Because Boron Could Not Oxidize Fluorine. New Nations Might Also Sign Peace Security Clause. Arthur King Can";
    let word_length = sentence.split_whitespace().count();

    let indexes: Vec<usize> = [1, 5, 6, 7, 8, 9, 15, 16, 19]
        .iter()
        .map(|s| ((s - 1) as usize))
        .collect();

    let hash: HashMap<usize, usize> = (0..word_length)
        .map(|s| if indexes.contains(&s) { (s, 1) } else { (s, 2) })
        .collect();

    sentence
        .split_whitespace()
        .enumerate()
        .map(|(idx, word)| (&word[0..hash[&idx]], ((idx + 1) as u32)))
        .collect()
}

/// chap01.05
/// What the hell are n-grams or bi-grams ?
///
#[allow(unused_variables)]
pub fn chap05() {
    let sentence = "I am an NLPer";
}

/// I don't know how to avoid putting a main function in main.rs without too much verbose entry (like include standard library and macros, etc..)
fn main() {
    println!("Please, use \"cargo test\" to check the results of the tests.");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_chap01_00_reverse() {
        assert_eq!("Hello", reverse("olleH"));
    }
    #[test]
    fn test_chap01_01_cat_odd() {
        assert_eq!("cold", cat_odd("schooled"));
    }
    #[test]
    fn test_chap01_01_cat_even() {
        assert_eq!("shoe", cat_even("schooled"));
    }

    #[test]
    fn test_chap01_02_joinalt() {
        assert_eq!("schooled", join_alt("shoe", "cold"));
    }

    #[test]
    fn test_chap01_03_convert_sentence() {
        assert_eq!(
            vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5, 8, 9, 7, 9],
            convert_sentence()
        );
    }

    #[test]
    fn test_chap01_04_atomic() {
        let periodic_table = [
            ("H", 1),
            ("He", 2),
            ("Li", 3),
            ("Be", 4),
            ("B", 5),
            ("C", 6),
            ("N", 7),
            ("O", 8),
            ("F", 9),
            ("Ne", 10),
            ("Na", 11),
            ("Mi", 12),
            ("Al", 13),
            ("Si", 14),
            ("P", 15),
            ("S", 16),
            ("Cl", 17),
            ("Ar", 18),
            ("K", 19),
            ("Ca", 20),
        ]
        .iter()
        .cloned()
        .collect::<HashMap<_, _>>();
        assert_eq!(periodic_table, atomic_symbols());
    }
}

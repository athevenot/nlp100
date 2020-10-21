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

///chap01.04
#[allow(unused_variables)]

pub fn atomic_symbols() -> bool {
    let sentence = "Hi He Lied Because Boron Could Not Oxidize Fluorine. New Nations Might Also Sign Peace Security Clause. Arthur King Can";
    let word_length = sentence.split_whitespace().count();
    println!("{}", word_length);
    true
}

fn main() {
    atomic_symbols();
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
    fn test_atomic() {
        assert_eq!(true, true);
    }
}

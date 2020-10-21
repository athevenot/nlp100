/// chap01 reverse
pub fn reverse(str: &str) -> String {
    str.chars().rev().collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_chap01_00_reverse() {
        assert_eq!("Hello", reverse("olleH"));
    }
}
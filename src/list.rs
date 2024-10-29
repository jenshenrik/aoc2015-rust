use fancy_regex::Regex;

pub fn is_nice(input: &str) -> bool {
    contains_atleast_3_vowels(input)
        && contains_double_letter(input)
        && contains_no_disallowed_strings(input)
}

pub fn better_is_nice(input: &str) -> bool {
    contains_spaced_double_letter(input) && contains_multiple_pairs(input)
}

fn contains_atleast_3_vowels(input: &str) -> bool {
    let rx = Regex::new(r"(a|e|i|o|u)").unwrap();

    rx.captures_iter(input).count() >= 3
}

fn contains_double_letter(input: &str) -> bool {
    let rx = Regex::new(r"(.)\1").unwrap();

    rx.is_match(input).unwrap()
}

fn contains_no_disallowed_strings(input: &str) -> bool {
    let rx = Regex::new(r"(ab|cd|pq|xy)").unwrap();

    rx.captures_iter(input).count() == 0
}

fn contains_spaced_double_letter(input: &str) -> bool {
    let rx = Regex::new(r"(.).(\1)").expect("unable to create regex");

    let results = match rx.captures(input).expect("error running regex") {
        Some(v) => v.len(),
        None => 0,
    };

    results >= 2
}

fn contains_multiple_pairs(input: &str) -> bool {
    let rx = Regex::new(r"(..).*(\1)").expect("unable to create regex");

    let results = match rx.captures(input).expect("error running regex") {
        Some(v) => v.len(),
        None => 0,
    };

    results >= 2
}

#[cfg(test)]
mod tests {
    use crate::list::{
        better_is_nice, contains_atleast_3_vowels, contains_double_letter,
        contains_no_disallowed_strings, contains_spaced_double_letter, is_nice,
    };

    use super::contains_multiple_pairs;

    #[test]
    fn test_three_vowels_nice() {
        assert!(contains_atleast_3_vowels("ugknbfddgicrmopn"))
    }

    #[test]
    fn test_three_vowels_naughty() {
        assert!(!contains_atleast_3_vowels("dvszwmarrgswjxmb"));
    }

    #[test]
    fn test_contains_double_letter_nice() {
        assert!(contains_double_letter("ugknbfddgicrmopn"))
    }

    #[test]
    fn test_contains_double_letter_naughty() {
        assert!(!contains_double_letter("jchzalrnumimnmhp"));
    }

    #[test]
    fn test_contains_no_disallowed_strings_nice() {
        assert!(contains_no_disallowed_strings("ugknbfddgicrmopn"));
    }

    #[test]
    fn test_contains_no_disallowed_strings_naughty() {
        assert!(!contains_no_disallowed_strings("haegwjzuvuyypxyu"));
    }

    #[test]
    fn test_is_nice_true() {
        assert!(is_nice("ugknbfddgicrmopn"));
    }

    #[test]
    fn test_is_nice_false_1() {
        assert!(!is_nice("jchzalrnumimnmhp"));
    }

    #[test]
    fn test_is_nice_false_2() {
        assert!(!is_nice("haegwjzuvuyypxyu"));
    }

    #[test]
    fn test_is_nice_false_3() {
        assert!(!is_nice("dvszwmarrgswjxmb"));
    }

    #[test]
    fn test_contains_spaced_double_letter_false() {
        assert!(!contains_spaced_double_letter("uurcxstgmygtbstg"));
    }

    #[test]
    fn test_contains_spaced_double_letter_true_1() {
        assert!(contains_spaced_double_letter("qjhvhtzxzqqjkmpb"));
    }

    #[test]
    fn test_contains_spaced_double_letter_true_2() {
        assert!(contains_spaced_double_letter("xxyxx"));
    }

    #[test]
    fn test_better_is_nice_true_1() {
        assert!(better_is_nice("qjhvhtzxzqqjkmpb"));
    }

    #[test]
    fn test_better_is_nice_true_2() {
        assert!(better_is_nice("xxyxx"));
    }

    #[test]
    fn test_better_is_nice_false_1() {
        assert!(!better_is_nice("uurcxstgmygtbstg"));
    }

    #[test]
    fn test_better_is_nice_false_2() {
        assert!(!better_is_nice("ieodomkazucvgmuy"));
    }

    #[test]
    fn test_contains_multiple_pairs_true_1() {
        assert!(contains_multiple_pairs("qjhvhtzxzqqjkmpb"));
    }

    #[test]
    fn test_contains_multiple_pairs_true_2() {
        assert!(contains_multiple_pairs("xxyxx"));
    }

    #[test]
    fn test_contains_multiple_pairs_true_3() {
        assert!(contains_multiple_pairs("uurcxstgmygtbstg"));
    }

    #[test]
    fn test_contains_multiple_pairs_false() {
        assert!(!contains_multiple_pairs("ieodomkazucvgmuy"));
    }
}

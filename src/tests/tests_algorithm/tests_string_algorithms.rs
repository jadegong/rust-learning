use crate::algorithm::string_algorithms;

#[test]
pub fn test_length_of_last_word_58() {
    let test_s = String::from("Hello World");
    let test_ret = string_algorithms::length_of_last_word_58(test_s);
    assert_eq!(test_ret, 5);
    let test_s = String::from("   fly me   to   the moon  ");
    let test_ret = string_algorithms::length_of_last_word_58(test_s);
    assert_eq!(test_ret, 4);
    let test_s = String::from("luffy is still joyboy");
    let test_ret = string_algorithms::length_of_last_word_58(test_s);
    assert_eq!(test_ret, 6);
}

#[test]
pub fn test_longest_common_prefix_14() {
    let test_strs: Vec<String> = vec![String::from("flower"), String::from("flow"), String::from("flight")];
    let expected_ret = string_algorithms::longest_common_prefix_14(test_strs);
    assert_eq!(expected_ret, String::from("fl"));
    let test_strs: Vec<String> = vec![String::from("dog"), String::from("racecar"), String::from("car")];
    let expected_ret = string_algorithms::longest_common_prefix_14(test_strs);
    assert_eq!(expected_ret, String::from(""));
}

#[test]
pub fn test_reverse_words_151() {
    let test_str = String::from("the sky is blue");
    let expected_ret = String::from("blue is sky the");
    let test_ret = string_algorithms::reverse_words_151(test_str);
    assert_eq!(test_ret, expected_ret);
    let test_str = String::from("  hello world  ");
    let expected_ret = String::from("world hello");
    let test_ret = string_algorithms::reverse_words_151(test_str);
    assert_eq!(test_ret, expected_ret);
    let test_str = String::from("a good   example");
    let expected_ret = String::from("example good a");
    let test_ret = string_algorithms::reverse_words_151(test_str);
    assert_eq!(test_ret, expected_ret);
}

#[test]
pub fn test_str_str_28() {
    let test_haystack = String::from("sadbutsad");
    let test_needle = String::from("sad");
    let test_ret = string_algorithms::str_str_28(test_haystack, test_needle);
    assert_eq!(test_ret, 0);
    let test_haystack = String::from("leetcode");
    let test_needle = String::from("leeto");
    let test_ret = string_algorithms::str_str_28(test_haystack, test_needle);
    assert_eq!(test_ret, -1);
    let test_haystack = String::from("DBCABDABDABA");
    let test_needle = String::from("ABDABA");
    let test_ret = string_algorithms::str_str_28(test_haystack, test_needle);
    assert_eq!(test_ret, 6);
}

#[test]
pub fn test_is_palindrome_125() {
    let test_str: String = String::from("A man, a plan, a canal: Panama");
    assert!(string_algorithms::is_palindrome_125(test_str));
    let test_str: String = String::from("race a car");
    assert!(!string_algorithms::is_palindrome_125(test_str));
    let test_str: String = String::from(" ");
    assert!(string_algorithms::is_palindrome_125(test_str));
}

#[test]
pub fn test_is_subsequence_392() {
    let test_s: String = String::from("abc");
    let test_t: String = String::from("ahbgdc");
    assert!(string_algorithms::is_subsequence_392(test_s, test_t));
    let test_s: String = String::from("axc");
    let test_t: String = String::from("ahbgdc");
    assert!(!string_algorithms::is_subsequence_392(test_s, test_t));
}


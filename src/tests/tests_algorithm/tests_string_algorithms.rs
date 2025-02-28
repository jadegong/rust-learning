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

#[test]
pub fn test_length_of_longest_substring_3() {
    let test_s: String = String::from("abcabcbb");
    let expected_ret = 3;
    let test_ret = string_algorithms::length_of_longest_substring_3(test_s);
    assert_eq!(test_ret, expected_ret);
    let test_s: String = String::from("bbbbb");
    let expected_ret = 1;
    let test_ret = string_algorithms::length_of_longest_substring_3(test_s);
    assert_eq!(test_ret, expected_ret);
    let test_s: String = String::from("pwwkew");
    let expected_ret = 3;
    let test_ret = string_algorithms::length_of_longest_substring_3(test_s);
    assert_eq!(test_ret, expected_ret);
}

#[test]
pub fn test_min_window_76() {
    let test_s: String = String::from("ADOBECODEBANC");
    let test_t: String = String::from("ABC");
    let expected_ret: String = String::from("BANC");
    let test_ret = string_algorithms::min_window_76(test_s, test_t);
    assert_eq!(expected_ret, test_ret);
    let test_s: String = String::from("a");
    let test_t: String = String::from("a");
    let expected_ret: String = String::from("a");
    let test_ret = string_algorithms::min_window_76(test_s, test_t);
    assert_eq!(expected_ret, test_ret);
    let test_s: String = String::from("a");
    let test_t: String = String::from("aa");
    let expected_ret: String = String::from("");
    let test_ret = string_algorithms::min_window_76(test_s, test_t);
    assert_eq!(expected_ret, test_ret);
}

#[test]
pub fn test_can_construct_383() {
    let test_s: String = String::from("a");
    let test_t: String = String::from("b");
    let test_ret = string_algorithms::can_construct_383(test_s, test_t);
    assert_eq!(test_ret, false);
    let test_s: String = String::from("aa");
    let test_t: String = String::from("ab");
    let test_ret = string_algorithms::can_construct_383(test_s, test_t);
    assert_eq!(test_ret, false);
    let test_s: String = String::from("aa");
    let test_t: String = String::from("aab");
    let test_ret = string_algorithms::can_construct_383(test_s, test_t);
    assert_eq!(test_ret, true);
}

#[test]
pub fn test_is_isomorphic_205() {
    let test_s: String = String::from("egg");
    let test_t: String = String::from("add");
    assert_eq!(string_algorithms::is_isomorphic_205(test_s, test_t), true);
    let test_s: String = String::from("foo");
    let test_t: String = String::from("bar");
    assert_eq!(string_algorithms::is_isomorphic_205(test_s, test_t), false);
    let test_s: String = String::from("paper");
    let test_t: String = String::from("title");
    assert_eq!(string_algorithms::is_isomorphic_205(test_s, test_t), true);
    let test_s: String = String::from("badc");
    let test_t: String = String::from("baba");
    assert_eq!(string_algorithms::is_isomorphic_205(test_s, test_t), false);
}

#[test]
pub fn test_word_pattern_290() {
    let test_pattern: String = String::from("abba");
    let test_s: String = String::from("dog cat cat dog");
    assert_eq!(string_algorithms::word_pattern_290(test_pattern, test_s), true);
    let test_pattern: String = String::from("abba");
    let test_s: String = String::from("dog cat cat fish");
    assert_eq!(string_algorithms::word_pattern_290(test_pattern, test_s), false);
    let test_pattern: String = String::from("aaaa");
    let test_s: String = String::from("dog cat cat dog");
    assert_eq!(string_algorithms::word_pattern_290(test_pattern, test_s), false);
    let test_pattern: String = String::from("deadbeef");
    let test_s: String = String::from("d e a d b e e f");
    assert_eq!(string_algorithms::word_pattern_290(test_pattern, test_s), true);
}

#[test]
pub fn test_valid_anagram_242() {
    let test_s: String = String::from("anagram");
    let test_t: String = String::from("nagaram");
    assert_eq!(string_algorithms::valid_anagram_242(test_s, test_t), true);
    let test_s: String = String::from("rat");
    let test_t: String = String::from("car");
    assert_eq!(string_algorithms::valid_anagram_242(test_s, test_t), false);
}

#[test]
pub fn test_is_valid_20() {
    let test_s: String = String::from("()[]{}");
    assert_eq!(string_algorithms::is_valid_20(test_s), true);
    let test_s: String = String::from("((])");
    assert_eq!(string_algorithms::is_valid_20(test_s), false);
}

#[test]
pub fn test_min_add_to_make_valid_921() {
    let test_s: String = String::from("())");
    let expected_ret = 1;
    assert_eq!(string_algorithms::min_add_to_make_valid_921(test_s), expected_ret);
    let test_s: String = String::from("(((");
    let expected_ret = 3;
    assert_eq!(string_algorithms::min_add_to_make_valid_921(test_s), expected_ret);
}


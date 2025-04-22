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

#[test]
pub fn test_max_score_1422() {
    let test_s: String = String::from("011101");
    let expected_ret = 5;
    assert_eq!(string_algorithms::max_score_1422(test_s), expected_ret);
    let test_s: String = String::from("00111");
    let expected_ret = 5;
    assert_eq!(string_algorithms::max_score_1422(test_s), expected_ret);
    let test_s: String = String::from("1111");
    let expected_ret = 3;
    assert_eq!(string_algorithms::max_score_1422(test_s), expected_ret);
}

#[test]
pub fn test_minimum_recolors_2379() {
    let test_blocks: String = String::from("WBBWWBBWBW");
    let test_k = 7;
    let expected_ret = 3;
    assert_eq!(string_algorithms::minimum_recolors_2379(test_blocks, test_k), expected_ret);
    let test_blocks: String = String::from("WBWBBBW");
    let test_k = 2;
    let expected_ret = 0;
    assert_eq!(string_algorithms::minimum_recolors_2379(test_blocks, test_k), expected_ret);
}

#[test]
pub fn test_multiply_43() {
    let test_num1: String = String::from("2");
    let test_num2: String = String::from("3");
    let expected_ret: String = String::from("6");
    assert_eq!(string_algorithms::multiply_43(test_num1, test_num2), expected_ret);
    let test_num1: String = String::from("123");
    let test_num2: String = String::from("456");
    let expected_ret: String = String::from("56088");
    assert_eq!(string_algorithms::multiply_43(test_num1, test_num2), expected_ret);
    let test_num1: String = String::from("9133");
    let test_num2: String = String::from("0");
    let expected_ret: String = String::from("0");
    assert_eq!(string_algorithms::multiply_43(test_num1, test_num2), expected_ret);
    let test_num1: String = String::from("6");
    let test_num2: String = String::from("501");
    let expected_ret: String = String::from("3006");
    assert_eq!(string_algorithms::multiply_43(test_num1, test_num2), expected_ret);
}

#[test]
pub fn test_add_binary_67() {
    let test_a: String = String::from("11");
    let test_b: String = String::from("1");
    let expected_ret: String = String::from("100");
    assert_eq!(string_algorithms::add_binary_67(test_a, test_b), expected_ret);
    let test_a: String = String::from("1010");
    let test_b: String = String::from("1011");
    let expected_ret: String = String::from("10101");
    assert_eq!(string_algorithms::add_binary_67(test_a, test_b), expected_ret);
}

#[test]
pub fn test_word_break_139() {
    let test_s: String = String::from("leetcode");
    let word_dict: Vec<String> = vec![String::from("leet"), String::from("code")];
    assert_eq!(string_algorithms::word_break_139(test_s, word_dict), true);
    let test_s: String = String::from("abcd");
    let word_dict: Vec<String> = vec![String::from("a"), String::from("abc"), String::from("b"), String::from("cd")];
    assert_eq!(string_algorithms::word_break_139(test_s, word_dict), true);
    let test_s: String = String::from("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaabaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa");
    let word_dict: Vec<String> = vec![String::from("a"), String::from("aa"), String::from("aaa"), String::from("aaaa"), String::from("aaaaa"), String::from("aaaaaa"), String::from("aaaaaaa"), String::from("aaaaaaaa"), String::from("aaaaaaaaa"), String::from("aaaaaaaaaa")];
    assert_eq!(string_algorithms::word_break_139(test_s, word_dict), false);
}

#[test]
pub fn test_reverse_vowels_345() {
    let test_s: String = String::from("IceCreAm");
    let expected_ret: String = String::from("AceCreIm");
    assert_eq!(string_algorithms::reverse_vowels_345(test_s), expected_ret);
}

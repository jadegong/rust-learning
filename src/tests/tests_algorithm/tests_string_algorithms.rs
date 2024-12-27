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

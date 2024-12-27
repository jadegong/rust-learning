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

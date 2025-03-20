use crate::algorithm::normal_algorithms;

#[test]
pub fn test_reverse_bits_190() {
    let test_x: u32 = 43261596;
    let expected_ret: u32 = 964176192;
    assert_eq!(normal_algorithms::reverse_bits_190(test_x), expected_ret);
    let test_x: u32 = 4294967293;
    let expected_ret: u32 = 3221225471;
    assert_eq!(normal_algorithms::reverse_bits_190(test_x), expected_ret);
}

#[test]
pub fn test_hamming_weight_191() {
    let test_n: i32 = 11;
    let expected_ret: i32 = 3;
    assert_eq!(normal_algorithms::hamming_weight_191(test_n), expected_ret);
    let test_n: i32 = 2147483645;
    let expected_ret: i32 = 30;
    assert_eq!(normal_algorithms::hamming_weight_191(test_n), expected_ret);
}

#[test]
pub fn test_single_number_136() {
    let test_nums: Vec<i32> = vec![2,2,1];
    let expected_ret: i32 = 1;
    assert_eq!(normal_algorithms::single_number_136(test_nums), expected_ret);
    let test_nums: Vec<i32> = vec![4,1,2,1,2];
    let expected_ret: i32 = 4;
    assert_eq!(normal_algorithms::single_number_136(test_nums), expected_ret);
}

#[test]
pub fn test_single_number_137() {
    let test_nums: Vec<i32> = vec![2,2,3,2];
    let expected_ret: i32 = 3;
    assert_eq!(normal_algorithms::single_number_137(test_nums), expected_ret);
    let test_nums: Vec<i32> = vec![0,1,0,1,0,1,99];
    let expected_ret: i32 = 99;
    assert_eq!(normal_algorithms::single_number_137(test_nums), expected_ret);
}

#[test]
pub fn test_is_palindrome_9() {
    let test_x = 121;
    let expected_ret = true;
    assert_eq!(normal_algorithms::is_palindrome_9(test_x), expected_ret);
    let test_x = -121;
    let expected_ret = false;
    assert_eq!(normal_algorithms::is_palindrome_9(test_x), expected_ret);
    let test_x = 10;
    let expected_ret = false;
    assert_eq!(normal_algorithms::is_palindrome_9(test_x), expected_ret);
}

#[test]
pub fn test_plus_one_66() {
    let test_nums: Vec<i32> = vec![1,2,3];
    let expected_nums: Vec<i32> = vec![1,2,4];
    assert_eq!(normal_algorithms::plus_one_66(test_nums), expected_nums);
    let test_nums: Vec<i32> = vec![4,3,2,9];
    let expected_nums: Vec<i32> = vec![4,3,3,0];
    assert_eq!(normal_algorithms::plus_one_66(test_nums), expected_nums);
}

#[test]
pub fn test_xor_all_nums_2425() {
    let test_nums1: Vec<i32> = vec![2,1,3];
    let test_nums2: Vec<i32> = vec![10,2,5,0];
    let expected_ret = 13;
    assert_eq!(normal_algorithms::xor_all_nums_2425(test_nums1, test_nums2), expected_ret);
    let test_nums1: Vec<i32> = vec![1,2];
    let test_nums2: Vec<i32> = vec![3,4];
    let expected_ret = 0;
    assert_eq!(normal_algorithms::xor_all_nums_2425(test_nums1, test_nums2), expected_ret);
}

#[test]
pub fn test_colored_cells_1579() {
    let test_n = 1;
    let expected_ret = 1;
    assert_eq!(normal_algorithms::colored_cells_2579(test_n), expected_ret);
    let test_n = 3;
    let expected_ret = 13;
    assert_eq!(normal_algorithms::colored_cells_2579(test_n), expected_ret);
}

#[test]
pub fn test_check_powers_of_three_1780() {
    let test_n = 12;
    assert_eq!(normal_algorithms::check_powers_of_three_1780(test_n), true);
    let test_n = 91;
    assert_eq!(normal_algorithms::check_powers_of_three_1780(test_n), true);
    let test_n = 21;
    assert_eq!(normal_algorithms::check_powers_of_three_1780(test_n), false);
}

#[test]
pub fn test_my_sqrt_69() {
    let test_x: i32 = 4;
    let expected_ret: i32 = 2;
    assert_eq!(normal_algorithms::my_sqrt_69(test_x), expected_ret);
    let test_x: i32 = 8;
    let expected_ret: i32 = 2;
    assert_eq!(normal_algorithms::my_sqrt_69(test_x), expected_ret);
    let test_x: i32 = 2147395599;
    let expected_ret: i32 = 46339;
    assert_eq!(normal_algorithms::my_sqrt_69(test_x), expected_ret);
}

#[test]
pub fn test_eval_rpn_150() {
    let test_tokens: Vec<String> = vec![String::from("10"),String::from("6"),String::from("9"),String::from("3"),String::from("+"),String::from("-11"),String::from("*"),String::from("/"),String::from("*"),String::from("17"),String::from("+"),String::from("5"),String::from("+")];
    let expected_ret = 22;
    assert_eq!(normal_algorithms::eval_rpn_150(test_tokens), expected_ret);
    let test_tokens: Vec<String> = vec![String::from("4"),String::from("13"),String::from("5"),String::from("/"),String::from("+")];
    let expected_ret = 6;
    assert_eq!(normal_algorithms::eval_rpn_150(test_tokens), expected_ret);
}

#[test]
pub fn test_convert_to_title_168() {
    let test_num: i32 = 26;
    let expected_ret: String = String::from("Z");
    assert_eq!(normal_algorithms::convert_to_title_168(test_num), expected_ret);
    let test_num: i32 = 28;
    let expected_ret: String = String::from("AB");
    assert_eq!(normal_algorithms::convert_to_title_168(test_num), expected_ret);
    let test_num: i32 = 701;
    let expected_ret: String = String::from("ZY");
    assert_eq!(normal_algorithms::convert_to_title_168(test_num), expected_ret);
}

#[test]
pub fn test_title_to_number_171() {
    let test_title: String = String::from("Z");
    let expected_ret = 26;
    assert_eq!(normal_algorithms::title_to_number_171(test_title), expected_ret);
    let test_title: String = String::from("AB");
    let expected_ret = 28;
    assert_eq!(normal_algorithms::title_to_number_171(test_title), expected_ret);
    let test_title: String = String::from("ZY");
    let expected_ret = 701;
    assert_eq!(normal_algorithms::title_to_number_171(test_title), expected_ret);
}

#[test]
pub fn test_range_bitwise_and_201() {
    let test_left: i32 = 5;
    let test_right: i32 = 7;
    let expected_ret: i32 = 4;
    assert_eq!(normal_algorithms::range_bitwise_and_201(test_left, test_right), expected_ret);
    let test_left: i32 = 0;
    let test_right: i32 = 0;
    let expected_ret: i32 = 0;
    assert_eq!(normal_algorithms::range_bitwise_and_201(test_left, test_right), expected_ret);
    let test_left: i32 = 1;
    let test_right: i32 = 2147483647;
    let expected_ret: i32 = 0;
    assert_eq!(normal_algorithms::range_bitwise_and_201(test_left, test_right), expected_ret);
    let test_left: i32 = 6;
    let test_right: i32 = 12;
    let expected_ret: i32 = 0;
    assert_eq!(normal_algorithms::range_bitwise_and_201(test_left, test_right), expected_ret);
}

#[test]
pub fn test_count_primes_204() {
    let test_n = 10;
    let expected_ret = 4;
    assert_eq!(normal_algorithms::count_primes_204(test_n), expected_ret);
    let test_n = 5000000;
    let expected_ret = 348513;
    assert_eq!(normal_algorithms::count_primes_204(test_n), expected_ret);
}

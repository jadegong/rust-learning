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

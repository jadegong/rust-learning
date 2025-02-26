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

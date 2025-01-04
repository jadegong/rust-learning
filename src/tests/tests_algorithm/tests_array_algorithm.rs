use crate::algorithm::array_algorithm;

#[test]
pub fn test_remove_element_27() {
    let mut test_nums: Vec<i32> = vec![0, 1, 2, 2, 3, 0, 4, 2];
    let val: i32 = 2;
    let ret_len = array_algorithm::remove_element_27(&mut test_nums, val);
    assert_eq!(ret_len, 5);
}

#[test]
pub fn test_remove_duplicates_26() {
    let mut test_nums: Vec<i32> = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
    let ret_len = array_algorithm::remove_duplicates_26(&mut test_nums);
    assert_eq!(ret_len, 5);
}

#[test]
pub fn test_remove_duplicates_80() {
    let mut test_nums: Vec<i32> = vec![0, 0, 1, 1, 1, 1, 2, 3, 3];
    let expected_nums: Vec<i32> = vec![0, 0, 1, 1, 2, 3, 3];
    let ret_len = array_algorithm::remove_duplicates_80(&mut test_nums);
    assert_eq!(ret_len, 7);
    for (i, el) in expected_nums.iter().enumerate() {
        assert_eq!(test_nums[i], *el);
    }
}

#[test]
pub fn test_merge_sorted_array_88() {
    let mut nums1: Vec<i32> = vec![1, 2, 3, 0, 0, 0];
    let mut nums2: Vec<i32> = vec![2, 5, 6];
    array_algorithm::merge_sorted_array_88(&mut nums1, 3, &mut nums2, 3);
    let expected_nums: Vec<i32> = vec![1, 2, 2, 3, 5, 6];
    for (i, el) in nums1.iter().enumerate() {
        assert_eq!(*el, expected_nums[i]);
    }
}

#[test]
pub fn test_min_sub_array_len_209() {
    let test_nums: Vec<i32> = vec![2,3,1,2,4,3];
    let expected_ret = 2;
    let test_ret = array_algorithm::min_sub_array_len_209(7, test_nums);
    assert_eq!(expected_ret, test_ret);
    let test_nums: Vec<i32> = vec![1,4,4];
    let expected_ret = 1;
    let test_ret = array_algorithm::min_sub_array_len_209(4, test_nums);
    assert_eq!(expected_ret, test_ret);
    let test_nums: Vec<i32> = vec![1,1,1,1,1,1,1,1];
    let expected_ret = 0;
    let test_ret = array_algorithm::min_sub_array_len_209(11, test_nums);
    assert_eq!(expected_ret, test_ret);
}

#[test]
pub fn test_max_sliding_window_239() {
    let test_nums: Vec<i32> = vec![1,3,-1,-3,5,3,6,7];
    let expected_ret = vec![3, 3, 5, 5, 6, 7];
    let test_ret = array_algorithm::max_sliding_window_239(test_nums, 3);
    assert_eq!(test_ret, expected_ret);
    let test_nums: Vec<i32> = vec![1];
    let expected_ret = vec![1];
    let test_ret = array_algorithm::max_sliding_window_239(test_nums, 1);
    assert_eq!(test_ret, expected_ret);
}

#[test]
pub fn test_majority_element_169() {
    let test_nums: Vec<i32> = vec![3, 2, 3];
    let expected_ret = 3;
    let test_ret = array_algorithm::majority_element_169(test_nums);
    assert_eq!(test_ret, expected_ret);
    let test_nums: Vec<i32> = vec![2, 2, 1, 1, 1, 2, 2];
    let expected_ret = 2;
    let test_ret = array_algorithm::majority_element_169(test_nums);
    assert_eq!(test_ret, expected_ret);
}

#[test]
pub fn test_set_zeroes_73() {
    let mut test_matrix: Vec<Vec<i32>> = vec![vec![1,1,1],vec![1,0,1],vec![1,1,1]];
    let expected_matrix: Vec<Vec<i32>> = vec![vec![1,0,1],vec![0,0,0],vec![1,0,1]];
    array_algorithm::set_zeroes_73(&mut test_matrix);
    assert_eq!(test_matrix, expected_matrix);
}

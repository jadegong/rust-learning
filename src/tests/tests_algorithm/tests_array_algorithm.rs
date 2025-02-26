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

#[test]
pub fn test_rotate_48() {
    let mut test_matrix: Vec<Vec<i32>> = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    let expected_matrix: Vec<Vec<i32>> = vec![vec![7, 4, 1], vec![8, 5, 2], vec![9, 6, 3]];
    array_algorithm::rotate_48(&mut test_matrix);
    assert_eq!(test_matrix, expected_matrix);
    let mut test_matrix: Vec<Vec<i32>> = vec![vec![5, 1, 9, 11], vec![2, 4, 8, 10], vec![13, 3, 6, 7], vec![15, 14, 12, 16]];
    let expected_matrix: Vec<Vec<i32>> = vec![vec![15, 13, 2, 5], vec![14, 3, 4, 1], vec![12, 6, 8, 9], vec![16, 7, 10, 11]];
    array_algorithm::rotate_48(&mut test_matrix);
    assert_eq!(test_matrix, expected_matrix);
}

#[test]
pub fn test_spiral_order_54() {
    let test_matrix: Vec<Vec<i32>> = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    let expected_ret: Vec<i32> = vec![1, 2, 3, 6, 9, 8, 7, 4, 5];
    let test_ret = array_algorithm::spiral_order_54(test_matrix);
    assert_eq!(test_ret, expected_ret);
    let test_matrix: Vec<Vec<i32>> = vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8], vec![9, 10, 11, 12]];
    let expected_ret: Vec<i32> = vec![1, 2, 3, 4, 8, 12, 11, 10, 9, 5, 6, 7];
    let test_ret = array_algorithm::spiral_order_54(test_matrix);
    assert_eq!(test_ret, expected_ret);
    let test_matrix: Vec<Vec<i32>> = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9], vec![10, 11, 12], vec![13, 14, 15]];
    let expected_ret: Vec<i32> = vec![1, 2, 3, 6, 9, 12, 15, 14, 13, 10, 7, 4, 5, 8, 11];
    let test_ret = array_algorithm::spiral_order_54(test_matrix);
    assert_eq!(test_ret, expected_ret);
}

#[test]
pub fn test_two_sum_1() {
    let test_nums: Vec<i32> = vec![2,7,11,15];
    let test_target: i32 = 9;
    let expected_ret: Vec<i32> = vec![0,1];
    assert_eq!(array_algorithm::two_sum_1(test_nums, test_target), expected_ret);
    let test_nums: Vec<i32> = vec![3,2,4];
    let test_target: i32 = 6;
    let expected_ret: Vec<i32> = vec![1,2];
    assert_eq!(array_algorithm::two_sum_1(test_nums, test_target), expected_ret);
    let test_nums: Vec<i32> = vec![3,3];
    let test_target: i32 = 6;
    let expected_ret: Vec<i32> = vec![0,1];
    assert_eq!(array_algorithm::two_sum_1(test_nums, test_target), expected_ret);
}

#[test]
pub fn test_contains_nearby_duplicate_219() {
    let test_nums: Vec<i32> = vec![1,2,3,1];
    let test_k: i32 = 3;
    assert_eq!(array_algorithm::contains_nearby_duplicate_219(test_nums, test_k), true);
    let test_nums: Vec<i32> = vec![1,0,1,1];
    let test_k: i32 = 1;
    assert_eq!(array_algorithm::contains_nearby_duplicate_219(test_nums, test_k), true);
    let test_nums: Vec<i32> = vec![1,2,3,1,2,3];
    let test_k: i32 = 2;
    assert_eq!(array_algorithm::contains_nearby_duplicate_219(test_nums, test_k), false);
}

#[test]
pub fn test_summary_ranges_228() {
    let test_nums: Vec<i32> = vec![0,1,2,4,5,7];
    let expected_ret: Vec<String> = vec![String::from("0->2"), String::from("4->5"), String::from("7")];
    assert_eq!(array_algorithm::summary_ranges_228(test_nums), expected_ret);
    let test_nums: Vec<i32> = vec![0,2,3,4,6,8,9];
    let expected_ret: Vec<String> = vec![String::from("0"), String::from("2->4"), String::from("6"), String::from("8->9")];
    assert_eq!(array_algorithm::summary_ranges_228(test_nums), expected_ret);
}

#[test]
pub fn test_insert_57() {
    let intervals: Vec<Vec<i32>> = vec![vec![1,3], vec![6,9]];
    let new_interval: Vec<i32> = vec![2,5];
    let expected_ret: Vec<Vec<i32>> = vec![vec![1,5], vec![6,9]];
    assert_eq!(array_algorithm::insert_57(intervals, new_interval), expected_ret);
    let intervals: Vec<Vec<i32>> = vec![vec![1,2],vec![3,5],vec![6,7],vec![8,10],vec![12,16]];
    let new_interval: Vec<i32> = vec![4,8];
    let expected_ret: Vec<Vec<i32>> = vec![vec![1,2],vec![3,10],vec![12,16]];
    assert_eq!(array_algorithm::insert_57(intervals, new_interval), expected_ret);
}

#[test]
pub fn test_find_min_153() {
    let test_nums: Vec<i32> = vec![3,4,5,1,2];
    let expected_ret = 1;
    assert_eq!(array_algorithm::find_min_153(test_nums), expected_ret);
    let test_nums: Vec<i32> = vec![11,13,15,17];
    let expected_ret = 11;
    assert_eq!(array_algorithm::find_min_153(test_nums), expected_ret);
}

#[test]
pub fn test_search_33() {
    let test_nums: Vec<i32> = vec![4,5,6,7,0,1,2];
    assert_eq!(array_algorithm::search_33(test_nums, 0), 4);
    let test_nums: Vec<i32> = vec![4,5,6,7,0,1,2];
    assert_eq!(array_algorithm::search_33(test_nums, 3), -1);
}

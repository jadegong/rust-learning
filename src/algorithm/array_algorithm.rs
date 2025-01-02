/// Leetcode 27 remove element in array
///
/// # Examples
/// ```
/// let mut test_nums: Vec<i32> = vec![0, 1, 2, 2, 3, 0, 4, 2];
/// let val: i32 = 2;
/// let ret_len = array_algorithm::remove_element(&mut test_nums, val);
/// assert_eq!(ret_len, 5);
/// ```
pub fn remove_element_27(nums: &mut Vec<i32>, val: i32) -> i32 {
    let mut index = 0;
    let mut ret_index = 0;
    let nums_len = nums.len();
    while index < nums_len {
        if nums[index] != val {
            nums[ret_index] = nums[index];
            ret_index += 1;
        }
        index +=1 ;
    }
    return ret_index as i32;
}

/// 
/// Leetcode 26 remove duplicates from sorted array
///
pub fn remove_duplicates_26(nums: &mut Vec<i32>) -> i32 {
    if nums.len() == 0 {
        return 0;
    }
    let mut index = 1;
    let mut ret_index = 1;
    let mut current_num = nums[0];
    let nums_len = nums.len();
    while index < nums_len {
        if nums[index] != current_num {
            nums[ret_index] = nums[index];
            current_num = nums[index];
            ret_index += 1;
        }
        index += 1;
    }
    return ret_index as i32;
}

/// 
/// Leetcode 80 remove duplicates from sorted array II
/// An element appears at most twice
///
pub fn remove_duplicates_80(nums: &mut Vec<i32>) -> i32 {
    if nums.len() == 0 {
        return 0;
    }
    let mut index = 1;
    let mut ret_index = 1;
    let mut current_num = nums[0];
    let mut current_num_repeat = 1;
    let nums_len = nums.len();
    while index < nums_len {
        if nums[index] == current_num {
            current_num_repeat += 1;
            // 等于时需要判断重复次数
            if current_num_repeat <= 2 {
                nums[ret_index] = nums[index];
                ret_index += 1;
            }
        } else { // 不等于时直接赋值
            nums[ret_index] = nums[index];
            current_num = nums[index];
            ret_index += 1;
            current_num_repeat = 1;
        }
        index += 1;
    }
    return ret_index as i32;
}

///
/// Leetcode 88 merge sorted arrays
///
pub fn merge_sorted_array_88(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    let mut index = m + n - 1;
    let mut index1 = m - 1;
    let mut index2 = n - 1;
    loop {
        if index1 < 0 {
            nums1[index as usize] = nums2[index2 as usize];
            index2 -= 1;
        } else if index2 < 0 {
            nums1[index as usize] = nums1[index1 as usize];
            index1 -= 1;
        } else {
            if nums1[index1 as usize] >= nums2[index2 as usize] {
                nums1[index as usize] = nums1[index1 as usize];
                index1 -= 1;
            } else {
                nums1[index as usize] = nums2[index2 as usize];
                index2 -= 1;
            }
        }
        if index == 0 {
            break;
        }
        index -= 1;
    }
}

///
/// Leetcode 209
/// Minimum Size Subarray Sum
///
pub fn min_sub_array_len_209(target: i32, nums: Vec<i32>) -> i32 {
    let mut left_index = 0; // current subarray's left edge
    let mut right_index = 0; // current subarray's right edge
    let mut current_sub_len = 1;
    let mut current_subarray_sum = nums[0]; // current subarray's sum
    let mut min_len_ret = std::i32::MAX;
    if current_subarray_sum >= target {
        min_len_ret = 1;
    }
    let nums_len = nums.len();
    while right_index < nums_len {
        if left_index < right_index {
            if current_subarray_sum > target { // Over target, minus left_edge
                left_index += 1;
                current_subarray_sum -= nums[left_index - 1];
                current_sub_len -= 1;
            } else { // Not over target, move right_edge towards
                right_index += 1;
                if right_index > nums_len - 1 {
                    break;
                }
                current_subarray_sum += nums[right_index];
                current_sub_len += 1;
            }
        } else { // edge is the same position, plus right_edge
            right_index += 1;
            if right_index > nums_len - 1 {
                break;
            }
            current_subarray_sum += nums[right_index];
            current_sub_len += 1;
        }
        if current_subarray_sum >= target { // Sum equal or over target
            // Compare current_sub_len and min_len_ret
            min_len_ret = std::cmp::min(current_sub_len, min_len_ret);
        }
        // println!("Current left: {}, right: {}, current_sub_len: {}, current_subarray_sum: {}, min_len_ret: {}", left_index, right_index, current_sub_len, current_subarray_sum, min_len_ret);
    }
    if min_len_ret == std::i32::MAX {
        return 0;
    }
    return min_len_ret;
}


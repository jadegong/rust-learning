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

///
/// Leetcode 239
/// Sliding Window Maximum
/// Time Limit Exceeded
pub fn max_sliding_window_239(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut max_ret: Vec<i32> = vec![];
    let mut max_index_offset = -1; // Record current max num's index offset according to current left index
    let nums_len = nums.len() as i32;
    let mut current_max_num = std::i32::MIN;
    let mut index = 0;
    // find the first max
    while index < k {
        if nums[index as usize] >= current_max_num {
            current_max_num = nums[index as usize];
            max_index_offset = index;
        }
        index += 1;
    }
    max_ret.push(current_max_num);
    let mut left_index = 1;
    while left_index <= nums_len - k {
        if nums[(left_index + k - 1) as usize] >= current_max_num {
            current_max_num = nums[(left_index + k - 1) as usize];
            max_index_offset = k - 1;
        } else {
            if max_index_offset == 0 {
                index = left_index;
                current_max_num = std::i32::MIN;
            } else {
                index = left_index + max_index_offset - 1;
            }
            while index < left_index + k {
                if nums[index as usize] >= current_max_num {
                    current_max_num = nums[index as usize];
                    max_index_offset = index - left_index;
                }
                index += 1;
            }
        }
        max_ret.push(current_max_num);
        left_index += 1;
    }
    return max_ret;
}

///
/// Leetcode 169
/// Majority Element appears more than n/2 times
///
pub fn majority_element_169(nums: Vec<i32>) -> i32 {
    let nums_len = nums.len();
    let mut index = 0;
    let mut nums_count_map: std::collections::HashMap<i32, i32> = std::collections::HashMap::new();
    let mut max_num_count = 0;
    let mut ret_num = 0;
    while index < nums_len {
        if nums_count_map.contains_key(&nums[index]) {
            let old_n = nums_count_map.get(&nums[index]).unwrap();
            if *old_n + 1 > max_num_count {
                max_num_count = *old_n + 1;
                ret_num = nums[index];
            }
            nums_count_map.insert(nums[index], *old_n + 1);
        } else {
            nums_count_map.insert(nums[index], 1);
            if index == 0 {
                ret_num = nums[index];
                max_num_count = 1;
            }
        }
        index += 1;
    }
    return ret_num;
}

///
/// Leetcode 73
/// Set matrix zeroes in place
///
pub fn set_zeroes_73(matrix: &mut Vec<Vec<i32>>) {
    let row_count = matrix.len();
    let column_count = matrix[0].len();
    let mut row_flag_map: std::collections::HashMap<usize, bool> = std::collections::HashMap::new(); // judge current row has zero
    let mut column_flag_map: std::collections::HashMap<usize, bool> = std::collections::HashMap::new(); // judge current column has zero
    let mut row_index = 0;
    let mut column_index = 0;
    // init row_flag_map
    while row_index < row_count {
        row_flag_map.insert(row_index, false);
        row_index += 1;
    }
    // init column_flag_map
    while column_index < column_count {
        column_flag_map.insert(column_index, false);
        column_index += 1;
    }
    row_index = 0;
    while row_index < row_count {
        column_index = 0;
        while column_index < column_count {
            if matrix[row_index][column_index] == 0 {
                row_flag_map.insert(row_index, true);
                column_flag_map.insert(column_index, true);
            }
            column_index += 1;
        }
        row_index += 1;
    }
    // set values
    row_index = 0;
    'row_loop: loop {
        if row_index >= row_count {
            break 'row_loop;
        }
        column_index = 0;
        'column_loop: loop {
            if column_index >= column_count {
                break 'column_loop;
            }
            if *row_flag_map.get(&row_index).unwrap() || *column_flag_map.get(&column_index).unwrap() {
                matrix[row_index][column_index] = 0;
            }
            column_index += 1;
        }
        row_index += 1;
    }
}

/// 
/// Leetcode 48
/// Rotate Image by 90 degrees (clockwise)
/// 1. Fold according the line from topcenter to bottomceter;
/// 2. Fold according the line from bottomleft to topright.
///
pub fn rotate_48(matrix: &mut Vec<Vec<i32>>) {
    let n = matrix.len();
    let mut row_index = 0;
    let mut column_index;
    // 1. Fold according the line from topcenter to bottomceter;
    let mut temp: i32;
    while row_index < n {
        column_index = 0;
        while column_index <= (n - 1) / 2 {
            temp = matrix[row_index][column_index];
            matrix[row_index][column_index] = matrix[row_index][n-1-column_index];
            matrix[row_index][n-1-column_index] = temp;
            column_index += 1;
        }
        row_index += 1;
    }
    // 2. Fold according the line from bottomleft to topright.
    row_index = 0;
    while row_index < n {
        column_index = 0;
        while column_index < (n - 1 - row_index) {
            temp = matrix[row_index][column_index];
            matrix[row_index][column_index] = matrix[n-1-column_index][n-1-row_index];
            matrix[n-1-column_index][n-1-row_index] = temp;
            column_index += 1;
        }
        row_index += 1;
    }
}

///
/// Leetcode 54
///
pub fn spiral_order_54(matrix: Vec<Vec<i32>>) -> Vec<i32> {
    let mut ret_vec: Vec<i32> = vec![];
    let row_count = matrix.len();
    let column_count = matrix[0].len();
    let mut start_row_index = 0;
    let mut start_column_index = 0;
    let mut current_row_count;
    let mut current_column_count;
    while start_row_index < (row_count + 1) / 2 && start_column_index < (column_count + 1) / 2 {
        current_row_count = row_count - start_row_index * 2;
        current_column_count = column_count - start_column_index * 2;
        let mut row_index = start_row_index;
        let mut column_index = start_column_index;
        if current_row_count == 1 { // Only one row
            while column_index < column_count - start_column_index {
                ret_vec.push(matrix[row_index][column_index]);
                column_index += 1;
            }
        } else { // Two or more rows
            if current_column_count == 1 { // Only one column
                while row_index < row_count - start_row_index {
                    ret_vec.push(matrix[row_index][column_index]);
                    row_index += 1;
                }
            } else { // Two or more columns
                // 1.Top row
                while column_index < column_count - start_column_index {
                    ret_vec.push(matrix[row_index][column_index]);
                    column_index += 1;
                }
                column_index -= 1;
                row_index += 1;
                // 2.Right column
                while row_index < row_count - start_row_index - 1 {
                    ret_vec.push(matrix[row_index][column_index]);
                    row_index += 1;
                }
                // row_index += 1;
                // 3.Bottom row
                while column_index >= start_column_index {
                    ret_vec.push(matrix[row_index][column_index]);
                    if column_index == 0 {
                        break;
                    }
                    column_index -= 1;
                }
                if start_column_index > 0 {
                    column_index += 1;
                }
                row_index -= 1;
                // 4.Left column
                while row_index > start_row_index {
                    ret_vec.push(matrix[row_index][column_index]);
                    row_index -= 1;
                }
            }
        }
        start_row_index += 1;
        start_column_index += 1;
    }
    return ret_vec;
}

///
/// Leetcode 1
///
pub fn two_sum_1(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut ret_vec: Vec<i32> = vec![];
    let nums_len = nums.len();
    let mut nums_index_map: std::collections::HashMap<i32, usize> = std::collections::HashMap::new();
    let mut index = 0;
    while index < nums_len {
        if nums_index_map.contains_key(&(target - nums[index])) {
            let map_v = nums_index_map.get(&(target - nums[index])).unwrap();
            ret_vec.push(*map_v as i32);
            ret_vec.push(index as i32);
            break;
        } else {
            nums_index_map.insert(nums[index], index);
        }
        index += 1;
    }
    return ret_vec;
}

///
/// Leetcode 219
///
pub fn contains_nearby_duplicate_219(nums: Vec<i32>, k: i32) -> bool {
    let mut nums_index_map: std::collections::HashMap<i32, usize> = std::collections::HashMap::new();
    let nums_len = nums.len();
    let mut index = 0;
    let mut ret = false;
    while index < nums_len {
        if nums_index_map.contains_key(&nums[index]) {
            let map_v = nums_index_map.get(&nums[index]).unwrap();
            if index - *map_v <= k as usize {
                ret = true;
                break;
            } else { // Do not satisfy, update index of current num
                nums_index_map.insert(nums[index], index);
            }
        } else {
            nums_index_map.insert(nums[index], index);
        }
        index += 1;
    }
    return ret;
}

///
/// Leetcode 228
///
pub fn summary_ranges_228(nums: Vec<i32>) -> Vec<String> {
    let mut ret_vec: Vec<String> = vec![];
    let nums_len = nums.len();
    if nums_len <= 0 {
        return ret_vec;
    }
    let mut index = 1;
    let mut current_start = nums[0];
    let mut current_num = nums[0];
    while index < nums_len {
        if nums[index] > current_num + 1 {
            // should print current_start -> current_num
            let mut current_str: String = current_start.to_string();
            if current_num != current_start {
                current_str = current_str + "->" + &(current_num.to_string());
            }
            ret_vec.push(current_str);
            current_start = nums[index]; // New start
        }
        current_num = nums[index];
        index += 1;
    }
    // Add lasf
    let mut current_str: String = current_start.to_string();
    if current_num != current_start {
        current_str = current_str + "->" + &(current_num.to_string());
    }
    ret_vec.push(current_str);
    return ret_vec;
}

///
/// Leetcode 57
///
pub fn insert_57(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
    let intervals_len = intervals.len();
    let mut ret_vec: Vec<Vec<i32>> = vec![];
    if intervals_len == 0 {
        ret_vec.push(vec![new_interval[0], new_interval[1]]);
        return ret_vec;
    }
    let mut index = 0;
    let mut new_start_index = intervals_len;
    let mut new_start_in = false;
    let mut new_end_index= intervals_len;
    let mut new_end_in = false;
    while index < intervals_len {
        if new_start_index == intervals_len {
            if new_interval[0] <= intervals[index][1] {
                new_start_index = index;
                if new_interval[0] >= intervals[index][0] {
                    new_start_in = true;
                }
            }
        }
        if new_end_index == intervals_len {
            if new_interval[1] <= intervals[index][1] {
                new_end_index = index;
                if new_interval[1] >= intervals[index][0] {
                    new_end_in = true;
                }
            }
        }
        index += 1;
    }
    index = 0;
    println!("new_start_index: {}, new_start_in: {}, new_end_index: {}, new_end_in: {}", new_start_index, new_start_in, new_end_index, new_end_in);
    let mut current_start: i32 = new_interval[0];
    let mut current_end: i32 = new_interval[1];
    while index < intervals_len {
        if index < new_start_index {
            ret_vec.push(vec![intervals[index][0], intervals[index][1]]);
        } else if index == new_start_index {
            if !new_start_in {
                current_start = new_interval[0];
            } else {
                current_start = intervals[index][0];
            }
            if index == new_end_index {
                if !new_end_in {
                    current_end = new_interval[1];
                    ret_vec.push(vec![current_start, current_end]);
                    ret_vec.push(vec![intervals[index][0], intervals[index][1]]);
                } else {
                    current_end = intervals[index][1];
                    ret_vec.push(vec![current_start, current_end]);
                }
            }
        } else if index == new_end_index {
            if !new_end_in {
                current_end = new_interval[1];
                ret_vec.push(vec![current_start, current_end]);
                ret_vec.push(vec![intervals[index][0], intervals[index][1]]);
            } else {
                current_end = intervals[index][1];
                ret_vec.push(vec![current_start, current_end]);
            }
        } else if index > new_end_index {
            ret_vec.push(vec![intervals[index][0], intervals[index][1]]);
        }
        index += 1;
    }
    if new_end_index == intervals_len {
        ret_vec.push(vec![current_start, new_interval[1]]);
    }
    return ret_vec;
}


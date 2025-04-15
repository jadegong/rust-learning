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
    let mut current_end;
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

///
/// Leetcode 35
/// Search Insert Position
///
pub fn search_insert_53(nums: Vec<i32>, target: i32) -> i32 {
    let mut left_index = 0;
    let mut right_index = nums.len() - 1;
    let mut target_index = usize::MAX;
    let mut middle_index;
    while left_index < right_index {
        middle_index = (left_index + right_index) / 2;
        if nums[middle_index] == target {
            target_index = middle_index;
            break;
        } else if nums[middle_index] > target {
            if middle_index == left_index {
                right_index = middle_index;
                break;
            }
            right_index = middle_index - 1;
        } else {
            left_index = middle_index + 1;
        }
    }
    if target_index == usize::MAX {
        if nums[right_index] >= target {
            target_index = right_index;
        } else {
            target_index = right_index + 1;
        }
    }
    target_index as i32
}

///
/// Leetcode 153
/// Find Minimum in Rotated Sorted Array
///
pub fn find_min_153(nums: Vec<i32>) -> i32 {
    let mut left_index = 0;
    let mut right_index = nums.len() - 1;
    let mut middle_index: usize;
    let mut ret = nums[left_index];
    while left_index < right_index {
        middle_index = (left_index + right_index) / 2;
        if nums[middle_index] > nums[left_index] {
            left_index = middle_index;
        } else if nums[middle_index] < nums[left_index] {
            right_index = middle_index;
            ret = nums[middle_index];
        } else { // while middle_index = left_index, means right_index = left_index + 1;
            break;
        }
    }
    std::cmp::min(nums[right_index], ret)
}

/// 
/// Leetcode 33
/// Search in Rotated Sorted Array
///
pub fn search_33(nums: Vec<i32>, target: i32) -> i32 {
    let mut left_index = 0;
    let mut right_index = nums.len() - 1;
    let mut middle_index;
    let mut target_index = usize::MAX;
    while left_index < right_index {
        middle_index = (left_index + right_index) / 2;
        if nums[middle_index] == target {
            target_index = middle_index;
            break;
        }
        // While target in left part
        else if nums[left_index] > nums[middle_index] && (nums[left_index] <= target || nums[middle_index] > target) {
            if middle_index == left_index {
                right_index = middle_index;
                break;
            }
            right_index = middle_index - 1;
        }
        // Another posibility of taret in left part
        else if nums[left_index] < nums[middle_index] && (nums[left_index] <= target && nums[middle_index] > target) {
            if middle_index == left_index {
                right_index = middle_index;
                break;
            }
            right_index = middle_index - 1;
        } else {
            left_index = middle_index + 1;
        }
    }
    if target_index == usize::MAX {
        if nums[right_index] == target {
            return right_index as i32;
        }
        return -1;
    }
    target_index as i32
}

/// 
/// Leetcode 2161
/// Partition Array According to Given Pivot
///
pub fn pivot_array(nums: Vec<i32>, pivot: i32) -> Vec<i32> {
    let mut left_arr: Vec<i32> = vec![];
    let mut middle_arr: Vec<i32> = vec![];
    let mut right_arr: Vec<i32> = vec![];
    let mut index = 0;
    let nums_len = nums.len();
    while index < nums_len {
        if nums[index] < pivot {
            left_arr.push(nums[index]);
        } else if nums[index] == pivot {
            middle_arr.push(nums[index]);
        } else {
            right_arr.push(nums[index]);
        }
        index += 1;
    }
    left_arr.append(&mut middle_arr);
    left_arr.append(&mut right_arr);
    left_arr
}

/// 
/// Leetcode 2364
/// Count Number of Bad Pairs
/// j - i != nums[j] - nums[i] => nums[i] - i != nums[j] - j
///
pub fn count_bad_pairs_2364(nums: Vec<i32>) -> i64 {
    let mut count_map: std::collections::HashMap<i32, i64> = std::collections::HashMap::new();
    let nums_len = nums.len();
    let mut index = 0;
    let mut normal_count: i64 = 0;
    while index < nums_len {
        if count_map.contains_key(&(nums[index] - index as i32)) {
            let old_n = count_map.get(&(nums[index] - index as i32)).unwrap();
            normal_count += *old_n;
            count_map.insert(nums[index] - index as i32, *old_n + 1);
        } else {
            count_map.insert(nums[index] - index as i32, 1);
        }
        index += 1;
    }
    // let map_keys: Vec<&i32> = count_map.keys().collect();
    // index = 0;
    // let count_len = map_keys.len();
    // let mut normal_count: i64 = 0;
    // while index < count_len {
        // let count_n = count_map.get(map_keys[index]).unwrap();
        // normal_count += *count_n * (*count_n - 1) / 2;
        // index += 1;
    // }
    let total: i64 = (nums_len * (nums_len - 1) / 2) as i64;
    total - normal_count
}

/// 
/// Leetcode 2342
/// Max Sum of a Pair With Equal Sum of Digits
///
pub fn maximum_sum_2342(nums: Vec<i32>) -> i32 {
    // Biggest two numbers for every digits sum
    let mut big_two_map: std::collections::HashMap<i32, Vec<i32>> = std::collections::HashMap::new();
    let mut digits_sum_repeat: std::collections::HashMap<i32, i32> = std::collections::HashMap::new();
    let nums_len = nums.len();
    let mut index = 0;
    let mut origin_num;
    let mut current_digits_sum;
    let mut ret = -1;
    while index < nums_len {
        origin_num = nums[index];
        current_digits_sum = 0;
        while origin_num > 0 {
            current_digits_sum += origin_num % 10;
            origin_num /= 10;
        }
        if digits_sum_repeat.contains_key(&current_digits_sum) {
            let old_n = digits_sum_repeat.get(&current_digits_sum).unwrap();
            digits_sum_repeat.insert(current_digits_sum, *old_n + 1);
            let old_big_two = big_two_map.get(&current_digits_sum).unwrap().clone();
            if nums[index] >= old_big_two[0] && nums[index] <= old_big_two[1] {
                big_two_map.insert(current_digits_sum, vec![nums[index], old_big_two[1]]);
                ret = std::cmp::max(ret, nums[index] + old_big_two[1]);
            } else if nums[index] > old_big_two[1] {
                big_two_map.insert(current_digits_sum, vec![old_big_two[1], nums[index]]);
                ret = std::cmp::max(ret, nums[index] + old_big_two[1]);
            }
        } else {
            digits_sum_repeat.insert(current_digits_sum, 1);
            big_two_map.insert(current_digits_sum, vec![i32::MIN, nums[index]]);
        }
        index += 1;
    }
    ret
}

/// 
/// Leetcode 3160
/// Find the Numbers of Distinct Colors Among The Balls
///
pub fn query_results_3160(_limit: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
    let mut ret: Vec<i32> = vec![];
    // count map for every color
    let mut color_count_map: std::collections::HashMap<i32, i32> = std::collections::HashMap::new();
    // color map for every ball
    let mut ball_color_map: std::collections::HashMap<i32, i32> = std::collections::HashMap::new();
    let mut current_query_count = 0;
    let queries_len = queries.len();
    let mut index = 0;
    while index < queries_len {
        let current_color = queries[index][1];
        // current ball has color
        let mut minus_prev = false;
        let add_current;
        if ball_color_map.contains_key(&queries[index][0]) {
            let prev_color = ball_color_map.get(&queries[index][0]).unwrap();
            if color_count_map.contains_key(prev_color) {
                let prev_color_count = color_count_map.get(prev_color).unwrap();
                if *prev_color_count == 1 {
                    minus_prev = true;
                }
                if *prev_color_count > 0 {
                    color_count_map.insert(*prev_color, *prev_color_count - 1);
                }
            }
            if color_count_map.contains_key(&current_color) {
                let current_color_count = color_count_map.get(&current_color).unwrap();
                if *current_color_count == 0 {
                    add_current = true;
                } else {
                    add_current = false;
                }
                color_count_map.insert(current_color, *current_color_count + 1);
            } else {
                add_current = true;
                color_count_map.insert(current_color, 1);
            }
        } else {
            if color_count_map.contains_key(&current_color) {
                let current_color_count = color_count_map.get(&current_color).unwrap();
                if *current_color_count == 0 {
                    add_current = true;
                } else {
                    add_current = false;
                }
                color_count_map.insert(current_color, *current_color_count + 1);
            } else {
                add_current = true;
                color_count_map.insert(current_color, 1);
            }
        }
        ball_color_map.insert(queries[index][0], queries[index][1]);
        if minus_prev {
            current_query_count -= 1;
        }
        if add_current {
            current_query_count += 1;
        }
        ret.push(current_query_count);
        index += 1;
    }
    ret
}

/// 
/// Leetcode 2657
/// Find the Prefix Common Array of Two Arrays
///
pub fn find_the_prefix_common_array_2657(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
    let mut ret: Vec<i32> = vec![];
    if a.len() != b.len() {
        return ret;
    }
    let n = a.len();
    // A number whether appears
    let mut appear_map: std::collections::HashMap<i32, bool> = std::collections::HashMap::new();
    let mut index = 0;
    let mut current_count = 0;
    while index < n {
        if appear_map.contains_key(&a[index]) {
            current_count += 1;
        } else {
            appear_map.insert(a[index], true);
        }
        if appear_map.contains_key(&b[index]) {
            current_count += 1;
        } else {
            appear_map.insert(b[index], true);
        }
        ret.push(current_count);
        index += 1;
    }
    ret
}

/// 
/// Leetcode 2683
/// Neighboring Bitwise XOR
///
pub fn does_valid_array_exist_2683(derived: Vec<i32>) -> bool {
    let derived_len = derived.len();
    if derived_len == 0 {
        return true;
    }
    let mut ret = true; // If current the same with first
    let mut index = 0;
    while index < derived_len {
        if derived[index] == 1 {
            if ret {
                ret = false;
            } else {
                ret = true;
            }
        }
        index += 1;
    }
    ret
}

/// 
/// Leetcode 2460
/// Apply Operations to an Array
///
pub fn apply_operations_2460(nums: Vec<i32>) -> Vec<i32> {
    let mut ret: Vec<i32> = vec![];
    let nums_len = nums.len();
    let mut index = 0;
    let mut prev_num = nums[index];
    let mut ret_index = 0;
    while index < nums_len - 1 {
        if prev_num == nums[index + 1] {
            if prev_num != 0 {
                ret.push(prev_num + nums[index + 1]);
                ret_index += 1;
            }
            prev_num = 0;
        } else {
            if prev_num != 0 {
                ret.push(prev_num);
                ret_index += 1;
            }
            prev_num = nums[index + 1];
        }
        index += 1;
    }
    // last element
    if prev_num != 0 {
        ret.push(prev_num);
        ret_index += 1;
    }
    while ret_index < nums_len {
        ret.push(0);
        ret_index += 1;
    }
    ret
}

/// 
/// Leetcode 2570
/// Merge Two 2D Arrays by Summing Values
///
pub fn merge_arrays_2570(nums1: Vec<Vec<i32>>, nums2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let nums1_len = nums1.len();
    let nums2_len = nums2.len();
    let mut ret: Vec<Vec<i32>> = vec![];
    let mut index1 = 0;
    let mut index2 = 0;
    while index1 < nums1_len && index2 < nums2_len {
        if nums1[index1][0] < nums2[index2][0] {
            ret.push(nums1[index1].clone());
            index1 += 1;
        } else if nums1[index1][0] == nums2[index2][0] {
            ret.push(vec![nums1[index1][0], nums1[index1][1] + nums2[index2][1]]);
            index1 += 1;
            index2 += 1;
        } else {
            ret.push(nums2[index2].clone());
            index2 += 1;
        }
    }
    if index1 < nums1_len {
        while index1 < nums1_len {
            ret.push(nums1[index1].clone());
            index1 += 1;
        }
    }
    if index2 < nums2_len {
        while index2 < nums2_len {
            ret.push(nums2[index2].clone());
            index2 += 1;
        }
    }
    ret
}

/// 
/// Leetcode 2965
/// Find Missing and Repeated Values
///
pub fn find_missing_and_repeated_values_2965(grid: Vec<Vec<i32>>) -> Vec<i32> {
    let n = grid.len();
    let mut count_map: std::collections::HashMap<i32, bool> = std::collections::HashMap::new();
    let mut row_index = 0;
    let mut col_index;
    let mut repeated_num = 0;
    while row_index < n {
        col_index = 0;
        while col_index < n {
            if count_map.contains_key(&grid[row_index][col_index]) {
                repeated_num = grid[row_index][col_index];
            } else {
                count_map.insert(grid[row_index][col_index], true);
            }
            col_index += 1;
        }
        row_index += 1;
    }
    let mut missing_num = 0;
    row_index = 0;
    while row_index < n * n {
        if !count_map.contains_key(&((row_index + 1) as i32)) {
            missing_num = (row_index + 1) as i32;
            break;
        }
        row_index += 1;
    }
    vec![repeated_num, missing_num]
}

/// 
/// Leetcode 56
/// Merge Intervals
///
pub fn merge_56(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut new_intervals: Vec<Vec<i32>> = intervals;
    new_intervals.sort_by(|a, b| a[0].cmp(&b[0]));
    let mut ret: Vec<Vec<i32>> = vec![];
    let n_len = new_intervals.len();
    let mut prev_left = new_intervals[0][0];
    let mut prev_right = new_intervals[0][1];
    let mut index = 1;
    while index < n_len {
        if new_intervals[index][0] > prev_right {
            ret.push(vec![prev_left, prev_right]);
            prev_left = new_intervals[index][0];
            prev_right = new_intervals[index][1];
        } else {
            prev_right = std::cmp::max(prev_right, new_intervals[index][1]);
        }
        index += 1;
    }
    ret.push(vec![prev_left, prev_right]);
    ret
}

/// 
/// Leetcode 75
/// Sort Colors
///
pub fn sort_colors_75(nums: &mut Vec<i32>) {
    let nums_len = nums.len();
    let mut index = 0;
    let mut zero_len = 0; // The next index of end of 0
    let mut one_len = 0; // The next index of end of 1
    let mut temp: i32;
    while index < nums_len {
        if nums[index] == 0 {
            // 1. Swap nums[index] and nums[zero_len];
            // 2. Swap nums[index] and nums[one_len];
            temp = nums[index];
            nums[index] = nums[zero_len];
            nums[zero_len] = temp;
            if one_len == zero_len { // No ones
                zero_len += 1;
                one_len += 1;
            } else {
                zero_len += 1;
                temp = nums[index];
                nums[index] = nums[one_len];
                nums[one_len] = temp;
                one_len += 1;
            }
        } else if nums[index] == 1 {
            // 1. Swap nums[index] and nums[one_len];
            temp = nums[index];
            nums[index] = nums[one_len];
            nums[one_len] = temp;
            one_len += 1;
        }
        index += 1;
    }
}


/// 
/// Leetcode 128
/// Longest Consecutive Sequence
///
pub fn longest_consecutive_128(nums: Vec<i32>) -> i32 {
    // Max top heap
    let mut max_heap: std::collections::BinaryHeap<i32> = std::collections::BinaryHeap::new();
    let nums_len = nums.len();
    let mut index = 0;
    while index < nums_len {
        max_heap.push(nums[index]);
        index += 1;
    }
    let mut current_vec: Vec<i32> = vec![];
    let mut max_len = 0;
    let mut current_len = 0;
    if nums_len == 0 {
        return 0;
    }
    let mut max_heap_top = max_heap.pop().unwrap();
    current_vec.push(max_heap_top);
    current_len += 1;
    while !max_heap.is_empty() {
        max_heap_top = max_heap.pop().unwrap();
        if max_heap_top == current_vec[current_len - 1] - 1 {
            current_vec.push(max_heap_top);
            current_len += 1;
        } else if max_heap_top == current_vec[current_len - 1] {
        }
        else {
            if current_len > max_len {
                max_len = current_len;
            }
            current_vec = vec![max_heap_top];
            current_len = 1;
        }
    }
    if current_len > max_len {
        max_len = current_len;
    }
    max_len as i32
}

/// 
/// Leetcode 167
/// Two Sum II - Input Array Is Sorted
///
pub fn two_sum_167(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    let nums_len = numbers.len();
    let mut middle_index = 0;
    let mut left_index = 0;
    let mut right_index = nums_len - 1;
    while left_index < right_index {
        middle_index = (left_index + right_index) / 2;
        if numbers[middle_index] * 2 == target {
            break;
        } else if numbers[middle_index] * 2 > target {
            right_index = middle_index;
        } else {
            if left_index == middle_index {
                middle_index = left_index + 1;
                break;
            }
            left_index = middle_index;
        }
    }
    if middle_index < nums_len - 1 && numbers[middle_index + 1] == numbers[middle_index] {
        middle_index += 1;
    }
    let mut res: Vec<i32> = vec![];
    if middle_index == 0 {
        return res;
    }
    left_index = middle_index;
    right_index = middle_index + 1;
    while left_index > 0 && right_index <= nums_len {
        if numbers[left_index - 1] + numbers[right_index - 1] == target {
            res.push(left_index as i32);
            res.push(right_index as i32);
            break;
        } else if numbers[left_index - 1] + numbers[right_index - 1] > target {
            left_index -= 1;
        } else {
            right_index += 1;
        }
    }
    res
}

/// 
/// Leetcode 217
/// Contains Duplicate
///
pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut ret = false;
    let nums_len = nums.len();
    let mut nums_map: std::collections::HashMap<i32, bool> = std::collections::HashMap::new();
    let mut index = 0;
    while index < nums_len {
        if nums_map.contains_key(&nums[index]) {
            ret = true;
            break;
        } else {
            nums_map.insert(nums[index], true);
        }
        index += 1;
    }
    ret
}

/// 
/// Leetcode 283
/// Move Zeroes
///
pub fn move_zeroes_283(nums: &mut Vec<i32>) {
    let mut none_zero_len = 0;
    let mut current_index = 0;
    let nums_len = nums.len();
    while current_index < nums_len {
        if nums[current_index] != 0 {
            // Swap current_index and none_zero_len
            let temp = nums[current_index];
            nums[current_index] = nums[none_zero_len];
            nums[none_zero_len] = temp;
            none_zero_len += 1;
        }
        current_index += 1;
    }
}

///
/// Leetcode 287
/// Find the Duplicate Number
/// Use slow fast pointers
///
pub fn find_duplicate_287(nums: Vec<i32>) -> i32 {
    let nums_len = nums.len();
    if nums_len < 2 {
        return nums[0];
    }
    let mut slow = nums[0];
    let mut fast = nums[0];
    // find the meeting point
    loop {
        slow = nums[slow as usize];
        fast = nums[nums[fast as usize] as usize];
        if slow == fast {
            break;
        }
    }
    slow = nums[0];
    while slow != fast {
        slow = nums[slow as usize];
        fast = nums[fast as usize];
    }
    slow
}

/// 
/// Leetcode 229
/// Majority Element II
///
pub fn majority_element_229(nums: Vec<i32>) -> Vec<i32> {
    let mut nums_map: std::collections::HashMap<i32, i32> = std::collections::HashMap::new();
    let nums_len = nums.len();
    let mut index = 0;
    while index < nums_len {
        if nums_map.contains_key(&nums[index]) {
            let old_n = nums_map.get(&nums[index]).unwrap();
            nums_map.insert(nums[index], *old_n + 1);
        } else {
            nums_map.insert(nums[index], 1);
        }
        index += 1;
    }
    index = 0;
    let count_vec: Vec<&i32> = nums_map.keys().collect();
    let count_vec_len = count_vec.len();
    let mut ret: Vec<i32> = vec![];
    while index < count_vec_len {
        let old_n = nums_map.get(count_vec[index]).unwrap();
        if *old_n > (nums_len / 3) as i32 {
            ret.push(*count_vec[index]);
        }
        index += 1;
    }
    ret
}

/// 
/// Leetcode 240
/// Search a 2D Matrix II
///
pub fn search_matrix_240(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    fn inner_search_matrix_col(matrix: &Vec<Vec<i32>>, start_col: usize, end_col: usize, row: usize, target: i32, start: bool) -> usize {
        // binary search
        let mut left = start_col;
        let mut right = end_col;
        let mut middle: usize;
        while left < right {
            middle = (left + right) / 2;
            if matrix[row][middle] > target {
                right = middle;
            } else if matrix[row][middle] == target {
                left = middle;
                right = middle;
                break;
            } else {
                left = middle;
                if left == right - 1 {
                    if matrix[row][right] == target {
                        left = right;
                    }
                    break;
                }
            }
        }
        if start {
            return right;
        } else {
            return left;
        }
    }
    fn inner_search_matrix_row(matrix: &Vec<Vec<i32>>, start_row: usize, end_row: usize, col: usize, target: i32, start: bool) -> usize {
        // binary search
        let mut left = start_row;
        let mut right = end_row;
        let mut middle: usize;
        while left < right {
            middle = (left + right) / 2;
            if matrix[middle][col] > target {
                right = middle;
            } else if matrix[middle][col] == target {
                left = middle;
                right = middle;
                break;
            } else {
                left = middle;
                if left == right - 1 {
                    if matrix[right][col] == target {
                        left = right;
                    }
                    break;
                }
            }
        }
        if start {
            return right;
        } else {
            return left;
        }
    }
    let rows =  matrix.len();
    let cols = matrix[0].len();
    let mut row_start: usize = 0;
    let mut row_end: usize = rows - 1;
    let mut col_start: usize = 0;
    let mut col_end: usize = cols - 1;
    loop {
        if row_start != row_end {
            row_start = inner_search_matrix_row(&matrix, row_start, row_end, col_end, target, true);
            row_end = inner_search_matrix_row(&matrix, row_start, row_end, col_start, target, false);
        }
        if col_start != col_end {
            col_start = inner_search_matrix_col(&matrix, col_start, col_end, row_end, target, true);
            col_end = inner_search_matrix_col(&matrix, col_start, col_end, row_start, target, false);
        }
        if row_start == row_end && col_start == col_end {
            break;
        }
        if matrix[row_start][col_end] == target {
            col_start = col_end;
            break;
        }
    }
    if matrix[row_start][col_start] == target {
        return true;
    }
    return false;
}
// Second method, Start from top right corner, if current bigger than target, move to prev column,
// if current smaller than target, move to next row
pub fn search_matrix_240_2(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    let rows = matrix.len();
    let cols = matrix[0].len();
    let mut row_index = 0;
    let mut col_index = cols - 1;
    loop {
        if matrix[row_index][col_index] == target {
            break;
        } else if matrix[row_index][col_index] > target {
            if col_index == 0 {
                return false;
            }
            col_index -= 1;
        } else {
            if row_index == rows - 1 {
                return false;
            }
            row_index += 1;
        }
    }
    return true;
}

/// 
/// Leetcode 74
/// Search a 2D Matrix
///
pub fn search_matrix_74(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    let rows = matrix.len();
    let cols = matrix[0].len();
    let mut row_index = 0;
    let mut col_index = cols - 1;
    loop {
        if matrix[row_index][col_index] == target {
            break;
        } else if matrix[row_index][col_index] > target {
            if col_index == 0 {
                return false;
            }
            col_index -= 1;
        } else {
            if row_index == rows - 1 {
                return false;
            }
            row_index += 1;
        }
    }
    return true;
}

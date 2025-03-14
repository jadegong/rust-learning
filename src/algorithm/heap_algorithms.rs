/// 
/// Leetcode 215
/// Kth Largest Element in an Array
///
pub fn find_kth_largest_215(nums: Vec<i32>, k: i32) -> i32 {
    let mut min_heap: std::collections::BinaryHeap<std::cmp::Reverse<i32>> = std::collections::BinaryHeap::new();
    let nums_len = nums.len();
    if k <= 0 || k > nums_len as i32 {
        return i32::MIN;
    }
    let mut index: usize = 0;
    while (index as i32) < k {
        min_heap.push(std::cmp::Reverse(nums[index]));
        index += 1;
    }
    let mut heap_top = min_heap.peek().unwrap().0;
    while index < nums_len {
        if nums[index] > heap_top {
            min_heap.pop(); // Remove heap top
            min_heap.push(std::cmp::Reverse(nums[index]));
            heap_top = min_heap.peek().unwrap().0;
        }
        index += 1;
    }
    heap_top
}

///
/// Leetcode 239
/// Sliding Window Maximum
/// 
pub fn max_sliding_window_239(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let nums_len = nums.len();
    // Max top heap
    let mut right_max_heap: std::collections::BinaryHeap<i32> = std::collections::BinaryHeap::new();
    let mut left_max_heap: std::collections::BinaryHeap<i32> = std::collections::BinaryHeap::new();
    let mut index: usize = 0;
    while (index as i32) < k {
        right_max_heap.push(nums[index]);
        index += 1;
    }
    let mut left_max_heap_top: i32;
    let mut right_max_heap_top = *right_max_heap.peek().unwrap();
    let mut ret: Vec<i32> = vec![];
    ret.push(right_max_heap_top);
    while index < nums_len {
        // Remove biggest left is top
        if nums[index - k as usize] == right_max_heap_top {
            right_max_heap.pop();
            right_max_heap.push(nums[index]);
        } else {
            right_max_heap.push(nums[index]);
            left_max_heap.push(nums[index - k as usize]);
        }
        right_max_heap_top = *right_max_heap.peek().unwrap();
        if !left_max_heap.is_empty() {
            left_max_heap_top = *left_max_heap.peek().unwrap();
            while !left_max_heap.is_empty() && left_max_heap_top == right_max_heap_top {
                right_max_heap.pop();
                left_max_heap.pop();
                right_max_heap_top = *right_max_heap.peek().unwrap();
                if !left_max_heap.is_empty() {
                    left_max_heap_top = *left_max_heap.peek().unwrap();
                }
            }
        }
        ret.push(right_max_heap_top);
        index += 1;
    }
    ret
}

/// 
/// Leetcode 347
/// Top K Frequent Elements
///
pub fn top_k_frequent_347(nums: Vec<i32>, k: i32) -> Vec<i32> {
    // Count of every num
    let mut count_map: std::collections::HashMap<i32, i32> = std::collections::HashMap::new();
    // Min top heap
    let mut min_heap: std::collections::BinaryHeap<std::cmp::Reverse<i32>> = std::collections::BinaryHeap::new();
    let nums_len = nums.len();
    let mut index = 0;
    let mut current_count;
    let mut min_heap_top: i32= i32::MIN;
    while index < nums_len {
        if count_map.contains_key(&nums[index]) {
            let old_n = count_map.get(&nums[index]).unwrap();
            current_count = *old_n + 1;
        } else {
            current_count = 1;
        }
        count_map.insert(nums[index], current_count);
        index += 1;
    }
    let mut ret: Vec<i32> = vec![];
    let map_keys: Vec<&i32> = count_map.keys().collect();
    let map_len = map_keys.len();
    index = 0;
    while index < map_len {
        current_count = *count_map.get(map_keys[index]).unwrap();
        if index < k as usize {
            min_heap.push(std::cmp::Reverse(current_count));
            min_heap_top = min_heap.peek().unwrap().0;
        } else {
            if current_count > min_heap_top {
                min_heap.pop();
                min_heap.push(std::cmp::Reverse(current_count));
                min_heap_top = min_heap.peek().unwrap().0;
            }
        }
        index += 1;
    }
    index = 0;
    while index < map_len {
        current_count = *count_map.get(map_keys[index]).unwrap();
        if current_count >= min_heap_top {
            ret.push(*map_keys[index]);
        }
        index += 1;
    }
    ret
}

/// 
/// Leetcode 506
/// Relative Ranks
///
pub fn find_relative_ranks_506(score: Vec<i32>) -> Vec<String> {
    let score_len = score.len();
    let mut index_map: std::collections::HashMap<i32, usize> = std::collections::HashMap::new();
    let mut max_heap: std::collections::BinaryHeap<i32> = std::collections::BinaryHeap::new();
    let mut index = 0;
    while index < score_len {
        index_map.insert(score[index], index);
        max_heap.push(score[index]);
        index += 1;
    }
    let mut ret: Vec<String> = vec![String::from(""); score_len];
    let mut max_heap_top: i32;
    index = 0;
    while index < score_len {
        max_heap_top = max_heap.pop().unwrap();
        let current_index = *index_map.get(&max_heap_top).unwrap();
        if index == 0 {
            ret[current_index] = String::from("Gold Medal");
        } else if index == 1 {
            ret[current_index] = String::from("Silver Medal");
        } else if index == 2 {
            ret[current_index] = String::from("Bronze Medal");
        } else {
            ret[current_index] = (index + 1).to_string();
        }
        index += 1;
    }
    ret
}

/// 
/// Leetcode 621
/// Task Scheduler
///
pub fn least_interval_621(tasks: Vec<char>, n: i32) -> i32 {
    let mut count_map: std::collections::HashMap<char, i32> = std::collections::HashMap::new();
    let mut max_heap: std::collections::BinaryHeap<i32> = std::collections::BinaryHeap::new();
    let tasks_len = tasks.len();
    let mut index = 0;
    let mut current_count: i32;
    while index < tasks_len {
        if count_map.contains_key(&tasks[index]) {
            let old_n = count_map.get(&tasks[index]).unwrap();
            current_count = *old_n + 1;
        } else {
            current_count = 1;
        }
        count_map.insert(tasks[index], current_count);
        index += 1;
    }
    let map_values: Vec<&i32> = count_map.values().collect();
    let map_len = map_values.len();
    index = 0;
    while index < map_len {
        max_heap.push(*map_values[index]);
        index += 1;
    }
    let max_heap_top = max_heap.pop().unwrap();
    let mut ret = max_heap_top;
    index = 1;
    let mut current_max_heap_top: i32;
    let mut need_before_n: i32 = 0;
    let mut leave_after_n: i32 = 0;
    while index < map_len {
        current_max_heap_top = max_heap.pop().unwrap();
        if index <= n as usize {
            ret += current_max_heap_top;
            if current_max_heap_top < max_heap_top {
                need_before_n += max_heap_top - 1 - current_max_heap_top;
            }
        } else {
            // ret += current_max_heap_top;
            leave_after_n += current_max_heap_top;
        }
        index += 1;
    }
    if need_before_n >= leave_after_n {
        ret += need_before_n;
    } else {
        ret += leave_after_n;
    }
    while index <= n as usize {
        ret += max_heap_top - 1;
        index += 1;
    }
    ret
}

///
/// Leetcode 55 jump game
///You are given an integer array nums. You are initially positioned at the array's first index, and each element in the array represents your maximum jump length at that position.
/// Return true if you can reach the last index, or false otherwise.
///
pub fn can_jump_55(nums: Vec<i32>) -> bool {
    let mut index: usize = 0;
    let mut max_arrive: usize = 0;
    let nums_len = nums.len();
    let mut current_num: usize;
    while index < nums_len {
        if max_arrive < index { // can't arrive current index
            return false;
        }
        current_num = nums[index] as usize;
        if index + current_num > max_arrive {
            max_arrive = index + current_num;
        }
        if max_arrive >= nums_len - 1 { // Can arrive last
            return true;
        }
        index += 1;
    }
    return false;
}

///
/// Leetcode 55 jump game
/// Minimum steps to jump from 0 to len-1
/// greedy
///
pub fn jump_game_45(nums: Vec<i32>) -> i32 {
    let mut steps = 0;
    let mut max_arrive = 0; // Current max arrive index
    let mut jump_index = 0; // Last step jump index
    let mut index: usize = 0;
    let nums_len = nums.len();
    while index < nums_len {
        if jump_index as usize >= nums_len - 1 {
            break;
        }
        if index as i32 + nums[index] > max_arrive {
            max_arrive = (index as i32) + nums[index];
        }
        if index as i32 == jump_index { // when index is last jump index
            steps += 1;
            jump_index = max_arrive;
            // jump one step to current max_arrive
            // even current num is 0, I can still jump from previous
            // indexes to max_arrive with the same steps
        }
        index += 1;
    }
    return steps;
}

/// 
/// Leetcode 135 Candy
///
pub fn candy_135(ratings: Vec<i32>) -> i32 {
    let mut nums_left: Vec<i32> = vec![]; // Count from left side
    let mut nums_right: Vec<i32> = vec![]; // Count from right side
    let nums_len = ratings.len();
    let mut index = 0;
    while index < nums_len {
        nums_left.push(0);
        nums_right.push(0);
        index += 1;
    }
    index = 0;
    while index < nums_len {
        if index == 0 {
            nums_left[index] = 1;
            nums_right[nums_len - 1 - index] = 1;
        } else {
            // Calc nums_left
            if ratings[index] > ratings[index - 1] {
                nums_left[index] = nums_left[index - 1] + 1;
            } else {
                nums_left[index] = 1;
            }
            // Calc nums_right
            if ratings[nums_len - 1 - index] > ratings[nums_len - index] {
                nums_right[nums_len - 1 - index] = nums_right[nums_len - index] + 1;
            } else {
                nums_right[nums_len - 1 - index] = 1;
            }
        }
        index += 1;
    }
    index = 0;
    let mut total = 0;
    while index < nums_len {
        if nums_left[index] > nums_right[index] {
            total += nums_left[index];
        } else {
            total += nums_right[index];
        }
        index += 1;
    }
    return total;
}

///
/// Leetcode 122 
/// Best Time to Buy and Sell Stock II
///
pub fn max_profit_122(prices: Vec<i32>) -> i32 {
    let mut total = 0;
    let mut buy_price = prices[0];
    let date_num = prices.len();
    let mut index = 1;
    while index < date_num {
        if prices[index] > buy_price {
            total += prices[index] - buy_price;
        }
        buy_price = prices[index];
        index += 1;
    }
    return total;
}

/// 
/// Leetcode 2938
/// Separate Black and White Balls
/// Find '0' from left to right, and move to the index of first '1', after this operation, the
/// index of first '1' will add one
pub fn minimum_steps_2938(s: String) -> i64 {
    let s_len = s.len();
    let s_chars: Vec<char> = s.chars().collect();
    let mut dest_index = 0; // Move current to the dest index
    let mut current_index = 0;
    let mut found_one = false;
    let mut ret = 0;
    while current_index < s_len {
        if found_one {
            if s_chars[current_index] == '0' {
                // Move current to dest, dest plus one
                ret += current_index - dest_index;
                dest_index += 1;
            }
        } else {
            if s_chars[current_index] == '1' {
                dest_index = current_index;
                found_one = true;
            }
        }
        current_index += 1;
    }
    ret as i64
}

/// 
/// Leetcode 670
/// Maximum Swap
/// 1.From right to left, find all increasing nums vec, 
///   and record indexes vec(indexes is increasing too); O(log n)
/// 2.From left to right, try to find a num smaller than 1's num, and index before it; O(log n)
pub fn maximum_swap_670(num: i32) -> i32 {
    let mut digits_vec: Vec<i32> = vec![];
    let mut origin_num = num;
    if origin_num == 0 {
        digits_vec.push(0);
    }
    while origin_num > 0 {
        digits_vec.push(origin_num % 10);
        origin_num /= 10;
    }
    let digits_len = digits_vec.len();
    if digits_len <= 1 {
        return num;
    }
    let mut move_index_vec: Vec<usize> = vec![0];
    let mut dest_index = usize::MAX;
    let mut move_index_vec_index = 0;
    let mut current_index = 1;
    while current_index < digits_len {
        if digits_vec[current_index] > digits_vec[move_index_vec[move_index_vec_index]] {
            move_index_vec.push(current_index);
            move_index_vec_index += 1;
        }
        current_index += 1;
    }
    current_index = digits_len - 1;
    move_index_vec_index = move_index_vec.len() - 1;
    loop {
        if current_index > move_index_vec[move_index_vec_index] {
            if digits_vec[current_index] < digits_vec[move_index_vec[move_index_vec_index]] {
                dest_index = current_index;
                break;
            }
        } else {
            if move_index_vec_index == 0 {
                break;
            }
            move_index_vec_index -= 1;
        }

        if current_index == 0 {
            break;
        }
        current_index -= 1
    }
    let mut ret = 0;
    if dest_index == usize::MAX {
        ret = num;
    } else {
        current_index = digits_len - 1;
        loop {
            if current_index == dest_index {
                ret = ret * 10 + digits_vec[move_index_vec[move_index_vec_index]];
            } else if current_index == move_index_vec[move_index_vec_index] {
                ret = ret * 10 + digits_vec[dest_index];
            } else {
                ret = ret * 10 + digits_vec[current_index];
            }
            if current_index == 0 {
                break;
            }
            current_index -= 1
        }
    }
    ret
}


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


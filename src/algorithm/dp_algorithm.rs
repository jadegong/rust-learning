/// 
/// Leetcode 121 Best time to buy and sell stock
///
pub fn max_profit_121(prices: Vec<i32>) -> i32 {
    let prices_len = prices.len();
    if prices_len == 0 {
        return 0;
    }
    // This to record profit, nums[i] means max profit when sell on day i
    let mut nums: Vec<i32> = vec![];
    let mut max_ret = 0;
    for (i, el) in prices.iter().enumerate() {
        //
        if i == 0 {
            nums.push(0);
        } else {
            if *el - prices[i - 1] + nums[i - 1] >= 0 {
                nums.push(*el - prices[i - 1] + nums[i - 1]);
            } else {
                nums.push(0);
            }
        }
        if nums[i] > max_ret {
            max_ret = nums[i];
        }
    }
    return max_ret;
}

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
    while index < nums_len - 1 {
        current_num = nums[index] as usize;
        if current_num == 0 {
            if max_arrive <= index {
                return false;
            }
        }
        if index + current_num > max_arrive {
            max_arrive = index + current_num;
        }
        index += 1;
    }
    return true;
}



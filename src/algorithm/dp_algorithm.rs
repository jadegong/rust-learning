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


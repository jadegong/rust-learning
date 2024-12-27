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
/// Leetcode 238 
///
pub fn product_except_self_238(nums: Vec<i32>) -> Vec<i32> {
    let mut left_product: Vec<i32> = vec![]; // product from left
    let mut right_product: Vec<i32> = vec![]; // product from right
    let nums_len = nums.len();
    let mut result_nums: Vec<i32> = vec![];
    for (i, _) in nums.iter().enumerate() {
        if i == 0 {
            left_product.push(1);
            right_product.push(1);
        } else {
            left_product.push(left_product[i - 1] * nums[i - 1]);
            right_product.push(right_product[i - 1] * nums[nums_len - i]);
        }
    }
    for (i, _) in left_product.iter().enumerate() {
        result_nums.push(left_product[i] * right_product[nums_len - 1 - i]);
    }
    return result_nums;
}


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

///
/// Leetcode 42
/// Trapping Rain Water
///
pub fn trap_42(height: Vec<i32>) -> i32 {
    let mut left_trap = 0; // from left to right (equal included in this condition)
    let mut right_trap = 0; // from right to left
    let mut left_index = 0;
    let mut right_index = 1;
    let mut current_valley_trap = 0;
    let n = height.len();
    if n <= 2 {
        return 0;
    }
    let mut equal_right_indexes: Vec<usize> = vec![];
    while left_index < n && right_index < n {
        if height[right_index] >= height[left_index] {
            // plus current_valley_trap
            if height[right_index] == height[left_index] {
                // record equal right index
                equal_right_indexes.push(right_index);
            }
            left_trap += current_valley_trap;
            current_valley_trap = 0;
            left_index = right_index;
        } else {
            current_valley_trap += height[left_index] - height[right_index];
        }
        right_index += 1;
    }
    left_index = n - 2;
    right_index = n - 1;
    current_valley_trap = 0;
    // from right to left
    loop {
        if height[left_index] >= height[right_index] {
            // have calculated from left to right
            if height[left_index] > height[right_index] {
                right_trap += current_valley_trap;
            } else {
                // equal
                let find_index = equal_right_indexes.iter().position(|&x| x == right_index);
                if find_index == None {
                    right_trap += current_valley_trap;
                }
            }
            current_valley_trap = 0;
            right_index = left_index;
        } else {
            current_valley_trap += height[right_index] - height[left_index];
        }
        if left_index <= 0 || right_index <= 0 {
            break;
        }
        left_index -= 1;
    }
    return left_trap + right_trap;
}

///
/// Leetcode 12
/// Container With Most Water
///
pub fn max_area_12(height: Vec<i32>) -> i32 {
    let height_len = height.len();
    let mut ret = 0;
    let mut left_index = 0;
    let mut right_index = height_len - 1;
    while left_index < right_index {
        ret = std::cmp::max(ret, std::cmp::min(height[left_index], height[right_index]) * (right_index - left_index) as i32);
        if height[left_index] < height[right_index] {
            left_index += 1;
        } else {
            right_index -= 1;
        }
    }
    return ret;
}

///
/// Leetcode 53
/// Maximum Subarray
///
pub fn maximum_subarray_53(nums: Vec<i32>) -> i32 {
    let nums_len = nums.len();
    let mut ret = i32::MIN;
    let mut current_max = 0;
    let mut index = 0;
    while index < nums_len {
        current_max = std::cmp::max(current_max + nums[index], nums[index]);
        ret = std::cmp::max(current_max, ret);
        index += 1;
    }
    ret
}


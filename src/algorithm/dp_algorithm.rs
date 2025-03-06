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

/// 
/// Leetcode 70
/// Climbing Stairs
///
pub fn climb_stairs_70(n: i32) -> i32 {
    if n == 1 {
        return 1;
    } else if n == 2 {
        return 2;
    }
    let mut prev_two_ways = 1;
    let mut prev_one_ways = 2;
    let mut current_ways = 3;
    let mut index = 3;
    while index <= n {
        current_ways = prev_two_ways + prev_one_ways;
        prev_two_ways = prev_one_ways;
        prev_one_ways = current_ways;
        index += 1;
    }
    current_ways
}

/// 
/// Leetcode 300
/// Triangle
///
pub fn minimum_total_120(triangle: Vec<Vec<i32>>) -> i32 {
    let rows = triangle.len();
    let mut row_index = 0;
    let mut col_index;
    let mut row_dp: Vec<i32> = vec![i32::MAX; rows];
    let mut ret = i32::MAX;
    while row_index < rows {
        col_index = row_index;
        loop {
            if row_index == 0 {
                row_dp[0] = triangle[0][0];
            } else { // Later rows
                if col_index == 0 {
                    row_dp[0] = row_dp[0] + triangle[row_index][0];
                } else if col_index == row_index { // Last column
                    row_dp[col_index] = row_dp[col_index - 1] + triangle[row_index][col_index];
                } else { // Later column
                    row_dp[col_index] = std::cmp::min(row_dp[col_index - 1], row_dp[col_index]) + triangle[row_index][col_index];
                }
            }
            if row_index == rows - 1 {
                ret = std::cmp::min(ret, row_dp[col_index]);
            }
            if col_index == 0 {
                break;
            }
            col_index -= 1;
        }
        row_index += 1;
    }
    ret
}

/// 
/// Leetcode 64
/// Minimum Path Sum
///
pub fn min_path_sum_64(grid: Vec<Vec<i32>>) -> i32 {
    let m_len = grid.len();
    let n_len = grid[0].len();
    let mut row_sum: Vec<i32> = vec![i32::MAX; n_len];
    let mut row_index = 0;
    let mut col_index;
    while row_index < m_len {
        col_index = 0;
        while col_index < n_len {
            if row_index == 0 { // first row
                if col_index == 0 {
                    row_sum[col_index] = grid[0][0];
                } else {
                    row_sum[col_index] = row_sum[col_index - 1] + grid[row_index][col_index];
                }
            } else {
                if col_index == 0 { // first column
                    row_sum[col_index] = row_sum[col_index] + grid[row_index][col_index];
                } else {
                    row_sum[col_index] = std::cmp::min(row_sum[col_index - 1], row_sum[col_index]) + grid[row_index][col_index];
                }
            }
            col_index += 1;
        }
        row_index += 1;
    }
    row_sum[n_len - 1]
}

/// 
/// Leetcode 63
/// Unique Paths II
///
pub fn unique_paths_with_obstacles_63(obstacle_grid: Vec<Vec<i32>>) -> i32 {
    let m_len = obstacle_grid.len();
    let n_len = obstacle_grid[0].len();
    let mut row_dp: Vec<i32> = vec![0; n_len];
    let mut row_index = 0;
    let mut col_index;
    while row_index < m_len {
        col_index = 0;
        while col_index < n_len {
            if row_index == 0 { // first row
                if col_index == 0 {
                    if obstacle_grid[0][0] == 1 {
                        row_dp[0] = 0;
                    } else {
                        row_dp[0] = 1;
                    }
                } else {
                    if obstacle_grid[0][col_index] == 1 {
                        row_dp[col_index] = 0;
                    } else {
                        row_dp[col_index] = row_dp[col_index - 1];
                    }
                }
            } else {
                if col_index == 0 { // first column
                    if obstacle_grid[row_index][col_index] == 1 {
                        row_dp[col_index] = 0;
                    }
                } else {
                    if obstacle_grid[row_index][col_index] == 1 {
                        row_dp[col_index] = 0;
                    } else {
                        row_dp[col_index] = row_dp[col_index - 1] + row_dp[col_index];
                    }
                }
            }
            col_index += 1;
        }
        row_index += 1;
    }
    row_dp[n_len - 1]
}

/// 
/// Leetcode 221
/// Maximal Square
///
pub fn maximal_square_221(matrix: Vec<Vec<char>>) -> i32 {
    let m_len = matrix.len();
    let mut n_len = 0;
    if m_len > 0 {
        n_len = matrix[0].len();
    }
    let mut ret = 0;
    let mut square_width: Vec<i32> = vec![0; n_len];
    let mut square_top_left_width1; // prev row top left
    let mut square_top_left_width2; // prev row top
    let mut row_index = 0;
    let mut col_index;
    while row_index < m_len {
        col_index = 0;
        square_top_left_width2 = 0;
        while col_index < n_len {
            square_top_left_width1 = square_top_left_width2;
            square_top_left_width2 = square_width[col_index];
            if row_index == 0 {
                if matrix[row_index][col_index] == '1' {
                    square_width[col_index] = 1;
                }
            } else {
                if matrix[row_index][col_index] == '1' {
                    if col_index == 0 {
                        square_width[col_index] = 1;
                    } else {
                        square_width[col_index] = std::cmp::min(std::cmp::min(square_width[col_index - 1], square_width[col_index]), square_top_left_width1) + 1;
                    }
                } else {
                    square_width[col_index] = 0;
                }
            }
            ret = std::cmp::max(ret, square_width[col_index]);
            col_index += 1;
        }
        row_index += 1;
    }
    ret * ret
}

/// 
/// Leetcode 1368
/// Minimum Cost to Make at Least One Valid Path in a Grid
/// TODO: Not the shortest path!!!
pub fn min_cost_1368(grid: Vec<Vec<i32>>) -> i32 {
    let m = grid.len();
    if m == 0 {
        return 0;
    }
    let n = grid[0].len();
    let mut min_dp: Vec<i32> = vec![0; n];
    let mut row_index = 0;
    let mut col_index;
    while row_index < m {
        col_index = 0;
        while col_index < n {
            if row_index == 0 { // First row
                if col_index == 0 {
                    min_dp[col_index] = 0;
                } else {
                    if grid[row_index][col_index - 1] == 1 {
                        min_dp[col_index] = min_dp[col_index - 1];
                    } else {
                        min_dp[col_index] = min_dp[col_index - 1] + 1;
                    }
                }
            } else {
                if col_index == 0 {
                    if grid[row_index - 1][col_index] == 3 {
                        min_dp[col_index] = min_dp[col_index];
                    } else {
                        min_dp[col_index] = min_dp[col_index] + 1;
                    }
                } else {
                    let mut left_min = min_dp[col_index - 1];
                    if grid[row_index][col_index - 1] != 1 {
                        left_min += 1;
                    }
                    let mut top_min = min_dp[col_index];
                    if grid[row_index - 1][col_index] != 3 {
                        top_min += 1;
                    }
                    min_dp[col_index] = std::cmp::min(left_min, top_min);
                }
            }
            col_index += 1;
        }
        row_index += 1;
    }
    min_dp[n - 1]
}

/// 
/// Leetcode 2466
/// Count Ways To Build Good Strings
///
pub fn count_good_strings_2466(low: i32, high: i32, zero: i32, one: i32) -> i32 {
    let mut dp_count: Vec<i32> = vec![0; 100001];
    let modulo = 1000000007;
    let mut index: usize = 0;
    let min_num = std::cmp::min(zero, one) as usize;
    let max_num = std::cmp::max(zero, one) as usize;
    let mut ret = 0;
    while index as i32 <= high {
        if index < min_num {
            dp_count[index] = 0;
        } else if index == min_num {
            dp_count[index] = 1;
            if index == max_num {
                dp_count[index] = 2;
            }
        } else if index > min_num && index < max_num {
            dp_count[index] = dp_count[index - min_num] % modulo;
        } else if index == max_num {
            dp_count[index] = (dp_count[index - min_num] + 1) % modulo;
        } else if index > max_num {
            dp_count[index] = (dp_count[index - min_num] + dp_count[index - max_num]) % modulo;
        }
        if index >= low as usize {
            ret = (ret + dp_count[index]) % modulo;
        }
        index += 1;
    }
    ret
}

/// 
/// Leetcode 1014
/// Best Sightseeing Pair
///
pub fn max_score_sightseeing_pair_1014(values: Vec<i32>) -> i32 {
    let values_len = values.len();
    if values_len <= 1 {
        return 0;
    }
    let mut left_index = 0;
    let mut right_index = 1;
    let mut ret = values[left_index] + left_index as i32 + values[right_index] - right_index as i32;
    right_index += 1;
    while right_index < values_len {
        if values[right_index - 1] + (right_index as i32 - 1) >= values[left_index] + left_index as i32 {
            left_index = right_index - 1;
        }
        ret = std::cmp::max(values[left_index] + left_index as i32 + values[right_index] - right_index as i32, ret);
        right_index += 1;
    }
    ret
}


///
/// Backtracking algorithms
/// 回溯算法
///

/// 
/// Leetcode 77
/// Combinations
///
pub fn combine_77(n: i32, k: i32) -> Vec<Vec<i32>> {
    fn inner_combine(res: &mut Vec<Vec<i32>>, current_vec: &mut Vec<i32>, index: i32, n: i32, k: i32) {
        // 1.index over max number;
        // 2.current_vec length over k;
        if index > n || current_vec.len() >= k as usize {
            return;
        }
        let mut loop_index = index;
        while loop_index <= n {
            current_vec.push(loop_index);
            if current_vec.len() == k as usize {
                res.push(current_vec.to_vec());
            }
            inner_combine(res, current_vec, loop_index + 1, n, k);
            current_vec.pop();
            loop_index += 1;
        }
    }
    let mut res: Vec<Vec<i32>> = vec![];
    let mut current_vec: Vec<i32> = vec![];
    inner_combine(&mut res, &mut current_vec, 1, n, k);
    res
}

/// 
/// Leetcode 78
/// Subsets
///
pub fn subsets_78(nums: Vec<i32>) -> Vec<Vec<i32>> {
    fn inner_subsets(res: &mut Vec<Vec<i32>>, current_vec: &mut Vec<i32>, nums: &Vec<i32>, index: usize) {
        res.push(current_vec.to_vec());
        // 1.index over nums length;
        let nums_len = nums.len();
        if index >= nums_len {
            return;
        }
        let mut loop_index = index;
        while loop_index < nums_len {
            current_vec.push(nums[loop_index]);
            inner_subsets(res, current_vec, nums, loop_index + 1);
            current_vec.pop();
            loop_index += 1;
        }
    }
    let mut res: Vec<Vec<i32>> = vec![];
    let mut current_vec: Vec<i32> = vec![];
    inner_subsets(&mut res, &mut current_vec, &nums, 0);
    res
}

/// 
/// Leetcode 216
/// Combination Sum III
///
pub fn combination_sum3_216(k: i32, n: i32) -> Vec<Vec<i32>> {
    fn inner_combination_sum3(res: &mut Vec<Vec<i32>>, current_vec: &mut Vec<i32>, index: i32, target: i32, k: i32) {
        // 1.index over max number;
        // 2.current_vec length over k;
        // 3.index over target, impossible to find index==target;
        if index > 9 || current_vec.len() >= k as usize || index > target {
            return;
        }
        let mut loop_index = index;
        while loop_index <= 9 {
            current_vec.push(loop_index);
            if loop_index == target && current_vec.len() == k as usize {
                res.push(current_vec.to_vec());
            }
            inner_combination_sum3(res, current_vec, loop_index + 1, target - loop_index, k);
            current_vec.pop();
            loop_index += 1;
        }
    }
    let mut res: Vec<Vec<i32>> = vec![];
    let mut current_vec: Vec<i32> = vec![];
    inner_combination_sum3(&mut res, &mut current_vec, 1, n, k);
    res
}

/// 
/// Leetcode 46
/// Permutations
///
pub fn permute_46(nums: Vec<i32>) -> Vec<Vec<i32>> {
    fn inner_permute(res: &mut Vec<Vec<i32>>, current_vec: &mut Vec<i32>, choices: &mut Vec<bool>, nums: &Vec<i32>, nums_len: usize) {
        // 1.current_vec length is the same with nums_len;
        if current_vec.len() == nums_len {
            res.push(current_vec.to_vec());
            return;
        }
        let mut index = 0;
        while index < nums_len {
            if !choices[index] {
                current_vec.push(nums[index]);
                choices[index] = true;
                inner_permute(res, current_vec, choices, nums, nums_len);
                current_vec.pop();
                choices[index] = false;
            }
            index += 1;
        }
    }
    let nums_len = nums.len();
    let mut res: Vec<Vec<i32>> = vec![];
    let mut current_vec: Vec<i32> = vec![];
    let mut choices: Vec<bool> = vec![false;nums_len];
    inner_permute(&mut res, &mut current_vec, &mut choices, &nums, nums_len);
    res
}

/// 
/// Leetcode 51
/// N-Queens
///
pub fn solve_n_queens_51(n: i32) -> Vec<Vec<String>> {
    fn inner_n_queen(res: &mut Vec<Vec<String>>, current_vec: &mut Vec<String>, row_map: &mut Vec<bool>, col_map: &mut Vec<bool>, lt_rb_map: &mut std::collections::HashMap<i32, bool>, rt_lb_map: &mut std::collections::HashMap<i32, bool>, n: i32) {
        if current_vec.len() == n as usize {
            res.push(current_vec.to_vec());
            return;
        }
        let mut index = 0;
        let row = current_vec.len();
        while index < n as usize {
            if row_map[index] == false && col_map[row] == false && !lt_rb_map.contains_key(&((index - row) as i32)) && !rt_lb_map.contains_key(&((index + row) as i32)) {
                let mut current_row_chars: Vec<char> = vec!['.';n as usize];
                current_row_chars[index] = 'Q';
                let current_row: String = current_row_chars.into_iter().collect();
                row_map[index] = true;
                col_map[row] = true;
                lt_rb_map.insert((index - row) as i32, true);
                rt_lb_map.insert((index + row) as i32, true);
                current_vec.push(current_row);
                inner_n_queen(res, current_vec, row_map, col_map, lt_rb_map, rt_lb_map, n);
                row_map[index] = false;
                col_map[row] = false;
                lt_rb_map.remove(&((index - row) as i32));
                rt_lb_map.remove(&((index + row) as i32));
                current_vec.pop();
            }
            index += 1;
        }
    }
    let mut res: Vec<Vec<String>> = vec![];
    let mut current_vec: Vec<String> = vec![];
    let mut row_map: Vec<bool> = vec![false; n as usize];
    let mut col_map: Vec<bool> = vec![false; n as usize];
    let mut lt_rb_map: std::collections::HashMap<i32, bool> = std::collections::HashMap::new();
    let mut rt_lb_map: std::collections::HashMap<i32, bool> = std::collections::HashMap::new();
    inner_n_queen(&mut res, &mut current_vec, &mut row_map, &mut col_map, &mut lt_rb_map, &mut rt_lb_map, n);
    res
}

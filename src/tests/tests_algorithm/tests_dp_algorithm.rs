use crate::algorithm::dp_algorithm;

#[test]
fn test_max_profit_121() {
    let test_prices1: Vec<i32> = vec![7, 1, 5, 3, 6, 4];
    let test_ret1 = dp_algorithm::max_profit_121(test_prices1);
    assert_eq!(test_ret1, 5);
    let test_prices1: Vec<i32> = vec![7, 6, 4, 3, 1];
    let test_ret1 = dp_algorithm::max_profit_121(test_prices1);
    assert_eq!(test_ret1, 0);
}

#[test]
fn test_product_except_self_238() {
    let test_nums: Vec<i32> = vec![1, 2, 3, 4];
    let result_nums = dp_algorithm::product_except_self_238(test_nums);
    let expected_nums: Vec<i32> = vec![24, 12, 8 ,6];
    for (i, el) in result_nums.iter().enumerate() {
        assert_eq!(*el, expected_nums[i]);
    }
    let test_nums: Vec<i32> = vec![-1, 1, 0, -3, 3];
    let result_nums = dp_algorithm::product_except_self_238(test_nums);
    let expected_nums: Vec<i32> = vec![0, 0, 9, 0, 0];
    for (i, el) in result_nums.iter().enumerate() {
        assert_eq!(*el, expected_nums[i]);
    }
}

#[test]
fn test_trap_42() {
    let test_height: Vec<i32> = vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1];
    let result_trap = dp_algorithm::trap_42(test_height);
    let expected_trap = 6;
    assert_eq!(result_trap, expected_trap);
    let test_height: Vec<i32> = vec![4, 2, 0, 3, 2, 5];
    let result_trap = dp_algorithm::trap_42(test_height);
    let expected_trap = 9;
    assert_eq!(result_trap, expected_trap);
    let test_height: Vec<i32> = vec![4, 2, 0, 3, 2, 4, 3, 4];
    let result_trap = dp_algorithm::trap_42(test_height);
    let expected_trap = 10;
    assert_eq!(result_trap, expected_trap);
}

#[test]
fn test_max_area_12() {
    let test_height: Vec<i32> = vec![1,8,6,2,5,4,8,3,7];
    let expected_ret = 49;
    let test_ret = dp_algorithm::max_area_12(test_height);
    assert_eq!(test_ret, expected_ret);
    let test_height: Vec<i32> = vec![1, 1];
    let expected_ret = 1;
    let test_ret = dp_algorithm::max_area_12(test_height);
    assert_eq!(test_ret, expected_ret);
    let test_height: Vec<i32> = vec![2,3,10,5,7,8,9];
    let expected_ret = 36;
    let test_ret = dp_algorithm::max_area_12(test_height);
    assert_eq!(test_ret, expected_ret);
}

#[test]
fn test_maximum_subarray_53() {
    let test_nums: Vec<i32> = vec![-2,1,-3,4,-1,2,1,-5,4];
    let expected_ret = 6;
    assert_eq!(dp_algorithm::maximum_subarray_53(test_nums), expected_ret);
    let test_nums: Vec<i32> = vec![5,4,-1,7,8];
    let expected_ret = 23;
    assert_eq!(dp_algorithm::maximum_subarray_53(test_nums), expected_ret);
}

#[test]
pub fn test_climb_stairs_70() {
    let test_n = 3;
    let expected_ret = 3;
    assert_eq!(dp_algorithm::climb_stairs_70(test_n), expected_ret);
    let test_n = 5;
    let expected_ret = 8;
    assert_eq!(dp_algorithm::climb_stairs_70(test_n), expected_ret);
}

#[test]
pub fn test_minimum_total_120() {
    let test_triangle: Vec<Vec<i32>> = vec![vec![2], vec![3,4], vec![6,5,7], vec![4,1,8,3]];
    let expected_ret = 11;
    assert_eq!(dp_algorithm::minimum_total_120(test_triangle), expected_ret);
}

#[test]
pub fn test_min_path_sum_64() {
    let test_grid: Vec<Vec<i32>> = vec![vec![1,3,1], vec![1,5,1], vec![4,2,1]];
    let expected_ret = 7;
    assert_eq!(dp_algorithm::min_path_sum_64(test_grid), expected_ret);
    let test_grid: Vec<Vec<i32>> = vec![vec![1,2,3], vec![4,5,6]];
    let expected_ret = 12;
    assert_eq!(dp_algorithm::min_path_sum_64(test_grid), expected_ret);
}

#[test]
pub fn test_unique_paths_with_obstacles_63() {
    let test_grid: Vec<Vec<i32>> = vec![vec![0,0,0],vec![0,1,0],vec![0,0,0]];
    let expected_ret = 2;
    assert_eq!(dp_algorithm::unique_paths_with_obstacles_63(test_grid), expected_ret);
    let test_grid: Vec<Vec<i32>> = vec![vec![0,1],vec![0,0]];
    let expected_ret = 1;
    assert_eq!(dp_algorithm::unique_paths_with_obstacles_63(test_grid), expected_ret);
}

#[test]
pub fn test_maximal_square_221() {
    let test_matrix: Vec<Vec<char>> = vec![vec!['1','0','1','0','0'],vec!['1','0','1','1','1'],vec!['1','1','1','1','1'],vec!['1','0','0','1','0']];
    let expected_ret = 4;
    assert_eq!(dp_algorithm::maximal_square_221(test_matrix), expected_ret);
    let test_matrix: Vec<Vec<char>> = vec![vec!['0', '1'],vec!['1', '0']];
    let expected_ret = 1;
    assert_eq!(dp_algorithm::maximal_square_221(test_matrix), expected_ret);
}

#[test]
pub fn test_count_good_strings_2466() {
    let expected_ret = 8;
    assert_eq!(dp_algorithm::count_good_strings_2466(3, 3, 1, 1), expected_ret);
    let expected_ret = 5;
    assert_eq!(dp_algorithm::count_good_strings_2466(2,3,1,2), expected_ret);
}

#[test]
pub fn test_max_score_sightseeing_pair_1014() {
    let test_values: Vec<i32> = vec![8,1,5,2,6];
    let expected_ret = 11;
    assert_eq!(dp_algorithm::max_score_sightseeing_pair_1014(test_values), expected_ret);
    let test_values: Vec<i32> = vec![1,2];
    let expected_ret = 2;
    assert_eq!(dp_algorithm::max_score_sightseeing_pair_1014(test_values), expected_ret);
}

#[test]
pub fn test_max_absolute_sum_1749() {
    let test_nums: Vec<i32> = vec![1,-3,2,3,-4];
    let expected_ret = 5;
    assert_eq!(dp_algorithm::max_absolute_sum_1749(test_nums), expected_ret);
    let test_nums: Vec<i32> = vec![2,-5,1,-4,3,-2];
    let expected_ret = 8;
    assert_eq!(dp_algorithm::max_absolute_sum_1749(test_nums), expected_ret);
}

#[test]
pub fn test_num_of_subarrays_1524() {
    let test_arr: Vec<i32> = vec![1,3,5];
    let expected_ret = 4;
    assert_eq!(dp_algorithm::num_of_subarrays_1524(test_arr), expected_ret);
    let test_arr: Vec<i32> = vec![2,4,6];
    let expected_ret = 0;
    assert_eq!(dp_algorithm::num_of_subarrays_1524(test_arr), expected_ret);
    let test_arr: Vec<i32> = vec![1,2,3,4,5,6,7];
    let expected_ret = 16;
    assert_eq!(dp_algorithm::num_of_subarrays_1524(test_arr), expected_ret);
}

#[test]
pub fn test_min_distance_72() {
    let test_word1: String = String::from("horse");
    let test_word2: String = String::from("ros");
    let expected_ret = 3;
    assert_eq!(dp_algorithm::min_distance_72(test_word1, test_word2), expected_ret);
    let test_word1: String = String::from("intention");
    let test_word2: String = String::from("execution");
    let expected_ret = 5;
    assert_eq!(dp_algorithm::min_distance_72(test_word1, test_word2), expected_ret);
}

#[test]
pub fn test_generate_118() {
    let test_num_rows = 5;
    let expected_ret: Vec<Vec<i32>> = vec![vec![1],vec![1,1],vec![1,2,1],vec![1,3,3,1],vec![1,4,6,4,1]];
    assert_eq!(dp_algorithm::generate_118(test_num_rows), expected_ret);
    let test_num_rows = 1;
    let expected_ret: Vec<Vec<i32>> = vec![vec![1]];
    assert_eq!(dp_algorithm::generate_118(test_num_rows), expected_ret);
}

#[test]
pub fn test_max_profit_123() {
    let test_prices: Vec<i32> = vec![3,3,5,0,0,3,1,4];
    let expected_ret = 6;
    assert_eq!(dp_algorithm::max_profit_123(test_prices), expected_ret);
    let test_prices: Vec<i32> = vec![1,2,3,4,5];
    let expected_ret = 4;
    assert_eq!(dp_algorithm::max_profit_123(test_prices), expected_ret);
    let test_prices: Vec<i32> = vec![7,6,4,3,1];
    let expected_ret = 0;
    assert_eq!(dp_algorithm::max_profit_123(test_prices), expected_ret);
    let test_prices: Vec<i32> = vec![6,1,3,2,4,7];
    let expected_ret = 7;
    assert_eq!(dp_algorithm::max_profit_123(test_prices), expected_ret);
}

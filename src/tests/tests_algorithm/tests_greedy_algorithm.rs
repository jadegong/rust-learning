use crate::algorithm::greedy_algorithm;

#[test]
fn test_can_jump_55() {
    let test_nums1: Vec<i32> = vec![2, 3, 1, 1, 4];
    let test_ret1 = greedy_algorithm::can_jump_55(test_nums1);
    assert_eq!(test_ret1, true);
    let test_nums1: Vec<i32> = vec![3, 2, 1, 0, 4];
    let test_ret1 = greedy_algorithm::can_jump_55(test_nums1);
    assert_eq!(test_ret1, false);
}

#[test]
fn test_jump_game_45() {
    let test_nums: Vec<i32> = vec![2, 3, 1, 1, 4];
    let test_ret = greedy_algorithm::jump_game_45(test_nums);
    assert_eq!(test_ret, 2);
    let test_nums: Vec<i32> = vec![2, 3, 0, 1, 4];
    let test_ret = greedy_algorithm::jump_game_45(test_nums);
    assert_eq!(test_ret, 2);
}

#[test]
fn test_candy_135() {
    let test_nums: Vec<i32> = vec![1, 0 ,2];
    let test_ret = greedy_algorithm::candy_135(test_nums);
    assert_eq!(test_ret, 5);
    let test_nums: Vec<i32> = vec![1, 3, 2, 2, 1];
    let test_ret = greedy_algorithm::candy_135(test_nums);
    assert_eq!(test_ret, 7);
}

#[test]
fn test_max_profix_122() {
    let test_prices: Vec<i32> = vec![7, 1, 5, 3, 6, 4];
    let expected_ret = 7;
    let test_ret = greedy_algorithm::max_profit_122(test_prices);
    assert_eq!(test_ret, expected_ret);
    let test_prices: Vec<i32> = vec![1, 2, 3, 4, 5];
    let expected_ret = 4;
    let test_ret = greedy_algorithm::max_profit_122(test_prices);
    assert_eq!(test_ret, expected_ret);
    let test_prices: Vec<i32> = vec![7, 6, 4, 3, 1];
    let expected_ret = 0;
    let test_ret = greedy_algorithm::max_profit_122(test_prices);
    assert_eq!(test_ret, expected_ret);
}

#[test]
pub fn test_maximum_swap_670() {
    let test_num: i32 = 2736;
    let expected_ret: i32 = 7236;
    assert_eq!(greedy_algorithm::maximum_swap_670(test_num), expected_ret);
    let test_num: i32 = 9973;
    let expected_ret: i32 = 9973;
    assert_eq!(greedy_algorithm::maximum_swap_670(test_num), expected_ret);
}


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
fn test_can_jump_55() {
    let test_nums1: Vec<i32> = vec![2, 3, 1, 1, 4];
    let test_ret1 = dp_algorithm::can_jump_55(test_nums1);
    assert_eq!(test_ret1, true);
    let test_nums1: Vec<i32> = vec![3, 2, 1, 0, 4];
    let test_ret1 = dp_algorithm::can_jump_55(test_nums1);
    assert_eq!(test_ret1, false);
}


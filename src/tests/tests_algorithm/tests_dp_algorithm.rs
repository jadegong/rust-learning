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


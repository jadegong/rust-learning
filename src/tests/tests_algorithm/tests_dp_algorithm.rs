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


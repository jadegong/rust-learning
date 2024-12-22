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


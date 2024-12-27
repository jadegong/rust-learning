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


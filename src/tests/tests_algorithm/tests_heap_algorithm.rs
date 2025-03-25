use crate::algorithm::{array_algorithm, heap_algorithms};

#[test]
pub fn test_find_kth_largest_215() {
    let test_nums: Vec<i32> = vec![3,2,1,5,6,4];
    let test_k: i32 = 2;
    let expected_ret: i32 = 5;
    assert_eq!(heap_algorithms::find_kth_largest_215(test_nums, test_k), expected_ret);
    let test_nums: Vec<i32> = vec![3,2,3,1,2,4,5,5,6];
    let test_k: i32 = 4;
    let expected_ret: i32 = 4;
    assert_eq!(heap_algorithms::find_kth_largest_215(test_nums, test_k), expected_ret);
}

#[test]
pub fn test_max_sliding_window_239() {
    let test_nums: Vec<i32> = vec![1,3,-1,-3,5,3,6,7];
    let expected_ret = vec![3, 3, 5, 5, 6, 7];
    let test_ret = heap_algorithms::max_sliding_window_239(test_nums, 3);
    assert_eq!(test_ret, expected_ret);
    let test_nums: Vec<i32> = vec![1];
    let expected_ret = vec![1];
    let test_ret = heap_algorithms::max_sliding_window_239(test_nums, 1);
    assert_eq!(test_ret, expected_ret);
}

#[test]
pub fn test_top_k_frequent_347() {
    let test_nums: Vec<i32> = vec![1,1,1,2,2,3,3,4];
    let test_k: i32 = 3;
    let expected_ret: Vec<i32> = vec![1,2,3];
    assert_eq!(heap_algorithms::top_k_frequent_347(test_nums, test_k), expected_ret);
    let test_nums: Vec<i32> = vec![1];
    let test_k: i32 = 1;
    let expected_ret: Vec<i32> = vec![1];
    assert_eq!(heap_algorithms::top_k_frequent_347(test_nums, test_k), expected_ret);
    let test_nums: Vec<i32> = vec![5,3,1,1,1,3,73,1];
    let test_k: i32 = 2;
    let expected_ret: Vec<i32> = vec![1,3];
    assert_eq!(heap_algorithms::top_k_frequent_347(test_nums, test_k), expected_ret);
}

#[test]
pub fn test_find_relative_ranks_506() {
    let test_score: Vec<i32> = vec![5,4,3,2,1];
    let expected_ret: Vec<String> = vec![String::from("Gold Medal"),String::from("Silver Medal"),String::from("Bronze Medal"),String::from("4"),String::from("5")];
    assert_eq!(heap_algorithms::find_relative_ranks_506(test_score), expected_ret);
    let test_score: Vec<i32> = vec![10,3,8,9,4];
    let expected_ret: Vec<String> = vec![String::from("Gold Medal"),String::from("5"),String::from("Bronze Medal"),String::from("Silver Medal"),String::from("4")];
    assert_eq!(heap_algorithms::find_relative_ranks_506(test_score), expected_ret);
}

#[test]
pub fn test_least_interval_621() {
    let test_tasks: Vec<char> = vec!['A','A','A','B','B','B'];
    let test_n: i32 = 2;
    let expected_ret: i32 = 8;
    assert_eq!(heap_algorithms::least_interval_621(test_tasks, test_n), expected_ret);
    let test_tasks: Vec<char> = vec!['A','C','A','B','D','B'];
    let test_n: i32 = 1;
    let expected_ret: i32 = 6;
    assert_eq!(heap_algorithms::least_interval_621(test_tasks, test_n), expected_ret);
    let test_tasks: Vec<char> = vec!['A','A','A','B','B','B'];
    let test_n: i32 = 3;
    let expected_ret: i32 = 10;
    assert_eq!(heap_algorithms::least_interval_621(test_tasks, test_n), expected_ret);
    let test_tasks: Vec<char> = vec!['B','C','D','A','A','A','A','G'];
    let test_n: i32 = 1;
    let expected_ret: i32 = 8;
    assert_eq!(heap_algorithms::least_interval_621(test_tasks, test_n), expected_ret);
}

#[test]
pub fn test_longest_consecutive_128() {
    let test_nums: Vec<i32> = vec![100,4,200,1,3,2];
    let expected_ret: i32 = 4;
    assert_eq!(array_algorithm::longest_consecutive_128(test_nums), expected_ret);
    let test_nums: Vec<i32> = vec![0,3,7,2,5,8,4,6,0,1];
    let expected_ret: i32 = 9;
    assert_eq!(array_algorithm::longest_consecutive_128(test_nums), expected_ret);
}

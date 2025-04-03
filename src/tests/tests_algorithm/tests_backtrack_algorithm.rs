use crate::algorithm::backtrack_algorithm;

#[test]
pub fn test_combination_sum3_216() {
    let test_k: i32 = 3;
    let test_n: i32 = 7;
    let expected_ret: Vec<Vec<i32>> = vec![vec![1,2,4]];
    assert_eq!(backtrack_algorithm::combination_sum3_216(test_k, test_n), expected_ret);
    let test_k: i32 = 3;
    let test_n: i32 = 9;
    let expected_ret: Vec<Vec<i32>> = vec![vec![1,2,6],vec![1,3,5],vec![2,3,4]];
    assert_eq!(backtrack_algorithm::combination_sum3_216(test_k, test_n), expected_ret);
    let test_k: i32 = 4;
    let test_n: i32 = 1;
    let expected_ret: Vec<Vec<i32>> = vec![];
    assert_eq!(backtrack_algorithm::combination_sum3_216(test_k, test_n), expected_ret);
}

#[test]
pub fn test_subsets_78() {
    let test_nums: Vec<i32> = vec![1,2,3];
    let expected_ret: Vec<Vec<i32>> = vec![vec![],vec![1],vec![1,2],vec![1,2,3],vec![1,3],vec![2],vec![2,3],vec![3]];
    assert_eq!(backtrack_algorithm::subsets_78(test_nums), expected_ret);
}

#[test]
pub fn test_permute_46() {
    let test_nums: Vec<i32> = vec![1,2,3];
    let expected_ret: Vec<Vec<i32>> = vec![vec![1,2,3],vec![1,3,2],vec![2,1,3],vec![2,3,1],vec![3,1,2],vec![3,2,1]];
    assert_eq!(backtrack_algorithm::permute_46(test_nums), expected_ret);
}

#[test]
pub fn test_total_n_queens_52() {
    let test_n: i32 = 4;
    let expected_ret: i32 = 2;
    assert_eq!(backtrack_algorithm::total_n_queens_52(test_n), expected_ret);
}

#[test]
pub fn test_exist_79() {
    let test_board: Vec<Vec<char>> = vec![vec!['A', 'B', 'C', 'E'], vec!['S', 'F', 'C', 'S'], vec!['A', 'D', 'E', 'E']];
    let test_word: String = String::from("ABCCED");
    let expected_ret = true;
    assert_eq!(backtrack_algorithm::exist_79(test_board, test_word), expected_ret);
    let test_board: Vec<Vec<char>> = vec![vec!['A', 'B', 'C', 'E'], vec!['S', 'F', 'C', 'S'], vec!['A', 'D', 'E', 'E']];
    let test_word: String = String::from("ABCB");
    let expected_ret = false;
    assert_eq!(backtrack_algorithm::exist_79(test_board, test_word), expected_ret);
}

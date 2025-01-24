use crate::algorithm::linked_list_algorithm;

#[test]
pub fn test_add_two_numbers_2() {
    let l1_nums: Vec<i32> = vec![2, 4, 3];
    let l2_nums: Vec<i32> = vec![5, 6, 4];
    let l1 = linked_list_algorithm::create_linked_list(l1_nums);
    let l2 = linked_list_algorithm::create_linked_list(l2_nums);
    let ret_nums: Vec<i32> = vec![7, 0, 8];
    let ret_l = linked_list_algorithm::create_linked_list(ret_nums);
    assert!(linked_list_algorithm::compare_two_linked_list(linked_list_algorithm::add_two_numbers_2(l1, l2), ret_l));
}

#[test]
pub fn test_reverse_between_92() {
    let test_nums: Vec<i32> = vec![1, 2, 3, 4, 5];
    let test_list = linked_list_algorithm::create_linked_list(test_nums);
    let expected_nums: Vec<i32> = vec![1, 4, 3, 2, 5];
    let expected_list = linked_list_algorithm::create_linked_list(expected_nums);
    assert!(linked_list_algorithm::compare_two_linked_list(expected_list, linked_list_algorithm::reverse_between_92(test_list, 2, 4)));
}

#[test]
pub fn test_reverse_k_group_25() {
    let test_nums: Vec<i32> = vec![1, 2, 3, 4, 5];
    let test_list = linked_list_algorithm::create_linked_list(test_nums);
    let expected_nums: Vec<i32> = vec![3, 2, 1, 4, 5];
    let expected_list = linked_list_algorithm::create_linked_list(expected_nums);
    assert!(linked_list_algorithm::compare_two_linked_list(expected_list, linked_list_algorithm::reverse_k_group_25(test_list, 3)));
    let test_nums: Vec<i32> = vec![1, 2, 3, 4, 5];
    let test_list = linked_list_algorithm::create_linked_list(test_nums);
    let expected_nums: Vec<i32> = vec![2, 1, 4, 3, 5];
    let expected_list = linked_list_algorithm::create_linked_list(expected_nums);
    assert!(linked_list_algorithm::compare_two_linked_list(expected_list, linked_list_algorithm::reverse_k_group_25(test_list, 2)));
}

#[test]
pub fn test_remove_nth_from_end_19() {
    let test_nums: Vec<i32> = vec![1, 2, 3, 4, 5];
    let test_list = linked_list_algorithm::create_linked_list(test_nums);
    let expected_nums: Vec<i32> = vec![1, 2, 3, 5];
    let expected_list = linked_list_algorithm::create_linked_list(expected_nums);
    assert!(linked_list_algorithm::compare_two_linked_list(expected_list, linked_list_algorithm::remove_nth_from_end_19(test_list, 2)));
    let test_nums: Vec<i32> = vec![1, 2];
    let test_list = linked_list_algorithm::create_linked_list(test_nums);
    let expected_nums: Vec<i32> = vec![1];
    let expected_list = linked_list_algorithm::create_linked_list(expected_nums);
    assert!(linked_list_algorithm::compare_two_linked_list(expected_list, linked_list_algorithm::remove_nth_from_end_19(test_list, 1)));
}


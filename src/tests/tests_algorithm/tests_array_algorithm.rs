use crate::algorithm::array_algorithm;

#[test]
fn test_remove_element_27() {
    let mut test_nums: Vec<i32> = vec![0, 1, 2, 2, 3, 0, 4, 2];
    let val: i32 = 2;
    let ret_len = array_algorithm::remove_element_27(&mut test_nums, val);
    assert_eq!(ret_len, 5);
}

#[test]
fn test_remove_duplicates_26() {
    let mut test_nums: Vec<i32> = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
    let ret_len = array_algorithm::remove_duplicates_26(&mut test_nums);
    assert_eq!(ret_len, 5);
}

#[test]
fn test_remove_duplicates_80() {
    let mut test_nums: Vec<i32> = vec![0, 0, 1, 1, 1, 1, 2, 3, 3];
    let expected_nums: Vec<i32> = vec![0, 0, 1, 1, 2, 3, 3];
    let ret_len = array_algorithm::remove_duplicates_80(&mut test_nums);
    assert_eq!(ret_len, 7);
    for (i, el) in expected_nums.iter().enumerate() {
        assert_eq!(test_nums[i], *el);
    }
}


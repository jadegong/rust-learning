use crate::algorithm::graph_algorithms;

#[test]
pub fn test_can_finish_207() {
    let test_num_courses: i32 = 2;
    let test_prerequisites: Vec<Vec<i32>> = vec![vec![1,0]];
    let expected_ret = true;
    assert_eq!(graph_algorithms::can_finish_207(test_num_courses, test_prerequisites), expected_ret);
    let test_num_courses: i32 = 2;
    let test_prerequisites: Vec<Vec<i32>> = vec![vec![1,0],vec![0,1]];
    let expected_ret = false;
    assert_eq!(graph_algorithms::can_finish_207(test_num_courses, test_prerequisites), expected_ret);
}

use std::rc::Rc;
use std::cell::RefCell;
use crate::structure::treenode::TreeNode;
use crate::algorithm::binary_tree_algorithm;

#[test]
pub fn test_is_same_tree_100() {
    let mut test_p = TreeNode::new(1);
    test_p.left = Some(Rc::new(RefCell::new(TreeNode::new(2))));
    test_p.right = Some(Rc::new(RefCell::new(TreeNode::new(3))));
    let mut test_q = TreeNode::new(1);
    test_q.left = Some(Rc::new(RefCell::new(TreeNode::new(2))));
    test_q.right = Some(Rc::new(RefCell::new(TreeNode::new(3))));
    assert_eq!(binary_tree_algorithm::is_same_tree_100(Some(Rc::new(RefCell::new(test_p))), Some(Rc::new(RefCell::new(test_q)))), true);
    let mut test_p = TreeNode::new(1);
    test_p.left = Some(Rc::new(RefCell::new(TreeNode::new(2))));
    let mut test_q = TreeNode::new(1);
    test_q.right = Some(Rc::new(RefCell::new(TreeNode::new(2))));
    assert_eq!(binary_tree_algorithm::is_same_tree_100(Some(Rc::new(RefCell::new(test_p))), Some(Rc::new(RefCell::new(test_q)))), false);
}

#[test]
pub fn test_sum_numbers_129() {
    let mut test_root = TreeNode::new(1);
    test_root.left = Some(Rc::new(RefCell::new(TreeNode::new(2))));
    test_root.right = Some(Rc::new(RefCell::new(TreeNode::new(3))));
    assert_eq!(binary_tree_algorithm::sum_numbers_129(Some(Rc::new(RefCell::new(test_root)))), 25);
    let mut test_root = TreeNode::new(4);
    let mut root_left = TreeNode::new(9);
    root_left.left = Some(Rc::new(RefCell::new(TreeNode::new(5))));
    root_left.right = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    let root_right = Some(Rc::new(RefCell::new(TreeNode::new(0))));
    test_root.left = Some(Rc::new(RefCell::new(root_left)));
    test_root.right = root_right;
    assert_eq!(binary_tree_algorithm::sum_numbers_129(Some(Rc::new(RefCell::new(test_root)))), 1026);
}

#[test]
pub fn test_inorder_traversal_94() {
    let test_nums: Vec<i32> = vec![1, i32::MIN, 2, i32::MIN, i32::MIN, 3];
    let test_root = binary_tree_algorithm::create_binary_tree(test_nums);
    let expected_ret: Vec<i32> = vec![1,3,2];
    assert_eq!(binary_tree_algorithm::inorder_traversal_94(test_root), expected_ret);
    let test_nums: Vec<i32> = vec![1,2,3,4,5,i32::MIN,8,i32::MIN,i32::MIN,6,7,i32::MIN,i32::MIN,9];
    let test_root = binary_tree_algorithm::create_binary_tree(test_nums);
    let expected_ret: Vec<i32> = vec![4,2,6,5,7,1,3,9,8];
    assert_eq!(binary_tree_algorithm::inorder_traversal_94(test_root), expected_ret);
}

#[test]
pub fn test_build_tree_105() {
    let test_preorder: Vec<i32> = vec![3,9,20,15,7];
    let test_inorder: Vec<i32> = vec![9,3,15,20,7];
    assert!(binary_tree_algorithm::is_same_tree_100(binary_tree_algorithm::build_tree_105(test_preorder, test_inorder), binary_tree_algorithm::create_binary_tree(vec![3,9,20,i32::MIN,i32::MIN,15,7])));
    let test_preorder: Vec<i32> = vec![-1];
    let test_inorder: Vec<i32> = vec![-1];
    assert!(binary_tree_algorithm::is_same_tree_100(binary_tree_algorithm::build_tree_105(test_preorder, test_inorder), binary_tree_algorithm::create_binary_tree(vec![-1])));
}

#[test]
pub fn test_build_tree_106() {
    let test_inorder: Vec<i32> = vec![9,3,15,20,7];
    let test_postorder: Vec<i32> = vec![9,15,7,20,3];
    assert!(binary_tree_algorithm::is_same_tree_100(binary_tree_algorithm::build_tree_106(test_inorder, test_postorder), binary_tree_algorithm::create_binary_tree(vec![3,9,20,i32::MIN,i32::MIN,15,7])));
    let test_inorder: Vec<i32> = vec![-1];
    let test_postorder: Vec<i32> = vec![-1];
    assert!(binary_tree_algorithm::is_same_tree_100(binary_tree_algorithm::build_tree_106(test_inorder, test_postorder), binary_tree_algorithm::create_binary_tree(vec![-1])));
}

#[test]
pub fn test_level_order_bottom_107() {
    let test_root = binary_tree_algorithm::create_binary_tree(vec![3,9,20,i32::MIN,i32::MIN,15,7]);
    let expected_ret: Vec<Vec<i32>> = vec![vec![15,7], vec![9,20], vec![3]];
    assert_eq!(binary_tree_algorithm::level_order_bottom_107(test_root), expected_ret);
    let test_root = binary_tree_algorithm::create_binary_tree(vec![1]);
    let expected_ret: Vec<Vec<i32>> = vec![vec![1]];
    assert_eq!(binary_tree_algorithm::level_order_bottom_107(test_root), expected_ret);
}

#[test]
pub fn test_sorted_array_to_bst_108() {
    let test_nums: Vec<i32> = vec![-10,-3,0,5,9];
    let expected_ret = binary_tree_algorithm::create_binary_tree(vec![0,-10,5,i32::MIN,-3,i32::MIN,9]);
    assert!(binary_tree_algorithm::is_same_tree_100(binary_tree_algorithm::sorted_array_to_bst_108(test_nums), expected_ret));
    let test_nums: Vec<i32> = vec![1,3];
    let expected_ret = binary_tree_algorithm::create_binary_tree(vec![1,i32::MIN,3]);
    assert!(binary_tree_algorithm::is_same_tree_100(binary_tree_algorithm::sorted_array_to_bst_108(test_nums), expected_ret));
}

#[test]
pub fn test_preorder_traversal_145() {
    let test_nums: Vec<i32> = vec![1,2,3,4,5,i32::MIN,8,i32::MIN,i32::MIN,6,7,i32::MIN,i32::MIN,9];
    let expected_ret: Vec<i32> = vec![1,2,4,5,6,7,3,8,9];
    assert_eq!(binary_tree_algorithm::preorder_traversal_144(binary_tree_algorithm::create_binary_tree(test_nums)), expected_ret);
}

#[test]
pub fn test_postorder_traversal_145() {
    let test_nums: Vec<i32> = vec![1,2,3,4,5,i32::MIN,8,i32::MIN,i32::MIN,6,7,i32::MIN,i32::MIN,9];
    let expected_ret: Vec<i32> = vec![4,6,7,5,2,9,8,3,1];
    assert_eq!(binary_tree_algorithm::postorder_traversal_145(binary_tree_algorithm::create_binary_tree(test_nums)), expected_ret);
}

#[test]
pub fn test_num_trees_96() {
    let test_n: i32 = 3;
    let expected_ret: i32 = 5;
    assert_eq!(binary_tree_algorithm::num_trees_96(test_n), expected_ret);
    let test_n: i32 = 17;
    let expected_ret: i32 = 129644790;
    assert_eq!(binary_tree_algorithm::num_trees_96(test_n), expected_ret);
}

#[test]
pub fn test_path_sum_113() {
    let test_nums: Vec<i32> = vec![5,4,8,11,i32::MIN,13,4,7,2,i32::MIN,i32::MIN,i32::MIN,i32::MIN,5,1];
    let root = binary_tree_algorithm::create_binary_tree(test_nums);
    let test_target_num = 22;
    let expected_ret: Vec<Vec<i32>> = vec![vec![5,4,11,2], vec![5,8,4,5]];
    assert_eq!(binary_tree_algorithm::path_sum_113(root, test_target_num), expected_ret);
    let test_nums: Vec<i32> = vec![-2,i32::MIN,-3];
    let root = binary_tree_algorithm::create_binary_tree(test_nums);
    let test_target_num = -5;
    let expected_ret: Vec<Vec<i32>> = vec![vec![-2,-3]];
    assert_eq!(binary_tree_algorithm::path_sum_113(root, test_target_num), expected_ret);
}

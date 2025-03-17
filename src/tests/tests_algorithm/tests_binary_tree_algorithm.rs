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

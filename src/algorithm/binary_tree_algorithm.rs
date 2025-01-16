use std::rc::Rc;
use std::cell::RefCell;
use crate::structure::treenode::TreeNode;

///
/// Leetcode 100
///
pub fn is_same_tree_100(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
    if p == None && q == None {
        return true;
    } else if (p == None && q != None) || (p != None && q == None) {
        return false;
    } else {
        let p_rc = p.unwrap();
        let q_rc = q.unwrap();
        let p_ref = p_rc.borrow();
        let q_ref = q_rc.borrow();
        return (p_ref.val) == (q_ref.val)
        && is_same_tree_100(p_ref.left.clone(), q_ref.left.clone())
         && is_same_tree_100(p_ref.right.clone(), q_ref.right.clone());
    }
}

///
/// Leetcode 112
///
pub fn has_path_sum_112(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
    if root == None {
        return false;
    } else {
        let root_rc = root.unwrap();
        let root_ref = root_rc.borrow();
        if root_ref.left == None && root_ref.right == None {
            return root_ref.val == target_sum;
        } else {
            return has_path_sum_112(root_ref.left.clone(), target_sum - root_ref.val) || has_path_sum_112(root_ref.right.clone(), target_sum - root_ref.val);
        }
    }
}


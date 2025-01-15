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


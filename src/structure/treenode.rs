use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    TreeNode {
      val,
      left: None,
      right: None
    }
  }
}

///
/// Leetcode 173
///
struct BSTIterator {
    pub vals: Vec<i32>,
    pub current_index: i32,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl BSTIterator {
    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        let mut vals_vec: Vec<i32> = vec![];
        fn inner_constructor(node: Option<Rc<RefCell<TreeNode>>>,vals_vec: &mut Vec<i32>) {
            if node == None {
                return;
            }
            let node_rc = node.unwrap();
            let node_ref = node_rc.borrow();
            inner_constructor(node_ref.left.clone(), vals_vec);
            vals_vec.push(node_ref.val);
            inner_constructor(node_ref.right.clone(), vals_vec);
        }
        inner_constructor(root, &mut vals_vec);
        BSTIterator {
            vals: vals_vec,
            current_index: -1
        }
    }

    fn next(&mut self) -> i32 {
        self.current_index += 1;
        if self.current_index as usize >= self.vals.len() {
            return i32::MIN;
        }
        return self.vals[self.current_index as usize];
    }

    fn has_next(&self) -> bool {
        if (self.current_index + 1) as usize >= self.vals.len() {
            return false;
        }
        return true;
    }
}

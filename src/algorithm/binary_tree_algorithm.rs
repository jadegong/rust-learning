use std::rc::Rc;
use std::cell::RefCell;
use crate::structure::treenode::TreeNode;

///
/// Create a binary tree from i32 Vec
/// DONE  nums: Vec<i32>
///
pub fn create_binary_tree(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    fn inner_create_binary_tree(nums: &Vec<i32>, num_index: usize) -> Option<Rc<RefCell<TreeNode>>> {
        let nums_len = nums.len();
        if num_index >= nums_len {
            return None;
        }
        if nums[num_index] == i32::MIN {
            return None;
        }
        let root = Rc::new(RefCell::new(TreeNode::new(nums[num_index])));
        root.borrow_mut().left = inner_create_binary_tree(&nums, (num_index + 1) * 2 - 1);
        root.borrow_mut().right = inner_create_binary_tree(&nums, (num_index + 1) * 2);
        Some(root)
    }
    inner_create_binary_tree(&nums, 0)
}

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

///
/// Leetcode 101
///
pub fn is_symmetric_101(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    fn inner_is_symmetric(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if (p == None && q != None) || (q == None && p != None) {
            return false;
        }
        if p == None && q == None {
            return true;
        }
        let p_rc = p.as_ref().unwrap();
        let q_rc = q.as_ref().unwrap();
        let p_ref = p_rc.borrow();
        let q_ref = q_rc.borrow();
        return p_ref.val == q_ref.val && inner_is_symmetric(p_ref.left.clone(), q_ref.right.clone()) && inner_is_symmetric(p_ref.right.clone(), q_ref.left.clone());
    }
    if root == None {
        return true;
    } else {
        let root_rc = root.as_ref().unwrap();
        let root_ref = root_rc.borrow();
        return inner_is_symmetric(root_ref.left.clone(), root_ref.right.clone());
    }
}

///
/// Leetcode 129
///
pub fn sum_numbers_129(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut nums_vec: Vec<i32> = vec![];
    let mut sum = 0;
    fn inner_sum_numbers(node: Option<Rc<RefCell<TreeNode>>>, sum: &mut i32, nums_vec: &mut Vec<i32>) {
        if node == None {
            return;
        }
        let node_rc = node.as_ref().unwrap();
        let node_ref = node_rc.borrow();
        nums_vec.push(node_ref.val);
        if node_ref.left == None && node_ref.right == None {
            // Leaf node, add sum
            let mut index = 0;
            let nums_vec_len = nums_vec.len();
            let mut current_sum = 0;
            while index < nums_vec_len {
                current_sum = current_sum * 10 + nums_vec[index];
                index += 1;
            }
            *sum += current_sum;
        } else {
            inner_sum_numbers(node_ref.left.clone(), sum, nums_vec);
            inner_sum_numbers(node_ref.right.clone(), sum, nums_vec);
        }
        nums_vec.pop();
    }
    inner_sum_numbers(root.clone(), &mut sum, &mut nums_vec);
    return sum;
}

///
/// Leetcode 114
/// Given the root of a binary tree, flatten the tree into a "linked list"
///
pub fn flattern_114(root: &mut Option<Rc<RefCell<TreeNode>>>) {
    fn inner_flattern(node: &mut Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if *node == None {
            return node.clone();
        }
        let node_rc = node.as_ref().unwrap();
        let mut node_ref = node_rc.borrow_mut();
        if node_ref.left == None && node_ref.right == None {
            return node.clone();
        }
        let node_left_next: Option<Rc<RefCell<TreeNode>>>;
        let node_right_next: Option<Rc<RefCell<TreeNode>>>;
        let mut node_left_mut = node_ref.left.clone();
        let mut node_right_mut = node_ref.right.clone();
        if node_ref.left != None {
            node_left_next = inner_flattern(&mut node_left_mut);
            if node_ref.right != None {
                node_right_next = inner_flattern(&mut node_right_mut);
                // node_left_next.left = None; node_left_next.right = node_ref.right;
                let node_left_next_rc = node_left_next.as_ref().unwrap();
                let mut node_left_next_ref = node_left_next_rc.borrow_mut();
                node_left_next_ref.left = None;
                node_left_next_ref.right = node_right_mut;
                node_ref.left = None;
                node_ref.right = node_left_mut;
                return node_right_next;
            } else {
                node_ref.left = None;
                node_ref.right = node_left_mut;
                return node_left_next;
            }
        } else {
            node_right_next = inner_flattern(&mut node_right_mut);
            node_ref.right = node_right_mut;
            return node_right_next;
        }
    }
    inner_flattern(root);
}

//
// Leetcode 124
//
// pub fn max_path_sum_124(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    // fn inner_single_max_path(node: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        // let mut ret = i32::MIN;
        // if node == None {
            // return vec![i32::MIN, i32::MIN];
        // } else {
            // let node_rc = node.unwrap();
            // let node_ref = node_rc.borrow();
            // if node_ref.left == None && node_ref.right == None {
                // return vec![node_ref.val, i32::MIN];
            // }
            // let left_ret = inner_single_max_path(node_ref.left.clone());
            // let right_ret = inner_single_max_path(node_ref.right.clone());
            // ret = std::cmp::max(left_ret[0], right_ret[0]);
            // if node_ref.val < 0 {
                // return vec![node_ref.val + ret, ret];
            // } else {
                // return vec![node_ref.val + ret, i32::MIN];
            // }
        // }
    // }
    // fn inner_max_path_sum(node: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        // let mut max_ret = i32::MIN;
        // let mut max_through_node = i32::MIN;
        // if node == None {
            // return max_ret;
        // } else {
            // let node_rc = node.unwrap();
            // let node_ref = node_rc.borrow();
            // if node_ref.left == None && node_ref.right == None {
                // max_through_node = node_ref.val;
            // } else {
                // let left_single_max = inner_single_max_path(node_ref.left.clone());
                // let right_single_max = inner_single_max_path(node_ref.right.clone());
                // if left_single_max == i32::MIN {
                    // max_through_node = std::cmp::max(node_ref.val + right_single_max, node_ref.val);
                // } else if right_single_max == i32::MIN {
                    // max_through_node = node_ref.val + left_single_max;
                // } else {
                    // max_through_node = node_ref.val + left_single_max + right_single_max;
                // }
            // }
            // let left_max_path = inner_max_path_sum(node_ref.left.clone());
            // let right_max_path = inner_max_path_sum(node_ref.right.clone());
            // return std::cmp::max(std::cmp::max(std::cmp::max(left_max_path, right_max_path), max_through_node), max_ret);
        // }
    // }
    // return inner_max_path_sum(root);
// }

/// 
/// Leetcode 222
///
pub fn count_nodes_222(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    if root == None {
        return 0;
    }
    let root_rc = root.unwrap();
    let root_ref = root_rc.borrow();
    let mut node_left = root_ref.left.clone();
    let mut node_right = root_ref.right.clone();
    let mut left_height = 0;
    let mut right_height = 0;
    while node_left != None {
        left_height += 1;
        node_left = node_left.unwrap().borrow().left.clone();
    }
    while node_right != None {
        right_height += 1;
        node_right = node_right.unwrap().borrow().right.clone();
    }
    if left_height == right_height { // Perfect binary tree, this will reduce time complexity O(logn)^2
        return i32::pow(2, left_height + 1) - 1;
    } else {
        return 1 + count_nodes_222(root_ref.left.clone()) + count_nodes_222(root_ref.right.clone());
    }
}

///
/// Leetcode 199
///
pub fn right_side_view_199(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    fn inner_right_side_view(node: Option<Rc<RefCell<TreeNode>>>, vals: &mut Vec<i32>, depth: &mut usize) {
        if node == None {
            return;
        }
        let prev_depth = *depth;
        *depth += 1;
        let node_rc = node.unwrap();
        let node_ref = node_rc.borrow();
        if *depth > vals.len() {
            vals.push(node_ref.val);
        } else {
            vals[*depth - 1] = node_ref.val;
        }
        inner_right_side_view(node_ref.left.clone(), vals, depth);
        *depth = prev_depth + 1;
        inner_right_side_view(node_ref.right.clone(), vals, depth);
    }
    let mut vals_vec: Vec<i32> = vec![];
    let mut depth: usize = 0;
    if root == None {
        return vals_vec;
    }
    inner_right_side_view(root, &mut vals_vec, &mut depth);
    return vals_vec;
}

///
/// Leetcode 236
///
pub fn lowest_common_ancestor_236(root: Option<Rc<RefCell<TreeNode>>>, p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    // The path from root to node: every node val
    fn inner_find_node(node: Option<Rc<RefCell<TreeNode>>>, dest_node: Option<Rc<RefCell<TreeNode>>>, path_vec: &mut Vec<i32>, flag: &mut bool) {
        if *flag == true {
            return;
        }
        if node == None {
            return;
        }
        let node_rc = node.unwrap();
        let node_ref = node_rc.borrow();
        let dest_node_rc = dest_node.clone().unwrap();
        let dest_node_ref = dest_node_rc.borrow();
        path_vec.push(node_ref.val);
        if node_ref.val == dest_node_ref.val {
            *flag = true;
        } else {
            inner_find_node(node_ref.left.clone(), dest_node.clone(), path_vec, flag);
            if *flag {
                return;
            }
            inner_find_node(node_ref.right.clone(), dest_node.clone(), path_vec, flag);
            if *flag {
                return;
            }
            path_vec.pop();
        }
    }
    let mut flag_p = false;
    let mut path_vec_p: Vec<i32> = vec![];
    inner_find_node(root.clone(), p, &mut path_vec_p, &mut flag_p);
    let mut flag_q = false;
    let mut path_vec_q: Vec<i32> = vec![];
    inner_find_node(root.clone(), q, &mut path_vec_q, &mut flag_q);
    if !flag_p || !flag_q {
        return None;
    }
    let mut index = 1;
    let mut ret_node: Option<Rc<RefCell<TreeNode>>> = root.clone();
    loop {
        if index >= path_vec_p.len() || index >= path_vec_q.len() {
            break;
        }
        if path_vec_p[index] != path_vec_q[index] {
            break;
        }
        let ret_node_rc = ret_node.clone().unwrap();
        let ret_node_ref = ret_node_rc.borrow();
        let ret_node_left = ret_node_ref.left.clone();
        let ret_node_right = ret_node_ref.right.clone();
        if ret_node_left == None {
            ret_node = ret_node_right.clone();
            index += 1;
            continue;
        }
        let ret_node_left_rc = ret_node_left.as_ref().unwrap();
        let ret_node_left_ref = ret_node_left_rc.borrow();
        if ret_node_right == None {
            ret_node = ret_node_left.clone();
            index += 1;
            continue;
        }
        let ret_node_right_rc = ret_node_right.as_ref().unwrap();
        let ret_node_right_ref = ret_node_right_rc.borrow();
        if ret_node_left_ref.val == path_vec_p[index] {
            ret_node = ret_node_left.clone();
            index += 1;
        } else if ret_node_right_ref.val == path_vec_p[index] {
            ret_node = ret_node_right.clone();
            index += 1;
        } else {
            break;
        }
    }
    return ret_node;
}

///
/// Leetcode 637
///
pub fn average_of_levels_637(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<f64> {
    // sums: sums of every level, nums: nums of every level
    fn inner_sum_of_levels(node: Option<Rc<RefCell<TreeNode>>>, sums: &mut Vec<i64>, nums: &mut Vec<i64>, depth: &mut usize) {
        if node == None {
            return;
        }
        let prev_depth = *depth;
        *depth += 1;
        let node_rc = node.unwrap();
        let node_ref = node_rc.borrow();
        if *depth > sums.len() {
            sums.push(node_ref.val as i64);
            nums.push(1);
        } else {
            sums[*depth - 1] += node_ref.val as i64;
            nums[*depth - 1] += 1;
        }
        inner_sum_of_levels(node_ref.left.clone(), sums, nums, depth);
        *depth = prev_depth + 1;
        inner_sum_of_levels(node_ref.right.clone(), sums, nums, depth);
    }
    let mut sums_vec: Vec<i64> = vec![];
    let mut nums_vec: Vec<i64> = vec![];
    let mut ret_vec: Vec<f64> = vec![];
    let mut depth = 0;
    if root == None {
        return ret_vec;
    }
    inner_sum_of_levels(root.clone(), &mut sums_vec, &mut nums_vec, &mut depth);
    let mut index = 0;
    while index < sums_vec.len() {
        ret_vec.push((sums_vec[index] as f64) / (nums_vec[index] as f64));
        index += 1;
    }
    return ret_vec;
}

///
/// Leetcode 102
///
pub fn level_order_102(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    fn inner_level_order(node: Option<Rc<RefCell<TreeNode>>>, vals: &mut Vec<Vec<i32>>, depth: &mut usize) {
        if node == None {
            return;
        }
        let prev_depth = *depth;
        *depth += 1;
        let node_rc = node.unwrap();
        let node_ref = node_rc.borrow();
        if *depth > vals.len() {
            vals.push(vec![node_ref.val]);
        } else {
            vals[*depth - 1].push(node_ref.val);
        }
        inner_level_order(node_ref.left.clone(), vals, depth);
        *depth = prev_depth + 1;
        inner_level_order(node_ref.right.clone(), vals, depth);
    }

    let mut vals: Vec<Vec<i32>> = vec![];
    let mut depth = 0;
    inner_level_order(root, &mut vals, &mut depth);
    return vals;
}

///
/// Leetcode 103
///
pub fn zigzag_level_order_103(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    fn inner_zigzag_level_order(node: Option<Rc<RefCell<TreeNode>>>, vals: &mut Vec<Vec<i32>>, depth: &mut usize) {
        if node == None {
            return;
        }
        let prev_depth = *depth;
        *depth += 1;
        let node_rc = node.unwrap();
        let node_ref = node_rc.borrow();
        if *depth > vals.len() {
            vals.push(vec![node_ref.val]);
        } else {
            if *depth % 2 == 0 {
                vals[*depth - 1].insert(0, node_ref.val);
            } else {
                vals[*depth - 1].push(node_ref.val);
            }
        }
        inner_zigzag_level_order(node_ref.left.clone(), vals, depth);
        *depth = prev_depth + 1;
        inner_zigzag_level_order(node_ref.right.clone(), vals, depth);
    }

    let mut vals: Vec<Vec<i32>> = vec![];
    let mut depth = 0;
    inner_zigzag_level_order(root, &mut vals, &mut depth);
    return vals;
}

///
/// Leetcode 530
///
pub fn get_minimum_difference_530(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    fn inner_generate_preorder(node: Option<Rc<RefCell<TreeNode>>>, nums_vec: &mut Vec<i32>) {
        if node == None {
            return;
        }
        let node_rc = node.unwrap();
        let node_ref = node_rc.borrow();
        inner_generate_preorder(node_ref.left.clone(), nums_vec);
        nums_vec.push(node_ref.val);
        inner_generate_preorder(node_ref.right.clone(), nums_vec);
    }
    let mut nums_vec: Vec<i32> = vec![];
    inner_generate_preorder(root, &mut nums_vec);
    let mut ret = i32::MAX;
    let mut index = 1;
    let nums_len = nums_vec.len();
    let mut prev_num = nums_vec[0];
    while index < nums_len {
        ret = i32::min(ret, nums_vec[index] - prev_num);
        prev_num = nums_vec[index];
        index += 1;
    }
    ret
}

///
/// Leetcode 98
///
pub fn is_valid_bst_98(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    fn inner_generate_preorder(node: Option<Rc<RefCell<TreeNode>>>, nums_vec: &mut Vec<i32>) {
        if node == None {
            return;
        }
        let node_rc = node.unwrap();
        let node_ref = node_rc.borrow();
        inner_generate_preorder(node_ref.left.clone(), nums_vec);
        nums_vec.push(node_ref.val);
        inner_generate_preorder(node_ref.right.clone(), nums_vec);
    }
    if root == None {
        return true;
    }
    let mut nums_vec: Vec<i32> = vec![];
    inner_generate_preorder(root, &mut nums_vec);
    let mut ret = true;
    let mut index = 1;
    let nums_len = nums_vec.len();
    let mut prev_num = nums_vec[0];
    while index < nums_len {
        if nums_vec[index] <= prev_num {
            ret = false;
            break;
        }
        prev_num = nums_vec[index];
        index += 1;
    }
    ret
}

///
/// Leetcode 230
///
pub fn kth_smallest_230(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
    // Preorder
    fn inner_generate_preorder(node: Option<Rc<RefCell<TreeNode>>>, nums_vec: &mut Vec<i32>) {
        if node == None {
            return;
        }
        let node_rc = node.unwrap();
        let node_ref = node_rc.borrow();
        inner_generate_preorder(node_ref.left.clone(), nums_vec);
        nums_vec.push(node_ref.val);
        inner_generate_preorder(node_ref.right.clone(), nums_vec);
    }
    let mut nums_vec: Vec<i32> = vec![];
    inner_generate_preorder(root, &mut nums_vec);
    return nums_vec[(k - 1) as usize];
}

/// 
/// Leetcode 94
/// Binary Tree Inorder Traversal
///
pub fn inorder_traversal_94(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    // Inorder
    fn inner_generate_inorder(node: Option<Rc<RefCell<TreeNode>>>, nums_vec: &mut Vec<i32>) {
        if node == None {
            return;
        }
        let node_rc = node.unwrap();
        let node_ref = node_rc.borrow();
        inner_generate_inorder(node_ref.left.clone(), nums_vec);
        nums_vec.push(node_ref.val);
        inner_generate_inorder(node_ref.right.clone(), nums_vec);
    }
    let mut nums_vec: Vec<i32> = vec![];
    inner_generate_inorder(root, &mut nums_vec);
    nums_vec
}

/// 
/// Leetcode 105
/// Construct Binary Tree from Preorder and Inorder Traversal
///
pub fn build_tree_105(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    let mut pre_map: std::collections::HashMap<i32, i32> = std::collections::HashMap::new();
    let mut in_map: std::collections::HashMap<i32, i32> = std::collections::HashMap::new();
    let mut index = 0;
    let tree_len = preorder.len();
    while index < tree_len {
        pre_map.insert(preorder[index], index as i32);
        in_map.insert(inorder[index], index as i32);
        index += 1;
    }
    // i: the index of root in preorder,
    // left: the left boundary of current tree in inorder, 
    // right: the boundary of current tree in inorder.
    fn inner_dfs(preorder: &Vec<i32>, in_map: &std::collections::HashMap<i32, i32>, i: i32, left: i32, right: i32) -> Option<Rc<RefCell<TreeNode>>> {
        if right - left < 0 {
            return None;
        }
        // Root node
        let root = Rc::new(RefCell::new(TreeNode::new(preorder[i as usize])));
        // Find root in inorder, to build left and right
        let root_index_inorder = in_map.get(&preorder[i as usize]).unwrap();
        // build left
        root.borrow_mut().left = inner_dfs(preorder, in_map, i + 1, left, *root_index_inorder - 1);
        // root_index_inorder - left is the length of left tree
        root.borrow_mut().right = inner_dfs(preorder, in_map, i + 1 + *root_index_inorder - left, *root_index_inorder + 1, right);
        Some(root)
    }
    inner_dfs(&preorder, &in_map, 0, 0, (tree_len - 1) as i32)
}

///
/// Leetcode 106
/// Construct Binary Tree from Inorder and Postorder Traversal
///
pub fn build_tree_106(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    let mut in_map: std::collections::HashMap<i32, i32> = std::collections::HashMap::new();
    let mut index = 0;
    let tree_len = postorder.len();
    while index < tree_len {
        in_map.insert(inorder[index], index as i32);
        index += 1;
    }
    // i: the index of root in postorder,
    // left: the left boundary of current tree in inorder, 
    // right: the boundary of current tree in inorder.
    fn inner_dfs(postorder: &Vec<i32>, in_map: &std::collections::HashMap<i32, i32>, i: i32, left: i32, right: i32) -> Option<Rc<RefCell<TreeNode>>> {
        if right - left < 0 {
            return None;
        }
        // Root node
        let root = Rc::new(RefCell::new(TreeNode::new(postorder[i as usize])));
        // Find root in inorder, to build left and right
        let root_index_inorder = in_map.get(&postorder[i as usize]).unwrap();
        // right - root_index_inorder is the length of right tree
        root.borrow_mut().left = inner_dfs(postorder, in_map, i - 1 - (right - *root_index_inorder), left, *root_index_inorder - 1);
        // build right
        root.borrow_mut().right = inner_dfs(postorder, in_map, i - 1, *root_index_inorder + 1, right);
        Some(root)
    }
    inner_dfs(&postorder, &in_map, (tree_len - 1) as i32, 0, (tree_len - 1) as i32)
}

/// 
/// Leetcode 107
/// Binary Tree Level Order Traversal II
///
pub fn level_order_bottom_107(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    fn inner_level_order_bottom(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        if root == None {
            return vec![];
        }
        let root_rc = root.unwrap();
        let root_ref = root_rc.borrow();
        let left_vec = inner_level_order_bottom(root_ref.left.clone());
        let right_vec = inner_level_order_bottom(root_ref.right.clone());
        // Combine left and right
        let left_len = left_vec.len();
        let right_len = right_vec.len();
        let mut index = 0;
        let mut current_vec: Vec<Vec<i32>> = vec![];
        let mut left_arr: Vec<i32>;
        let mut right_arr: Vec<i32>;
        while index < left_len || index < right_len {
            if left_len < right_len {
                if index < right_len - left_len {
                    current_vec.push(right_vec[index].clone());
                } else {
                    left_arr = left_vec[index - (right_len - left_len)].clone();
                    right_arr = right_vec[index].clone();
                    left_arr.append(&mut right_arr);
                    current_vec.push(left_arr);
                }
            } else {
                if index < left_len - right_len {
                    current_vec.push(left_vec[index].clone());
                } else {
                    left_arr = left_vec[index].clone();
                    right_arr = right_vec[index - (left_len - right_len)].clone();
                    left_arr.append(&mut right_arr);
                    current_vec.push(left_arr);
                }
            }
            index += 1;
        }
        // Add current node val
        current_vec.push(vec![root_ref.val]);
        current_vec
    }
    inner_level_order_bottom(root)
}

/// 
/// Leetcode 108
/// Convert Sorted Array to Binary Search Tree
///
pub fn sorted_array_to_bst_108(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    // left: left boundary;
    // right: right boundary;
    fn inner_sorted_array_tobst(nums: &Vec<i32>, left: i32, right: i32) -> Option<Rc<RefCell<TreeNode>>> {
        if right < left {
            return None;
        }
        let root_index = (left + right) / 2;
        let root = Rc::new(RefCell::new(TreeNode::new(nums[root_index as usize])));
        root.borrow_mut().left = inner_sorted_array_tobst(nums, left, root_index - 1);
        root.borrow_mut().right = inner_sorted_array_tobst(nums, root_index + 1, right);
        Some(root)
    }
    inner_sorted_array_tobst(&nums, 0, (nums.len() - 1) as i32)
}

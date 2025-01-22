use crate::structure::linked_list::ListNode;

///
/// Create a linked_list from i32 Vec
/// return the head
/// ``
/// use crate::structure::linked_list::ListNode;
/// use crate::algorithm::linked_list_algorithm;
///
/// let nums: Vec<i32> = vec![1, 2, 3];
/// let head: Option<Box<ListNode>> = linked_list_algorithm::create_linked_list(nums);
/// ``
///
pub fn create_linked_list(nums: Vec<i32>) -> Option<Box<ListNode>> {
    let mut ret: Option<Box<ListNode>> = None;
    let mut prev_node = &mut ret;
    let mut index = 0;
    let nums_len = nums.len();
    while index < nums_len {
        let node = Box::new(ListNode::new(nums[index]));
        prev_node = &mut prev_node.insert(node).next;
        index += 1;
    }
    ret
}

///
/// Compare two linked_list if equal
///
pub fn compare_two_linked_list(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> bool {
    let mut l1_node = l1.clone();
    let mut l2_node = l2.clone();
    let mut ret = true;
    while l1_node != None && l2_node != None {
        if (l1_node == None && l2_node != None) || (l1_node != None && l2_node == None) { // One is None
            ret = false;
            break;
        } else { // Both are not None
            if l1_node.as_mut().unwrap().val != l2_node.as_mut().unwrap().val {
                ret = false;
                break;
            } else {
                l1_node = l1_node.unwrap().next;
                l2_node = l2_node.unwrap().next;
            }
        }
    }
    ret
}

///
/// Leetcode 2
///
pub fn add_two_numbers_2(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut l1_vec: Vec<i32> = vec![];
    let mut l1_node = l1.clone();
    while l1_node != None {
        let l1_node_box = l1_node.clone().unwrap();
        let l1_node_val = l1_node_box.as_ref();
        l1_vec.push(l1_node_val.val);
        l1_node = l1_node_val.next.clone();
    }
    let mut l2_vec: Vec<i32> = vec![];
    let mut l2_node = l2.clone();
    while l2_node != None {
        let l2_node_box = l2_node.clone().unwrap();
        let l2_node_val = l2_node_box.as_ref();
        l2_vec.push(l2_node_val.val);
        l2_node = l2_node_val.next.clone();
    }
    let l1_len = l1_vec.len();
    let l2_len = l2_vec.len();
    let mut ret_vec: Vec<i32> = vec![];
    let mut index = 0;
    let mut add_one = 0;
    loop {
        if index >= l1_len && index >= l2_len {
            break;
        } else {
            let current_val;
            if index >= l1_len && index < l2_len { // Only calc l2
                current_val = l2_vec[index] + add_one;
            } else if index >= l2_len && index < l1_len { // Only calc l1
                current_val = l1_vec[index] + add_one;
            } else {
                current_val = l1_vec[index] + l2_vec[index] + add_one;
            }
            ret_vec.push(current_val % 10);
            add_one = current_val / 10;
        }
        index += 1;
    }
    if add_one > 0 {
        ret_vec.push(add_one);
    }
    let ret_vec_len = ret_vec.len();
    index = ret_vec_len;
    let mut ret_node = ListNode::new(ret_vec[index - 1]);
    index -= 1;
    while index > 0 {
        let temp_node = ret_node.clone();
        ret_node = ListNode::new(ret_vec[index - 1]);
        ret_node.next = Some(Box::new(temp_node));
        index -= 1;
    }
    return Some(Box::new(ret_node));
}

///
/// Leetcode 92
///
pub fn reverse_between_92(head: Option<Box<ListNode>>, left: i32, right: i32) -> Option<Box<ListNode>> {
    let mut current_index = 0;
    let mut current_node: Option<Box<ListNode>> = head.clone();
    let mut reverse_vec: Vec<i32> = vec![];
    while current_node != None {
        let current_val = current_node.as_mut().unwrap().val;
        if current_index < left - 1 || current_index > right - 1 { // Not in left -> right
            reverse_vec.push(current_val);
        } else {
            reverse_vec.insert((left - 1) as usize, current_val);
        }
        current_index += 1;
        current_node = current_node.unwrap().next;
    }
    let mut ret: Option<Box<ListNode>> = None;
    let mut prev_node = &mut ret;
    current_index = 0;
    let vec_len = reverse_vec.len();
    while current_index < (vec_len as i32) {
        let node = Box::new(ListNode::new(reverse_vec[current_index as usize]));
        prev_node = &mut prev_node.insert(node).next;
        current_index += 1;
    }
    ret
}

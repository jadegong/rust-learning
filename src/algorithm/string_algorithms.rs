///
/// Leetcode 58
///
pub fn length_of_last_word_58(s: String) -> i32 {
    let mut ret_len = 0;
    let mut should_recalc = false;
    for (_, c) in s.chars().enumerate() {
        if c == ' ' {
            should_recalc = true;
        } else {
            if should_recalc {
                ret_len = 1;
            } else {
                ret_len += 1;
            }
            should_recalc = false;
        }
    }
    return ret_len;
}

///
/// Leetcode 14
///
pub fn longest_common_prefix_14(strs: Vec<String>) -> String {
    let strs_len = strs.len();
    let mut min_len = strs[0].len();
    let mut index = 1;
    while index < strs_len {
        if strs[index].len() <  min_len {
            min_len = strs[index].len();
        }
        index += 1;
    }
    index = 0;
    let mut ret_str = String::new();
    'min_len_loop: loop {
        if index >= min_len {
            break 'min_len_loop;
        }
        let mut is_same = true;
        let mut strs_len_index = 1;
        let current_char = strs[0].get(index..(index+1)).unwrap();
        // loop to judge if char at index is same
        'strs_len_loop: loop {
            if strs_len_index >= strs_len {
                break 'strs_len_loop;
            }
            if current_char != strs[strs_len_index].get(index..(index+1)).unwrap() {
                is_same = false;
                break 'strs_len_loop;
            }
            strs_len_index += 1;
        }
        if is_same {
            ret_str.push_str(current_char);
            index += 1;
        } else {
            break 'min_len_loop;
        }
    }
    return ret_str;
}

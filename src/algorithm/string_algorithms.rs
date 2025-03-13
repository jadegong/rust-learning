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

///
/// Leetcode 151 reverse words use single space
///
pub fn reverse_words_151(s: String) -> String {
    let words: Vec<&str> = s.trim().split(" ").collect();
    let words_len = words.len();
    let mut ret_str = String::new();
    let mut index = 0;
    while index < words_len {
        if words[words_len - 1 - index] != "" {
            if ret_str.len() > 0 {
                ret_str.push_str(" ");
            }
            ret_str.push_str(words[words_len - 1 - index]);
        }
        index += 1;
    }
    return ret_str;
}

///
/// Leetcode 28 Find the Index of the First Occurrence in a String
/// KMP string searching
///
pub fn str_str_28(haystack: String, needle: String) -> i32 {
    let haystack_chars: Vec<char> = haystack.chars().collect();
    let needle_chars: Vec<char> = needle.chars().collect();
    let needle_len = needle_chars.len();
    // needle_kmp[i] means when first needle_kmp[i] and last needle_kmp[i] chars are the same in first i + 1 chars of needle
    // eg: ABCAB , needle_kmp[3], pick first 3+1 chars ABCA, the max match is first 1 char and last 1 char A, so needle_kmp[3] = 1
    let mut needle_kmp: Vec<usize> = vec![];
    let mut index = 0;
    let mut sub_needle = String::new();
    while index < needle_len { // loop needle_len
        sub_needle.push(needle_chars[index]);
        // sub needle length index + 1
        let mut current_kmp_v = 0;
        for sub_index in (0..index).rev() {
            // first sub_index+1 chars equal to last sub_index+1 chars
            if sub_needle.get(0..(sub_index+1)).unwrap() == sub_needle.get((index-sub_index)..(index+1)).unwrap() {
                // get needle_kmp[index]
                current_kmp_v = sub_index+1;
                break;
            }
        }
        needle_kmp.push(current_kmp_v);
        // loop for every first index + 1 chars
        index += 1;
    }
    index = 0;
    let mut has_match = false;
    let haystack_len = haystack.len();
    while index < haystack_len {
        let mut sub_index = 0;
        let mut match_len = 0;
        while sub_index < needle_len && index + sub_index < haystack_len {
            if haystack_chars[index + sub_index] == needle_chars[sub_index] {
                match_len += 1;
                sub_index += 1;
            } else {
                break;
            }
        }
        if match_len == needle_len {
            has_match = true;
            break;
        } else if match_len == 0 {
            index += 1;
        } else {
            // index move right according to kmp
            index += match_len - needle_kmp[match_len - 1];
        }
    }
    if !has_match {
        return -1;
    }
    return index as i32;
}

///
/// Leetcode 125
/// Valid Palindrome
///
pub fn is_palindrome_125(s: String) -> bool {
    let chars_arr: Vec<char> = s.chars().collect();
    let mut palin_arr: Vec<char> = vec![];
    let mut s_len = chars_arr.len();
    let mut left_index = 0;
    while left_index < s_len {
        if chars_arr[left_index].is_ascii_digit() || chars_arr[left_index].is_ascii_uppercase() || chars_arr[left_index].is_ascii_lowercase() {
            palin_arr.push(chars_arr[left_index].to_ascii_uppercase());
        }
        left_index += 1;
    }
    left_index = 0;
    s_len = palin_arr.len();
    if s_len <= 0 {
        return true;
    }
    let mut right_index = s_len - 1;
    let middle_index = s_len / 2;
    while left_index <= middle_index && right_index >= middle_index {
        if palin_arr[left_index] != palin_arr[right_index] {
            return false;
        }
        left_index += 1;
        right_index -= 1;
    }
    return true;
}

///
/// Leetcode 392
/// Is Subsequence
///
pub fn is_subsequence_392(s: String, t: String) -> bool {
    let s_len = s.len();
    let t_len = t.len();
    if s_len > t_len {
        return false;
    }
    let mut s_index = 0;
    let mut t_index = 0;
    let s_chars: Vec<char> = s.chars().collect();
    let t_chars: Vec<char> = t.chars().collect();
    while s_index < s_len && t_index < t_len {
        if s_chars[s_index] == t_chars[t_index] {
            s_index += 1;
        }
        t_index += 1;
    }
    if s_index == s_len {
        return true;
    } else {
        return false;
    }
}

///
/// Leetcode 3
/// Given a string s, find the length of the longes substring without repeating characters.
///
pub fn length_of_longest_substring_3(s: String) -> i32 {
    if s.len() == 0 {
        return 0;
    }
    let mut longest_len_ret = std::i32::MIN;
    let s_vec: Vec<char> = s.chars().collect();
    let mut left_index = 0;
    let mut right_index = 0;
    let mut current_sub_len = 0;
    let s_len = s.len();
    while right_index < s_len {
        // Find current right_index in s_vec
        let mut index = left_index;
        let mut found = false;
        while index < right_index {
            if s_vec[index] == s_vec[right_index] {
                found = true;
                break;
            }
            index += 1;
        }
        if found {
            left_index = index + 1;
            current_sub_len = (right_index - left_index + 1) as i32; // Recalc current sub len
        } else {
            current_sub_len += 1;
        }
        longest_len_ret = std::cmp::max(longest_len_ret, current_sub_len);
        right_index += 1;
    }
    return longest_len_ret;
}

///
/// Leetcode 76
/// Minimum Window Substring
/// 
pub fn min_window_76(s: String, t: String) -> String {
    let mut min_window_len = std::usize::MAX;
    let mut min_window_left_index = 0;
    let mut left_index = 0;
    let mut right_index = 0;
    let s_chars: Vec<char> = s.chars().collect();
    let t_chars: Vec<char> = t.chars().collect();
    let t_len = t.len();
    let s_len = s.len();
    let mut char_nums: std::collections::HashMap<char, i32> = std::collections::HashMap::new(); // Save the count of every char in t
    let mut index = 0;
    let mut t_count = t_len;
    while index < t_len {
        if char_nums.contains_key(&t_chars[index]) {
            let old_n = char_nums.get(&t_chars[index]).unwrap();
            char_nums.insert(t_chars[index], *old_n + 1);
        } else {
            char_nums.insert(t_chars[index], 1);
        }
        index += 1;
    }
    while right_index < s_len {
        // move right_index by right one step util include all t string
        if char_nums.contains_key(&s_chars[right_index]) { // No record useless chars
            let old_n = char_nums.get(&s_chars[right_index]).unwrap();
            if *old_n > 0 {
                t_count -= 1;
            }
            char_nums.insert(s_chars[right_index], *old_n - 1);
        }
        while t_count == 0 {
            if right_index - left_index + 1 < min_window_len {
                min_window_len = right_index - left_index + 1;
                min_window_left_index = left_index;
            }
            // Atemp move left_index by right one
            if char_nums.contains_key(&s_chars[left_index]) { // No record useless chars
                let old_n = char_nums.get(&s_chars[left_index]).unwrap();
                if *old_n == 0 { // Need another char left_index
                    t_count += 1;
                }
                char_nums.insert(s_chars[left_index], *old_n + 1);
            }
            left_index += 1;
        }
        right_index += 1;
    }
    if min_window_len == std::usize::MAX {
        return "".to_string();
    }
    return String::from_iter(&s_chars[min_window_left_index..(min_window_left_index + min_window_len)]);
}

///
/// Leetcode 383
///
pub fn can_construct_383(ransom_note: String, magazine: String) -> bool {
    let magazine_chars: Vec<char> = magazine.chars().collect();
    let ransom_note_chars: Vec<char> = ransom_note.chars().collect();
    let mut ransom_note_chars_count = ransom_note.len();
    let magazine_len = magazine.len();
    let mut char_nums: std::collections::HashMap<char, i32> = std::collections::HashMap::new();
    let mut index = 0;
    while index < ransom_note_chars_count {
        if char_nums.contains_key(&ransom_note_chars[index]) {
            let old_n = char_nums.get(&ransom_note_chars[index]).unwrap();
            char_nums.insert(ransom_note_chars[index], *old_n + 1);
        } else {
            char_nums.insert(ransom_note_chars[index], 1);
        }
        index += 1;
    }
    index = 0;
    while index < magazine_len {
        if char_nums.contains_key(&magazine_chars[index]) {
            let old_n = char_nums.get(&magazine_chars[index]).unwrap();
            if *old_n > 0 {
                ransom_note_chars_count -= 1;
            }
            char_nums.insert(magazine_chars[index], *old_n - 1);
        }
        index += 1;
    }
    if ransom_note_chars_count == 0 {
        return true;
    }
    return false;
}

/// 
/// Leetcode 205
///
pub fn is_isomorphic_205(s: String, t: String) -> bool {
    let mut s_t_map: std::collections::HashMap<char, char> = std::collections::HashMap::new();
    let mut t_s_map: std::collections::HashMap<char, char> = std::collections::HashMap::new();
    let mut ret = true;
    let n = s.len();
    let s_chars: Vec<char> = s.chars().collect();
    let t_chars: Vec<char> = t.chars().collect();
    let mut index = 0;
    while index < n {
        if s_t_map.contains_key(&s_chars[index]) {
            let map_v = s_t_map.get(&s_chars[index]).unwrap();
            if *map_v != t_chars[index] {
                ret = false;
                break;
            }
        } else {
            if t_s_map.contains_key(&t_chars[index]) {
                ret = false;
                break;
            } else {
                t_s_map.insert(t_chars[index], s_chars[index]);
                s_t_map.insert(s_chars[index], t_chars[index]);
            }
        }
        index += 1;
    }
    return ret;
}

///
/// Leetcode 290
///
pub fn word_pattern_290(pattern: String, s: String) -> bool {
    let pattern_chars: Vec<char> = pattern.chars().collect();
    let s_words: Vec<&str> = s.split(' ').collect();
    let pattern_len = pattern_chars.len();
    let s_words_len = s_words.len();
    if pattern_len != s_words_len {
        return false;
    }
    let mut p_s_map: std::collections::HashMap<char, &str> = std::collections::HashMap::new();
    let mut s_p_map: std::collections::HashMap<&str, char> = std::collections::HashMap::new();
    let mut index = 0;
    let mut ret: bool = true;
    while index < pattern_len {
        if p_s_map.contains_key(&pattern_chars[index]) {
            let p_s_map_v = p_s_map.get(&pattern_chars[index]).unwrap();
            if *p_s_map_v != s_words[index] {
                ret = false;
                break;
            }
        } else {
            if s_p_map.contains_key(&s_words[index]) {
                ret = false;
                break;
            } else {
                p_s_map.insert(pattern_chars[index], s_words[index]);
                s_p_map.insert(s_words[index], pattern_chars[index]);
            }
        }
        index += 1;
    }
    return ret;
}

///
/// Leetcode 242
///
pub fn valid_anagram_242(s: String, t: String) -> bool {
    let s_len = s.len();
    let t_len = t.len();
    if s_len != t_len {
        return false;
    }
    let s_chars: Vec<char> = s.chars().collect();
    let t_chars: Vec<char> = t.chars().collect();
    let mut char_count_map: std::collections::HashMap<char, i32> = std::collections::HashMap::new();
    let mut index = 0;
    while index < s_len {
        if char_count_map.contains_key(&s_chars[index]) {
            let current_count = char_count_map.get(&s_chars[index]).unwrap();
            char_count_map.insert(s_chars[index], *current_count + 1);
        } else {
            char_count_map.insert(s_chars[index], 1);
        }
        index += 1;
    }
    let mut ret = true;
    index = 0;
    while index < t_len {
        if char_count_map.contains_key(&t_chars[index]) {
            let current_count = char_count_map.get(&t_chars[index]).unwrap();
            if *current_count == 0 {
                ret = false;
                break;
            } else if *current_count == 1 {
                char_count_map.remove(&t_chars[index]);
            } else {
                char_count_map.insert(t_chars[index], *current_count - 1);
            }
        } else {
            ret = false;
            break;
        }
        index += 1;
    }
    return ret;
}

/// 
/// Leetcode 20
///
pub fn is_valid_20(s: String) -> bool {
    let s_len = s.len();
    let s_chars: Vec<char> = s.chars().collect();
    let mut stack_vec: Vec<char> = vec![];
    let mut ret = true;
    let mut index = 0;
    while index < s_len {
        if s_chars[index] == '{' || s_chars[index] == '(' || s_chars[index] == '[' {
            stack_vec.push(s_chars[index]);
        } else if s_chars[index] == '}' {
            if stack_vec.len() == 0 {
                ret = false;
                break;
            } else {
                if stack_vec.pop().unwrap() != '{' {
                    ret = false;
                    break;
                }
            }
        } else if s_chars[index] == ']' {
            if stack_vec.len() == 0 {
                ret = false;
                break;
            } else {
                if stack_vec.pop().unwrap() != '[' {
                    ret = false;
                    break;
                }
            }
        } else if s_chars[index] == ')' {
            if stack_vec.len() == 0 {
                ret = false;
                break;
            } else {
                if stack_vec.pop().unwrap() != '(' {
                    ret = false;
                    break;
                }
            }
        } else {
            ret = false;
            break;
        }
        index += 1;
    }
    if stack_vec.len() > 0 {
        ret = false;
    }
    return ret;
}

/// 
/// Leetcode 2696
/// Minimum String Length After Removing Substrings
///
pub fn min_length_2696(s: String) -> i32 {
    let s_chars: Vec<char> = s.chars().collect();
    let s_len = s.len();
    let mut stack_vec: Vec<char> = vec![];
    let mut index = 0;
    let mut ret: usize = 0; // Stack len
    while index < s_len {
        if s_chars[index] == 'B' {
            if ret == 0 {
                stack_vec.push(s_chars[index]);
                ret += 1;
            } else if stack_vec[ret - 1] != 'A' { // Stack top not 'A'
                stack_vec.push(s_chars[index]);
                ret += 1;
            } else {
                stack_vec.pop();
                ret -= 1;
            }
        } else if s_chars[index] == 'D' {
            if ret == 0 {
                stack_vec.push(s_chars[index]);
                ret += 1;
            } else if stack_vec[ret - 1] != 'C' { // Stack top not 'C'
                stack_vec.push(s_chars[index]);
                ret += 1;
            } else {
                stack_vec.pop();
                ret -= 1;
            }
        } else {
            stack_vec.push(s_chars[index]);
            ret += 1;
        }
        index += 1;
    }
    ret as i32
}

/// 
/// Leetcode 921
/// Minimum Add To Make Parentheses Valid
///
pub fn min_add_to_make_valid_921(s: String) -> i32 {
    let s_chars: Vec<char> = s.chars().collect();
    let s_len = s.len();
    let mut stack_vec: Vec<char> = vec![];
    let mut index = 0;
    let mut ret = 0; // stack len
    while index < s_len {
        if s_chars[index] == ')' {
            if ret == 0 {
                stack_vec.push(s_chars[index]);
                ret += 1;
            } else if stack_vec[ret - 1] != '(' {
                stack_vec.push(s_chars[index]);
                ret += 1;
            } else { // ret > 0 && stack top is '(', current char is ')'
                stack_vec.pop();
                ret -= 1;
            }
        } else {
            stack_vec.push(s_chars[index]);
            ret += 1;
        }
        index += 1;
    }
    ret as i32
}

/// 
/// Leetcode 3174
/// Clear Digits
///
pub fn clear_digits_3174(s: String) -> String {
    let s_chars: Vec<char> = s.chars().collect();
    let mut stack_vec: Vec<char> = vec![];
    let s_len = s.len();
    let mut index = 0;
    let mut stack_vec_len = 0;
    while index < s_len {
        if s_chars[index] >= '0' && s_chars[index] <= '9' {
            if stack_vec_len == 0 {
                stack_vec.push(s_chars[index]);
                stack_vec_len += 1;
            } else if stack_vec[stack_vec_len - 1] >= '0' && stack_vec[stack_vec_len - 1] <= '9' {
                stack_vec.push(s_chars[index]);
                stack_vec_len += 1;
            } else {
                stack_vec.pop();
                stack_vec_len -= 1;
            }
        } else {
            stack_vec.push(s_chars[index]);
            stack_vec_len += 1;
        }
        index += 1;
    }
    String::from_iter(stack_vec)
}

///
/// Leetcode 1422
/// Maximum Score After Splitting a String
///
pub fn max_score_1422(s: String) -> i32 {
    let s_len = s.len();
    if s_len <= 1 {
        return s_len as i32;
    }
    let s_chars: Vec<char> = s.chars().collect();
    let mut one_count = 0;
    let mut index = 0;
    while index < s_len {
        if s_chars[index] == '1' {
            one_count += 1;
        }
        index += 1;
    }
    index = 0; // end index of left string
    let mut zero_count = 0;
    let mut ret = 0;
    while index < s_len - 1 {
        if s_chars[index] == '0' {
            zero_count += 1;
        } else {
            one_count -= 1;
        }
        ret = std::cmp::max(ret, zero_count + one_count);
        index += 1;
    }
    ret
}

/// 
/// Leetcode 2379
/// Minimum Recolors to Get K Consecutive Black Blocks
///
pub fn minimum_recolors_2379(blocks: String, k: i32) -> i32 {
    let s_len = blocks.len();
    let s_chars: Vec<char> = blocks.chars().collect();
    let mut index = 0;
    let mut max_b_count = 0;
    let mut current_b_count = 0;
    while index < k as usize {
        if s_chars[index] == 'B' {
            current_b_count += 1;
            max_b_count += 1;
        }
        index += 1;
    }
    while index < s_len {
        if s_chars[index] == 'B' {
            current_b_count += 1;
        }
        if s_chars[index - (k as usize)] == 'B' {
            current_b_count -= 1;
        }
        max_b_count = std::cmp::max(max_b_count, current_b_count);
        index += 1;
    }
    k - max_b_count
}

/// 
/// Leetcode 43
/// Multiply Strings
///
pub fn multiply_43(num1: String, num2: String) -> String {
    let num1_chars: Vec<char> = num1.chars().collect();
    let num2_chars: Vec<char> = num2.chars().collect();
    let num1_len = num1_chars.len();
    let num2_len = num2_chars.len();
    let mut multiply_tab: Vec<Vec<u32>> = vec![];
    let mut num1_index = 0;
    let mut num2_index;
    let mut current_num1_digit;
    let mut current_num2_digit;
    while num1_index < num1_len {
        let mut current_tab_row: Vec<u32> = vec![];
        current_num1_digit = num1_chars[num1_index].to_digit(10).unwrap();
        num2_index = 0;
        while num2_index < num2_len {
            current_num2_digit = num2_chars[num2_index].to_digit(10).unwrap();
            current_tab_row.push(current_num1_digit * current_num2_digit);
            num2_index += 1;
        }
        multiply_tab.push(current_tab_row);
        num1_index += 1;
    }
    num1_index = num1_len;
    let mut loop_num1_index;
    let mut loop_num2_index;
    let mut current_line_digit;
    let mut ret = String::from("");
    let mut add_digit = 0;
    while num1_index > 0 {
        loop_num1_index = num1_index;
        loop_num2_index = num2_len;
        current_line_digit = 0;
        while loop_num1_index <= num1_len && loop_num2_index > 0 {
            current_line_digit += multiply_tab[loop_num1_index - 1][loop_num2_index - 1];
            loop_num2_index -= 1;
            loop_num1_index += 1;
        }
        current_line_digit += add_digit;
        ret = (current_line_digit % 10).to_string() + &ret;
        add_digit = current_line_digit / 10;
        num1_index -= 1;
    }
    num2_index = num2_len - 1;
    while num2_index > 0 {
        loop_num1_index = 1;
        loop_num2_index = num2_index;
        current_line_digit = 0;
        while loop_num1_index <= num1_len && loop_num2_index > 0 {
            current_line_digit += multiply_tab[loop_num1_index - 1][loop_num2_index - 1];
            loop_num2_index -= 1;
            loop_num1_index += 1;
        }
        current_line_digit += add_digit;
        ret = (current_line_digit % 10).to_string() + &ret;
        add_digit = current_line_digit / 10;
        num2_index -= 1;
    }
    while add_digit > 0 {
        ret = (add_digit % 10).to_string() + &ret;
        add_digit /= 10;
    }
    // remove starting zeroes
    let ret_len = ret.len();
    let ret_chars: Vec<char> = ret.chars().collect();
    num1_index = 0;
    let mut result: String = String::from("");
    let mut start_zero: bool = true;
    while num1_index < ret_len {
        if ret_chars[num1_index] != '0' {
            start_zero = false;
        }
        if !start_zero {
            result += &ret_chars[num1_index].to_string();
        }
        num1_index += 1;
    }
    if result.len() == 0 {
        return String::from("0");
    }
    result
}

/// 
/// Leetcode 67
/// Add Binary
///
pub fn add_binary_67(a: String, b: String) -> String {
    let a_chars: Vec<char> = a.chars().collect();
    let b_chars: Vec<char> = b.chars().collect();
    let a_len = a_chars.len();
    let b_len = b_chars.len();
    let mut index = 0;
    let mut add_digit = 0;
    let mut current_digit;
    let mut ret: String = String::from("");
    while index < a_len || index < b_len {
        if index >= a_len {
            current_digit = b_chars[b_len - 1 - index].to_digit(10).unwrap() + add_digit;
        } else if index >= b_len {
            current_digit = a_chars[a_len - 1 - index].to_digit(10).unwrap() + add_digit;
        } else {
            current_digit = a_chars[a_len - 1 - index].to_digit(10).unwrap() + b_chars[b_len - 1 - index].to_digit(10).unwrap() + add_digit;
        }
        ret = (current_digit % 2).to_string() + &ret;
        add_digit = current_digit / 2;
        index += 1;
    }
    while add_digit > 0 {
        ret = (add_digit % 2).to_string() + &ret;
        add_digit /= 2;
    }
    ret
}

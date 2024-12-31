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


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

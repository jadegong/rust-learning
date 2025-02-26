///
/// Leetcode 190
/// Reverse Bits
///
pub fn reverse_bits_190(x: u32) -> u32 {
    let mut current_num = x;
    let mut current_bit: u32;
    let mut ret = 0;
    let mut index = 0;
    while index < 32 {
        current_bit = current_num % 2;
        ret = ret * 2 + current_bit;
        current_num >>= 1;
        index += 1;
    }
    ret
}

///
/// Leetcode 191
/// Number of 1 Bits
///
pub fn hamming_weight_191(n: i32) -> i32 {
    let mut current_num = n;
    let mut ret: i32 = 0;
    while current_num > 0 {
        if current_num % 2 == 1 {
            ret += 1;
        }
        current_num >>= 1;
    }
    ret
}

///
/// Leetcode 136
/// Single Number
///
pub fn single_number_136(nums: Vec<i32>) -> i32 {
    let nums_len = nums.len();
    let mut index = 0;
    let mut ret = 0;
    while index < nums_len {
        ret ^= nums[index];
        index += 1;
    }
    ret
}

///
/// Leetcode 137
/// Single Number II
///
pub fn single_number_137(nums: Vec<i32>) -> i32 {
    let nums_len = nums.len();
    if nums_len == 0 {
        return 0;
    }
    // let mut num_of_bits: Vec<i32> = vec![0; 32];
    let mut num_of_bit = 0;
    let mut num_index = 0;
    let mut bit_index = 0;
    let mut ret = 0;
    while bit_index < 32 {
        while num_index < nums_len {
            if nums[num_index] >> bit_index & 1 == 1 {
                num_of_bit += 1;
            }
            num_index += 1;
        }
        if num_of_bit % 3 != 0 {
            ret += 1 << bit_index;
        }
        num_of_bit = 0;
        num_index = 0;
        bit_index += 1;
    }
    ret
}

/// 
/// Leetcode 9
/// Palindrome Number
///
pub fn is_palindrome_9(x: i32) -> bool {
    if x < 0 {
        return false;
    }
    let mut nums: Vec<i32> = vec![];
    let mut num = x;
    while num > 0 {
        nums.push(num % 10);
        num /= 10;
    }
    let nums_len = nums.len();
    if nums_len <= 1 {
        return true;
    }
    let mut index = 0;
    let middle_index = nums_len / 2;
    let mut ret = true;
    while index < middle_index {
        if nums[index] != nums[nums_len - index - 1] {
            ret = false;
            break;
        }
        index += 1;
    }
    ret
}

/// 
/// Leetcode 66
/// Plus One
///
pub fn plus_one_66(digits: Vec<i32>) -> Vec<i32> {
    let digits_len = digits.len();
    let mut add_num = 0; // If add one to next digit
    let mut index = 0;
    let mut ret_nums: Vec<i32> = vec![];
    let mut current_sum;
    while index < digits_len {
        current_sum = add_num + digits[digits_len - 1 - index];
        if index == 0 {
            current_sum += 1;
        }
        add_num = current_sum / 10;
        ret_nums.push(current_sum % 10);
        index += 1;
    }
    if add_num != 0 {
        ret_nums.push(add_num);
    }
    ret_nums.reverse();
    ret_nums
}

/// 
/// Leetcode 172
/// Factorial Trailing Zeroes
///
pub fn trailing_zeroes_172(n: i32) -> i32 {
    let mut ret = 0;
    let mut five_pow = 5;
    while five_pow <= n {
        ret += n / five_pow;
        five_pow *= 5;
    }
    ret
}

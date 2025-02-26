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

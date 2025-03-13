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

/// 
/// Leetcode 2425
/// Bitwise XOR of All Parings
///
pub fn xor_all_nums_2425(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
    let mut ret = 0;
    let nums1_len = nums1.len();
    let nums2_len = nums2.len();
    let mut index;
    if nums2_len % 2 == 1 {
        index = 0;
        while index < nums1_len {
            ret ^= nums1[index];
            index += 1;
        }
    }
    if nums1_len % 2 == 1 {
        index = 0;
        while index < nums2_len {
            ret ^= nums2[index];
            index += 1;
        }
    }
    ret
}

/// 
/// Leetcode 2579
/// Count Total Number of Colored Cells
/// Method 2: f(n) = n*n + (n-1)*(n-1)
pub fn colored_cells_2579(n: i32) -> i64 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }
    let mut ret: i64 = 1;
    let mut index: i64 = 2;
    while index <= n as i64 {
        ret = 2 * index + 2*(index - 2) + ret;
        index += 1;
    }
    ret
}

/// 
/// Leetcode 1780
/// Check if Number is a Sum of Powers of Three
///
pub fn check_powers_of_three_1780(n: i32) -> bool {
    // 1.find the maxmum x, pow(3, x) <= n
    let mut x = 0;
    let mut x_pow = 1;
    loop {
        if x_pow > n {
            break;
        }
        x += 1;
        x_pow *= 3;
    }
    x -= 1;
    x_pow /= 3;
    // 2.Loop from x to 0, n minus x_pow if x_pow <= current_n, repeat.
    let mut current_n = n;
    loop {
        if current_n <= 0 || x < 0 {
            break;
        }
        if current_n >= x_pow {
            current_n -= x_pow;
        }
        x_pow /= 3;
        x -= 1;
    }
    return current_n == 0
}

/// 
/// Leetcode 69
/// Sqrt(x)
/// Newton Iteration
///
pub fn my_sqrt_69(x: i32) -> i32 {
    let f_x: f64 = f64::from(x);
    let mut current_l: f64 = f_x;
    let mut current_w: f64 = f_x / current_l;
    let err: f64 = 0.00001;
    while current_l - current_w > err {
        current_l = (current_l + current_w) / 2.0;
        current_w = f_x / current_l;
    }
    current_l.floor() as i32
}

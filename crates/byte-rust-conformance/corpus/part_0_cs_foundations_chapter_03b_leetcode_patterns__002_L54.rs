use std::cmp::max;

// Tìm độ dài của mảng con liên tiếp dài nhất có tổng <= max_sum
pub fn longest_subarray_sum(nums: &[i32], max_sum: i32) -> usize {
    let mut max_len = 0;
    let mut current_sum = 0;
    let mut left = 0;

    for right in 0..nums.len() {
        current_sum += nums[right];

        // Shrink the window if current_sum is too large
        while current_sum > max_sum && left <= right {
            current_sum -= nums[left];
            left += 1;
        }

        max_len = max(max_len, right - left + 1);
    }

    max_len
}

fn main() {}

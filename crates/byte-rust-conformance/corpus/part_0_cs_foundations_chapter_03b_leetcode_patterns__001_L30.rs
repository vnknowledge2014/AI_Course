// ❌ Cách Imperative (Giống C/C++)
pub fn two_sum_imperative(nums: &[i32], target: i32) -> Option<(usize, usize)> {
    let mut left = 0;
    let mut right = nums.len() - 1;

    while left < right {
        let sum = nums[left] + nums[right];
        if sum == target {
            return Some((left, right));
        } else if sum < target {
            left += 1;
        } else {
            right -= 1; // Khá rủi ro nếu right = 0 do kiểu usize
        }
    }
    None
}

fn main() {}

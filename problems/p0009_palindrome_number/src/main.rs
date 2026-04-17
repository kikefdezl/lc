struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }
        let mut x = x;
        let mut nums = Vec::new();
        while x > 0 {
            nums.push(x % 10);
            x /= 10;
        }

        for i in 0..nums.len() {
            if nums[i] != nums[nums.len() - 1 - i] {
                return false;
            }
        }
        true
    }
}

fn main() {
    assert!(Solution::is_palindrome(1221));
    assert!(!Solution::is_palindrome(1223));
    assert!(!Solution::is_palindrome(-121));
}

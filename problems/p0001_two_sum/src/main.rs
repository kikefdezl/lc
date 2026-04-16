use std::collections::HashMap;

struct Solution;

// T O(n) - S O(n) hashmap solution
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut cache = HashMap::new();
        for (i, num) in nums.iter().enumerate() {
            if let Some(j) = cache.get(&(target - num)) {
                return vec![*j as i32, i as i32];
            } else {
                cache.insert(num, i);
            };
        }
        unreachable!();
    }
}

// T O(nlogn) - S O(1) binary search solution
impl Solution {
    pub fn two_sum_binary(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut indices: Vec<usize> = (0..nums.len()).collect();
        indices.sort_by_key(|&i| nums[i]);

        let sorted: Vec<i32> = indices.iter().map(|i| nums[*i]).collect();

        for i in 0..sorted.len() - 1 {
            let mut lo = i + 1;
            let mut hi = sorted.len() - 1;
            while lo <= hi {
                let mid = lo + ((hi - lo) / 2);
                let sum = sorted[i] + sorted[mid];
                if sum == target {
                    return vec![indices[i] as i32, indices[mid] as i32];
                } else if sum > target {
                    hi = mid - 1;
                } else {
                    lo = mid + 1;
                }
            }
        }
        unreachable!()
    }
}

fn main() {
    assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
    assert_eq!(Solution::two_sum(vec![3, 2, 4], 6), vec![1, 2]);
    assert_eq!(Solution::two_sum(vec![2, 5, 5, 11], 10), vec![1, 2]);
    assert_eq!(Solution::two_sum_binary(vec![2, 7, 11, 15], 9), vec![0, 1]);
    assert_eq!(Solution::two_sum_binary(vec![3, 2, 4], 6), vec![1, 2]);
    assert_eq!(Solution::two_sum_binary(vec![2, 5, 5, 11], 10), vec![1, 2]);
}

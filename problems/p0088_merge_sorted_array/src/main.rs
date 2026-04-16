struct Solution;

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut curr = m + n - 1;
        let mut a = m - 1;
        let mut b = n - 1;

        while a >= 0 && b >= 0 {
            if nums1[a as usize] >= nums2[b as usize] {
                nums1[curr as usize] = nums1[a as usize];
                a -= 1;
            } else {
                nums1[curr as usize] = nums2[b as usize];
                b -= 1;
            }
            curr -= 1;
        }

        while b >= 0 {
            nums1[b as usize] = nums2[b as usize];
            b -= 1;
        }
    }
}

fn main() {
    let mut vec1 = vec![1, 2, 3, 0, 0, 0];
    let mut vec2 = vec![2, 5, 6];

    Solution::merge(&mut vec1, 3, &mut vec2, 3);
    assert_eq!(vec1, vec![1, 2, 2, 3, 5, 6]);
}

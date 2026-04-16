use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let mut cache = HashMap::new();
        Solution::_climb_stairs_cached(n, &mut cache)
    }

    pub fn _climb_stairs_cached(n: i32, cache: &mut HashMap<i32, i32>) -> i32 {
        if let Some(val) = cache.get(&n) {
            return *val;
        }

        if n == 0 {
            return 0;
        } else if n == 1 {
            return 1;
        } else if n == 2 {
            return 2;
        }

        let mut count = 0;
        count += Solution::_climb_stairs_cached(n - 1, cache);
        count += Solution::_climb_stairs_cached(n - 2, cache);
        cache.insert(n, count);
        count
    }
}

fn main() {
    assert_eq!(Solution::climb_stairs(2), 2);
    assert_eq!(Solution::climb_stairs(3), 3);
}

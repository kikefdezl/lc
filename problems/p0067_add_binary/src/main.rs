pub struct Solution;

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let mut c = String::new();
        let mut carry = 0;

        let mut a_iter = a.chars().rev();
        let mut b_iter = b.chars().rev();
        loop {
            let a_c = a_iter.next();
            let b_c = b_iter.next();

            if a_c.is_none() && b_c.is_none() && carry == 0 {
                return c;
            }

            let mut s = carry;
            if let Some('1') = a_c {
                s += 1;
            }
            if let Some('1') = b_c {
                s += 1;
            }

            c.insert(0, char::from_digit(s % 2, 10).unwrap());
            carry = s / 2;
        }
    }
}

fn main() {
    assert_eq!(
        Solution::add_binary("10011".to_string(), "010".to_string()),
        "10101".to_string()
    );
}

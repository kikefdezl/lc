struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = Vec::new();
        for c in s.chars() {
            if c == '(' || c == '{' || c == '[' {
                stack.push(c);
            } else {
                let last = match stack.last() {
                    None => return false,
                    Some(l) => l,
                };
                if (c == ')' && *last != '(')
                    || (c == ']' && *last != '[')
                    || (c == '}' && *last != '{')
                {
                    return false;
                } else {
                    stack.pop();
                }
            }
        }
        if stack.is_empty() {
            return true;
        }
        false
    }
}

fn main() {
    assert!(Solution::is_valid("{}".to_string()));
    assert!(Solution::is_valid("{()}".to_string()));
    assert!(Solution::is_valid("{[]()}".to_string()));
    assert!(!Solution::is_valid("{[])}".to_string()));
    assert!(!Solution::is_valid("(".to_string()));
}

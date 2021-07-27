#[derive(Debug)]
struct Solution {}

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let x_array: Vec<char> = x.to_string().chars().collect();
        let len = x_array.len();
        for (index, j) in x_array.iter().enumerate() {
            if len - 1 == index {
                break;
            }
            if x_array[len - index - 1] == *j {
                continue;
            } else {
                return false;
            }
        }
        true
    }
}
fn main() {
    assert_eq!(false, Solution::is_palindrome(123));
    assert_eq!(true, Solution::is_palindrome(121));
    assert_eq!(true, Solution::is_palindrome(12521));
    assert_eq!(false, Solution::is_palindrome(-121));
    assert_eq!(false, Solution::is_palindrome(1211111));
}

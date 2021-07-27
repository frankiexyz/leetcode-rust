#[derive(Debug)]
struct Solution {}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        for (i, ix) in nums.iter().enumerate() {
            for (j, jx) in nums.iter().enumerate() {
                if i == j {
                    continue;
                }
                if ix + jx == target {
                    return vec![i as i32, j as i32];
                }
            }
        }
        vec![]
    }
}
fn main() {
    assert_eq!(vec![0, 1], Solution::two_sum(vec![2, 7, 11, 5], 9));
    assert_eq!(vec![1, 2], Solution::two_sum(vec![3, 2, 4], 6));
    assert_eq!(vec![0, 1], Solution::two_sum(vec![3, 3], 6));
}

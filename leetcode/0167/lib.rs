use std::cmp::Ordering;

struct Solution;
impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut left = 0;
        let mut right = numbers.len() - 1;
        while left < right {
            match (numbers[left] + numbers[right]).cmp(&target) {
                Ordering::Less => {
                    left += 1;
                }
                Ordering::Greater => {
                    right -= 1;
                }
                Ordering::Equal => {
                    return vec![(left + 1) as i32, (right + 1) as i32];
                }
            }
        }
        vec![]
    }
}

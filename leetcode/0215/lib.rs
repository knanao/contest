use std::collections::BinaryHeap;

struct Solution;
impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let mut m = BinaryHeap::new();
        nums.iter().for_each(|&x| m.push(x));

        for _ in 0..k - 1 {
            m.pop();
        }
        m.pop().unwrap()
    }
}

struct Solution;
impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let mut v = nums;
        v.sort();
        v[0]
    }
}

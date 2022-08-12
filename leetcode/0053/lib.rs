struct Solution;
impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let (mut acc, mut ret) = (nums[0], nums[0]);
        for i in 1..nums.len() {
            if acc < 0 {
                acc = nums[i];
            } else {
                acc += nums[i];
            }
            ret = std::cmp::max(ret, acc);
        }
        ret
    }
}

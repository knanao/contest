struct Solution;
impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return nums[0];
        }
        let mut ret = nums
            .iter()
            .take(nums.len() - 1)
            .fold((0, 0), |(a, b), x| (a.max(b + x), a))
            .0;
        ret = std::cmp::max(
            ret,
            nums.iter()
                .skip(1)
                .fold((0, 0), |(a, b), x| (a.max(b + x), a))
                .0,
        );
        ret
    }
}

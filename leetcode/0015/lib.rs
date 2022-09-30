use std::collections::HashMap;

struct Solution;
impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ret: Vec<Vec<i32>> = vec![];

        let mut nums = nums;
        nums.sort();

        for i in 0..(nums.len() - 2) {
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }

            if nums[i] > 0 {
                return ret;
            }

            let mut left = i + 1;
            let mut right = nums.len() - 1;

            while left < right {
                let sum = nums[left] + nums[right] + nums[i];
                if sum > 0 {
                    right -= 1;
                } else if sum < 0 {
                    left += 1;
                } else {
                    ret.push(vec![nums[left], nums[right], nums[i]]);
                    while left < right && nums[left] == nums[left + 1] {
                        left += 1;
                    }
                    while left < right && nums[right] == nums[right - 1] {
                        right -= 1;
                    }

                    left += 1;
                    right -= 1;
                }
            }
        }
        ret
    }
}

use std::collections::VecDeque;

struct Solution;
impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        /*
         * 0: true
         * 1: false
         * 2: unknown
         */
        let mut dp: Vec<usize> = vec![2; nums.len()];
        dp[nums.len() - 1] = 0;

        Self::helper(0, &nums, &mut dp)
    }

    fn helper(p: usize, nums: &Vec<i32>, dp: &mut Vec<usize>) -> bool {
        if dp[p] != 2 {
            return dp[p] == 0;
        }

        let max = std::cmp::min(p + nums[p] as usize, nums.len() - 1);
        for i in p + 1..=max {
            if Self::helper(i, nums, dp) {
                dp[p] = 0;
                return true;
            }
        }

        dp[p] = 1;
        false
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_can_jump() {
        let input = vec![2, 5, 0, 0];
        let got = Solution::can_jump(input);
        assert_eq!(true, got);
    }
}

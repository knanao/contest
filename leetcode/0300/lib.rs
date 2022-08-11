struct Solution;

impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut dp = vec![1; nums.len()];
        let mut ret: i32 = 1;
        for i in 1..nums.len() {
            for j in 0..i {
                if nums[j] < nums[i] {
                    dp[i] = std::cmp::max(dp[i], dp[j] + 1);
                }
            }
            ret = std::cmp::max(ret, dp[i]);
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_length_of_lis() {
        // example1
        println!("test1");
        let mut input = vec![10, 9, 2, 5, 3, 7, 101, 18];
        let mut got = Solution::length_of_lis(input);
        assert_eq!(got, 4);

        // example2
        println!("test2");
        input = vec![0, 1, 0, 3, 2, 3];
        got = Solution::length_of_lis(input);
        assert_eq!(got, 4);

        // example3
        println!("test3");
        input = vec![7, 7, 7, 7, 7, 7, 7];
        got = Solution::length_of_lis(input);
        assert_eq!(got, 1);

        // example4
        println!("test4");
        input = vec![4, 10, 4, 3, 8, 9];
        got = Solution::length_of_lis(input);
        assert_eq!(got, 3);
    }
}

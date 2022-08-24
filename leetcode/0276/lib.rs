struct Solution;
impl Solution {
    pub fn num_ways(n: i32, k: i32) -> i32 {
        let mut dp = vec![0; n as usize];
        if n == 0 {
            return k;
        }
        if n == 1 {
            return k * k;
        }

        dp[0] = k;
        dp[1] = k * k;
        for i in 2..n {
            dp[i as usize] = (k - 1) * (dp[(i - 1) as usize] + dp[(i - 2) as usize])
        }
        dp[(n - 1) as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_ways() {
        let mut input = (7, 2);
        let mut got = Solution::num_ways(input.0, input.1);
        assert_eq!(42, got);
    }
}

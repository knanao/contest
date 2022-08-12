struct Solution;

impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let mut dp = vec![vec![0; n as usize]; m as usize];
        for x in 0..n as usize {
            dp[0][x] = 1;
        }
        for y in 0..m as usize {
            dp[y][0] = 1;
        }

        for i in 1..m as usize {
            for j in 1..n as usize {
                dp[i][j] = dp[i - 1][j] + dp[i][j - 1];
            }
        }
        dp[m as usize - 1][n as usize - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unique_paths() {
        let mut input: (i32, i32) = (3, 7);
        let mut got = Solution::unique_paths(input.0, input.1);
        assert_eq!(28, got);

        input = (3, 2);
        got = Solution::unique_paths(input.0, input.1);
        assert_eq!(3, got);
    }
}

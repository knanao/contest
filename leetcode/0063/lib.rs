struct Solution;
impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (obstacle_grid.len(), obstacle_grid[0].len());
        if obstacle_grid[0][0] == 1 || obstacle_grid[m - 1][n - 1] == 1 {
            return 0;
        }
        let mut dp = vec![vec![0; n]; m];
        dp[0][0] = 1;
        for i in 1..m {
            if obstacle_grid[i][0] != 1 && dp[i - 1][0] != 0 {
                dp[i][0] = 1;
            }
        }
        for i in 1..n {
            if obstacle_grid[0][i] != 1 && dp[0][i - 1] != 0 {
                dp[0][i] = 1;
            }
        }

        for i in 1..m {
            for j in 1..n {
                if obstacle_grid[i][j] == 1 {
                    dp[i][j] = 0;
                } else {
                    dp[i][j] = dp[i][j - 1] + dp[i - 1][j];
                }
            }
        }
        dp[m - 1][n - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_unique_paths_with_obstacles() {
        let mut input = vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]];
        let mut got = Solution::unique_paths_with_obstacles(input);
        assert_eq!(2, got);

        input = vec![vec![0, 1], vec![0, 0]];
        got = Solution::unique_paths_with_obstacles(input);
        assert_eq!(1, got);

        input = vec![vec![0, 0], vec![1, 1], vec![0, 0]];
        got = Solution::unique_paths_with_obstacles(input);
        assert_eq!(0, got);

        input = vec![
            vec![0, 1, 0, 0, 0],
            vec![1, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0],
        ];
        got = Solution::unique_paths_with_obstacles(input);
        assert_eq!(0, got);
    }
}

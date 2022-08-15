struct Solution;
impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let mut max = amount + 1;
        let mut dp = vec![max; (amount + 1) as usize];
        dp[0] = 0;
        for i in 1..=amount {
            for j in 0..coins.len() {
                if coins[j] <= i {
                    dp[i as usize] = std::cmp::min(dp[i as usize], dp[(i - coins[j]) as usize] + 1);
                }
            }
        }
        if dp[amount as usize] > amount {
            -1
        } else {
            dp[amount as usize]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_coin_change() {
        let mut input = (vec![1, 2, 5], 11);
        let mut got = Solution::coin_change(input.0, input.1);
        assert_eq!(3, got);
    }
}

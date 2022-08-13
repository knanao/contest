struct Solution;
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        prices
            .iter()
            .fold((0, 0, 0), |(a, b, c), &v| {
                if v - b <= a || c == 0 {
                    if c == 0 {
                        (a, v, c + 1)
                    } else {
                        (a, v.min(b), c + 1)
                    }
                } else {
                    (v - b, b, c + 1)
                }
            })
            .0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_profit() {
        let mut input = vec![7, 1, 5, 3, 6, 4];
        let mut got = Solution::max_profit(input);
        assert_eq!(5, got);

        input = vec![7, 6, 4, 3, 1];
        got = Solution::max_profit(input);
        assert_eq!(0, got);

        input = vec![2, 1, 2, 1, 0, 1, 2];
        got = Solution::max_profit(input);
        assert_eq!(2, got);

        input = vec![0, 3, 8, 6, 8, 6, 6, 8, 2, 0, 2, 7];
        got = Solution::max_profit(input);
        assert_eq!(8, got);
    }
}

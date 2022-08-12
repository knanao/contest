struct Solution;
impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        nums.into_iter()
            .fold((0, 0), |(a, b), x| {
                println!("a: {}, b: {}", a, b);
                (a.max(b + x), a)
            })
            .0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rob() {
        let mut input = vec![1, 2, 3, 1];
        let mut got = Solution::rob(input);
        assert_eq!(4, got);

        input = vec![2, 7, 9, 3, 1];
        got = Solution::rob(input);
        assert_eq!(12, got);
    }
}

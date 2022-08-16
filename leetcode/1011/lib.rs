struct Solution;
impl Solution {
    pub fn ship_within_days(weights: Vec<i32>, days: i32) -> i32 {
        let mut low = weights.iter().fold(0, |acc, x| std::cmp::max(acc, *x));
        let mut high = weights.iter().sum();
        while low != high {
            let mid = (low + high) / 2;
            println!("low: {}, high: {}, mid: {}", low, high, mid);
            if Self::helper(&weights, mid) > days {
                low = mid + 1;
            } else {
                high = mid;
            }
        }
        low
    }

    fn helper(weights: &Vec<i32>, max: i32) -> i32 {
        let (mut count, mut cur) = (1, 0);
        for &w in weights.iter() {
            cur += w;
            if cur > max {
                count += 1;
                cur = w;
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ship_within_days() {
        let mut input = (vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 5);
        let mut got = Solution::ship_within_days(input.0, input.1);
        assert_eq!(15, got);
    }
}

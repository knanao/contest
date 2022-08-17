struct Solution;
impl Solution {
    // pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
    //     if nums.len() == 0 {
    //         return 0;
    //     }

    //     let mut ans = std::i32::MAX;
    //     let mut acc = nums.clone();
    //     for i in 0..nums.len() - 1 {
    //         acc[i + 1] += acc[i];
    //     }

    //     for i in 0..nums.len() {
    //         for j in i..nums.len() {
    //             let sum = acc[j] - acc[i] + nums[i];
    //             if sum >= target {
    //                 ans = std::cmp::min(ans, (j - i) as i32 + 1);
    //                 break;
    //             }
    //         }
    //     }
    //     if ans != std::i32::MAX {
    //         ans
    //     } else {
    //         0
    //     }
    // }
    //
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let (mut ans, mut i, n) = (std::i32::MAX, 0, nums.len());

        (0..n).fold(0, |mut sum, j| {
            sum += nums[j];
            while sum >= target {
                ans = ans.min((j - i) as i32 + 1);
                sum -= nums[i];
                i += 1;
            }
            sum
        });

        if ans == std::i32::MAX {
            0
        } else {
            ans
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_sub_array_len() {
        let mut input = (4, vec![1, 4, 4]);
        let mut got = Solution::min_sub_array_len(input.0, input.1);
        assert_eq!(1, got);

        input = (7, vec![2, 3, 1, 2, 4, 3]);
        got = Solution::min_sub_array_len(input.0, input.1);
        assert_eq!(2, got);

        input = (11, vec![1, 2, 3, 4, 5]);
        got = Solution::min_sub_array_len(input.0, input.1);
        assert_eq!(3, got);

        input = (15, vec![5, 1, 3, 5, 10, 7, 4, 9, 2, 8]);
        got = Solution::min_sub_array_len(input.0, input.1);
        assert_eq!(2, got);
    }
}

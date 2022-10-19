struct Solution;
impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let n = height.len();
        if n < 3 {
            return 0;
        }

        let (mut left, mut right) = (vec![0; n], vec![0; n]);
        left[0] = height[0];
        for i in 1..n {
            left[i] = std::cmp::max(left[i - 1], height[i]);
        }

        right[n - 1] = height[n - 1];
        for i in (0..n - 1).rev() {
            right[i] = std::cmp::max(right[i + 1], height[i]);
        }

        let mut ans = 0;
        for i in 0..n {
            ans += std::cmp::min(left[i], right[i]) - height[i];
        }
        ans
    }
}

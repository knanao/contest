struct Solution;
impl Solution {
    pub fn max_dist_to_closest(seats: Vec<i32>) -> i32 {
        let n = seats.len();
        let mut left = vec![n; n];
        let mut right = vec![n; n];

        for i in 0..n {
            if seats[i] == 1 {
                left[i] = 0;
            } else if i > 0 {
                left[i] = left[i - 1] + 1;
            }
        }

        for i in (0..n).rev() {
            if seats[i] == 1 {
                right[i] = 0;
            } else if i < n - 1 {
                right[i] = right[i + 1] + 1;
            }
        }

        let mut ans = 0;
        for i in 0..n {
            if seats[i] == 0 {
                ans = std::cmp::max(ans, std::cmp::min(left[i], right[i]));
            }
        }

        ans as i32
    }
}

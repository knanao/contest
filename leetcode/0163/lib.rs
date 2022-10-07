struct Solution;
impl Solution {
    pub fn find_missing_ranges(nums: Vec<i32>, lower: i32, upper: i32) -> Vec<String> {
        let mut ans = Vec::new();
        let mut prev = lower - 1;
        for i in 0..=nums.len() {
            let cur = {
                if i < nums.len() {
                    nums[i]
                } else {
                    upper + 1
                }
            };

            if prev + 1 <= cur - 1 {
                ans.push(Self::format_range(prev + 1, cur - 1));
            }
            prev = cur;
        }
        ans
    }

    fn format_range(low: i32, up: i32) -> String {
        if low == up {
            format!("{}", low)
        } else {
            format!("{}->{}", low, up)
        }
    }
}

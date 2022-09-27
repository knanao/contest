struct Solution;
impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut maxarea = 0;
        let mut left = 0;
        let mut right = height.len() - 1;
        while left < right {
            let width = (right - left) as i32;
            maxarea = std::cmp::max(maxarea, std::cmp::min(height[left], height[right]) * width);
            if height[left] <= height[right] {
                left += 1;
            } else {
                right -= 1;
            }
        }
        maxarea
    }
}

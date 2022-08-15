struct Solution;
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut n = nums.len();
        if n == 0 {
            return -1;
        }
        let mut base = 0;
        while n > 1 {
            let half = n / 2;
            let mid = base + half;
            if nums[mid] == target {
                return mid as i32;
            }
            if !(((nums[base] < nums[mid]) && (target >= nums[base] && target <= nums[mid]))
                || ((nums[base] > nums[mid]) && (target >= nums[base] || target <= nums[mid])))
            {
                base = mid;
            }
            n -= half;
        }
        if nums[base] == target {
            base as i32
        } else {
            -1
        }
    }
}

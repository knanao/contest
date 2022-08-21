struct Solution;
impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let n = nums.len();
        nums.retain(|&x| x != 0);

        for _ in 0..(n - nums.len()) {
            nums.push(0);
        }
    }
}

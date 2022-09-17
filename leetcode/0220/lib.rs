use std::collections::BTreeSet;

struct Solution;
impl Solution {
    pub fn contains_nearby_almost_duplicate(
        nums: Vec<i32>,
        index_diff: i32,
        value_diff: i32,
    ) -> bool {
        let (k, t) = (index_diff as usize, value_diff as i64);
        let mut bst: BTreeSet<i64> = BTreeSet::new();
        for i in 0..nums.len() {
            if i > k {
                bst.remove(&(nums[i - 1 - k] as i64));
            }
            if bst
                .range(nums[i] as i64 - t..=nums[i] as i64 + t)
                .next()
                .is_some()
            {
                return true;
            }
            bst.insert(nums[i] as i64);
        }
        false
    }
}

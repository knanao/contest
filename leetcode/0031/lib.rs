struct Solution;
impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        use std::cmp::Ordering;
        let last_ascending = match nums.windows(2).rposition(|w| w[0] < w[1]) {
            Some(i) => i,
            None => {
                nums.reverse();
                return;
            }
        };

        let swap_with = nums[last_ascending + 1..]
            .binary_search_by(|n| nums[last_ascending].cmp(n).then(Ordering::Greater))
            .unwrap_err();

        nums.swap(last_ascending, last_ascending + swap_with);
        nums[last_ascending + 1..].reverse();
    }

    // fn next_permutation<T>(nums: &mut [T]) -> bool
    // where
    //     T: std::cmp::Ord,
    // {
    //     use std::cmp::Ordering;
    //     let last_ascending = match nums.windows(2).rposition(|w| w[0] < w[1]) {
    //         Some(i) => i,
    //         None => {
    //             nums.reverse();
    //             return false;
    //         }
    //     };

    //     let swap_with = nums[last_ascending + 1..]
    //         .binary_search_by(|n| nums[last_ascending].cmp(n).then(Ordering::Greater))
    //         .unwrap_err();

    //     nums.swap(last_ascending, last_ascending + swap_with);
    //     nums[last_ascending + 1..].reverse();
    //     true
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_permutation() {
        let mut input = vec![1, 2, 3];
        Solution::next_permutation(&mut input);
        assert_eq!(vec![5, 1, 1], input);
    }
}

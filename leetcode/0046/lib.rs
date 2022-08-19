struct Solution;
impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut n = nums;
        n.sort();
        let mut ret = vec![n.clone()];
        while Self::next_permutation(&mut n) {
            ret.push(n.clone());
        }
        ret
    }

    fn next_permutation(nums: &mut Vec<i32>) -> bool {
        use std::cmp::Ordering;
        let last_ascending = match nums.windows(2).rposition(|w| w[0] < w[1]) {
            Some(i) => i,
            None => {
                nums.reverse();
                return false;
            }
        };

        let swap_with = nums[last_ascending + 1..]
            .binary_search_by(|n| i32::cmp(&nums[last_ascending], n).then(Ordering::Less))
            .unwrap_err();

        nums.swap(last_ascending, last_ascending + swap_with);
        nums[last_ascending + 1..].reverse();
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_permute() {
        let mut input = vec![1, 2, 3];
        let mut got = Solution::permute(input);
        assert_eq!(
            got,
            vec![
                [1, 2, 3],
                [1, 3, 2],
                [2, 1, 3],
                [2, 3, 1],
                [3, 1, 2],
                [3, 2, 1]
            ]
        );

        input = vec![0, -1, 1];
        got = Solution::permute(input);
        assert_eq!(
            got,
            vec![
                [0, -1, 1],
                [0, 1, -1],
                [-1, 0, 1],
                [-1, 1, 0],
                [1, 0, -1],
                [1, -1, 0]
            ]
        );
    }
}

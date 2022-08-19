use std::collections::HashSet;

struct Solution;
impl Solution {
    pub fn subsets(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        if nums.len() == 0 {
            return vec![vec![]];
        }

        let last = nums.pop().unwrap();
        let sub = Self::subsets(nums);
        let mut res = sub.clone();
        for mut v in sub {
            v.push(last);
            res.push(v);
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subsets() {
        let mut input = vec![1, 2, 3];
        let mut got = Solution::subsets(input);
        assert_eq!(
            vec![
                vec![],
                vec![1],
                vec![2],
                vec![1, 2],
                vec![3],
                vec![1, 3],
                vec![2, 3],
                vec![1, 2, 3]
            ],
            got
        )
    }
}

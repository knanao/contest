struct Solution;
impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut cs = candidates;
        cs.sort();
        Self::combination(&cs, 0, target)
    }

    fn combination(candidates: &Vec<i32>, prev: i32, target: i32) -> Vec<Vec<i32>> {
        if target == 0 {
            return vec![vec![]];
        }

        let mut ret = vec![];
        for &c in candidates {
            if target < c {
                break;
            }

            if prev > c {
                continue;
            }

            let reset = Self::combination(candidates, c, target - c);
            for mut v in reset {
                v.push(c);
                ret.push(v);
            }
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_combination_sum() {
        let mut input = (vec![2, 3, 6, 7], 7);
        let mut got = Solution::combination_sum(input.0, input.1);
        assert_eq!(vec![vec![2, 2, 3], vec![7]], got)
    }
}

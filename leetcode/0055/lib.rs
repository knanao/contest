use std::collections::VecDeque;

struct Solution;
impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        if nums.len() == 1 {
            return true;
        }

        let g = nums.len() - 1;
        let mut ps: VecDeque<usize> = VecDeque::new();
        ps.push_back(0);
        while !ps.is_empty() {
            println!("{:?}", ps);
            let p = ps.pop_front().unwrap();
            if p >= g {
                return true;
            }
            for i in 1..=nums[p] as usize {
                if p + i >= g {
                    return true;
                }
                if nums[p + i] == 0 {
                    continue;
                }
                ps.push_back(p + i);
            }
        }
        false
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_can_jump() {
        let input = vec![2, 5, 0, 0];
        let got = Solution::can_jump(input);
        assert_eq!(true, got);
    }
}

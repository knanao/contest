struct Solution;
impl Solution {
    /*        0
          /       \
         0          1
       /   \      /    \
       0     1    1      0
     / \     / \   / \   / \
     0  1   1   0  1  0  0  1
    */
    pub fn kth_grammar(n: i32, k: i32) -> i32 {
        if n == 1 {
            return 0;
        }

        if k % 2 == 0 {
            if Self::kth_grammar(n - 1, k / 2) == 0 {
                return 1;
            } else {
                return 0;
            }
        } else {
            if Self::kth_grammar(n - 1, (k + 1) / 2) == 0 {
                return 0;
            } else {
                return 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kth_grammar() {
        let mut input = (1, 1);
        let mut got = Solution::kth_grammar(input.0, input.1);
        assert_eq!(0, got);

        input = (2, 1);
        got = Solution::kth_grammar(input.0, input.1);
        assert_eq!(0, got);

        input = (2, 2);
        got = Solution::kth_grammar(input.0, input.1);
        assert_eq!(1, got);

        input = (3, 3);
        got = Solution::kth_grammar(input.0, input.1);
        assert_eq!(1, got);
    }
}

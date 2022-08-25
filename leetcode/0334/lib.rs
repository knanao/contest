struct Solution;
impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        Self::helper(s, 0, s.len() - 1);
    }

    fn helper(s: &mut Vec<char>, left: usize, right: usize) {
        if left >= right {
            return;
        }

        s.swap(left, right);
        Self::helper(s, left + 1, right - 1);
    }
}

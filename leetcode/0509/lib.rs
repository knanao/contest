struct Solution;
impl Solution {
    pub fn fib(n: i32) -> i32 {
        Self::helper(n, &mut vec![0; n as usize + 1])
    }

    fn helper(n: i32, memo: &mut Vec<i32>) -> i32 {
        if n == 0 || n == 1 {
            memo[n as usize] = n;
            return n;
        }

        if memo[n as usize] != 0 {
            return memo[n as usize];
        }
        memo[n as usize] = Self::helper(n - 1, memo) + Self::helper(n - 2, memo);
        memo[n as usize]
    }
}

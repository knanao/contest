struct Solution;
impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let mut cs = s.chars().skip_while(|&x| x == ' ').peekable();
        let sign = if cs.peek().map_or(false, |&x| x == '-') {
            cs.next();
            -1i32
        } else {
            if cs.peek().map_or(false, |&x| x == '+') {
                cs.next();
            }

            1i32
        };

        cs.into_iter()
            .take_while(|n| n.is_numeric())
            .try_fold(0i32, |acc, n| {
                acc.checked_mul(10)
                    .and_then(|acc| acc.checked_add(n.to_digit(10).unwrap() as i32))
            })
            .map(|n| n * sign)
            .unwrap_or(if sign > 0 {
                std::i32::MAX
            } else {
                std::i32::MIN
            })
    }
}

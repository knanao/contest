struct Solution;

impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        let (mut ret, mut a, mut b): (f64, f64, i64) = (1f64, x, n as i64);
        if n < 0 {
            b *= -1;
        }
        while b > 0 {
            if b & 1 == 1 {
                if n > 0 {
                    ret = ret * a;
                } else {
                    ret = ret * 1f64 / a;
                }
            }
            a = a * a;
            if b == -1 {
                return ret;
            }
            b >>= 1;
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_pow() {
        let mut input = (2.00000, -2);
        let mut got = Solution::my_pow(input.0, input.1);
        assert_eq!(0.25000, got);

        input = (2.00000, -2147483648);
        got = Solution::my_pow(input.0, input.1);
        assert_eq!(0.00000, got);
    }
}

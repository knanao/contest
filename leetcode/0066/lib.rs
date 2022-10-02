struct Solution;
impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut ret = digits;
        ret.reverse();
        let mut i = 0;
        while i <= ret.len() {
            ret[i] += 1;
            if ret[i] < 10 {
                break;
            }

            ret[i] = 0;
            if i == ret.len() - 1 {
                ret.push(1);
                break;
            }
            i += 1;
        }
        ret.reverse();
        ret
    }
}

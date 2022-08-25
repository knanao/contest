struct Solution;
impl Solution {
    pub fn can_attend_meetings(intervals: Vec<Vec<i32>>) -> bool {
        let mut mtgs = intervals;
        mtgs.sort_by(|a, b| a[0].cmp(&b[0]));

        let mut end = 0;
        for m in mtgs {
            if m[0] < end {
                return false;
            }
            end = m[1];
        }
        true
    }
}

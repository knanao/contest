struct Solution;
impl Solution {
    pub fn min_meeting_rooms(intervals: Vec<Vec<i32>>) -> i32 {
        let mut mtgs = intervals;
        mtgs.sort_by(|a, b| (b[1] - b[0]).cmp(&(a[1] - a[0])));

        let mut cap = vec![0i32; 1000005];
        for mtg in mtgs {
            let mut conf = false;
            for i in mtg[0]..mtg[1] {
                if cap[i as usize] != 0 {
                    conf = true;
                }
                cap[i as usize] += 1;
            }
        }
        *cap.iter().max().unwrap()
    }
}

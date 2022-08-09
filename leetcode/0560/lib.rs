use std::collections::HashMap;

impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut count = 0;
        let mut sum = 0;
        let mut map = HashMap::new();
        map.insert(0, 1);

        for i in nums {
            sum += i;
            if map.contains_key(&(sum - k)) {
                count += map.get(&(sum - k)).unwrap();
            }
            let v: i32 = {
                let mut ret = 1;
                if map.get(&sum) != None {
                    ret = map.get(&sum).unwrap() + 1
                }
                ret
            };
            map.insert(sum, v);
        }
        count
    }
}

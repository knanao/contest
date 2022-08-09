struct KthLargest {
    k: usize,
    nums: Vec<i32>,
}

impl KthLargest {
    fn new(k: i32, nums: Vec<i32>) -> Self {
        KthLargest {
            k: k as usize,
            nums: nums,
        }
    }

    fn add(&mut self, val: i32) -> i32 {
        self.nums.push(val);
        self.nums.sort_by(|a, b| b.cmp(a));
        self.nums[self.k - 1]
    }
}

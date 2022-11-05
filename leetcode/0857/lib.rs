use std::cmp::Ordering;
use std::collections::BinaryHeap;

const EPSILON: f64 = 1.0e-5;

#[derive(Debug, Default)]
struct Worker {
    ratio: f64,
    quality: i32,
    wage: i32,
}

impl Eq for Worker {}

impl PartialEq for Worker {
    fn eq(&self, other: &Self) -> bool {
        (self.ratio - other.ratio).abs() < EPSILON
    }
}

impl PartialOrd for Worker {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Worker {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.ratio - other.ratio > EPSILON {
            Ordering::Greater
        } else if other.ratio - self.ratio > EPSILON {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    }
}

struct Solution;
impl Solution {
    pub fn mincost_to_hire_workers(quality: Vec<i32>, wage: Vec<i32>, k: i32) -> f64 {
        use std::collections::BinaryHeap;

        let n = quality.len();
        let mut workers = vec![];
        for i in 0..n {
            let worker = Worker {
                ratio: wage[i] as f64 / quality[i] as f64,
                quality: quality[i],
                wage: wage[i],
            };
            workers.push(worker);
        }
        workers.sort();
        let mut ans = 1.0e9;
        let mut sum = 0;
        let mut heap = BinaryHeap::<i32>::new();
        for worker in &workers {
            heap.push(worker.quality);
            sum += worker.quality;
            if heap.len() as i32 > k {
                sum -= heap.pop().unwrap();
            }
            if heap.len() as i32 == k {
                let x = sum as f64 * worker.ratio;
                if x < ans {
                    ans = x
                }
            }
        }
        ans
    }
}

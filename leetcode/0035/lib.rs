use std::cmp::Ordering;

struct Solution;
impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        nums.lower_bound(&target)
    }
}

trait BinarySearch<T> {
    fn lower_bound(&self, x: &T) -> i32;
    fn upper_bound(&self, x: &T) -> i32;
}

impl<T: Ord> BinarySearch<T> for [T] {
    fn lower_bound(&self, x: &T) -> i32 {
        let mut low = 0;
        let mut high = self.len() as i32;

        while low != high {
            let mid = (low + high) / 2;
            match self[mid as usize].cmp(x) {
                Ordering::Less => {
                    low = mid + 1;
                }
                Ordering::Equal | Ordering::Greater => {
                    high = mid;
                }
            }
        }
        low
    }

    fn upper_bound(&self, x: &T) -> i32 {
        let mut low = 0;
        let mut high = self.len() as i32;

        while low != high {
            let mid = (low + high) / 2;
            match self[mid as usize].cmp(x) {
                Ordering::Less | Ordering::Equal => {
                    low = mid + 1;
                }
                Ordering::Greater => {
                    high = mid;
                }
            }
        }
        low
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_insert() {
        let mut input = (vec![1, 3, 5, 6], 5);
        let mut got = Solution::search_insert(input.0, input.1);
        assert_eq!(2, got);

        input = (vec![1, 3, 5, 6], 2);
        got = Solution::search_insert(input.0, input.1);
        assert_eq!(1, got);

        input = (vec![1, 3, 5, 6], 7);
        got = Solution::search_insert(input.0, input.1);
        assert_eq!(4, got);
    }
}

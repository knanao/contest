use std::collections::HashMap;

struct Solution;
impl Solution {
    pub fn total_fruit(fruits: Vec<i32>) -> i32 {
        let mut count: HashMap<i32, usize> = HashMap::new();
        let (mut res, mut i) = (0, 0);
        for j in 0..fruits.len() {
            if let Some(v) = count.get_mut(&fruits[j]) {
                *v += 1;
            } else {
                count.insert(fruits[j], 1);
            }
            while count.len() > 2 {
                count.insert(fruits[i], count.get(&fruits[i]).unwrap() - 1);
                if *count.get(&fruits[i]).unwrap() == 0 {
                    count.remove(&fruits[i]);
                }
                i += 1;
            }
            res = std::cmp::max(res, j - i + 1);
        }
        res as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total_fruit() {
        let input = vec![3, 3, 3, 1, 2, 1, 1, 2, 3, 3, 4];
        let got = Solution::total_fruit(input);
        assert_eq!(5, got);
    }
}

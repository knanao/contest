struct Solution;
impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        let mut tri: Vec<Vec<i32>> = Vec::new();
        for i in 0..=row_index as usize {
            let mut v = Vec::new();
            for j in 0..i + 1 {
                if j == 0 || j == i {
                    v.push(1);
                } else {
                    v.push(0);
                }
            }
            tri.push(v);
        }
        Solution::helper(&mut tri, 0, 0);
        tri[row_index as usize].clone()
    }

    fn helper(tri: &mut Vec<Vec<i32>>, i: usize, j: usize) {
        if tri[i][j] != 0 {
            if j < tri[i].len() - 1 {
                Self::helper(tri, i, j + 1);
            }
            if i < tri.len() - 1 {
                Self::helper(tri, i + 1, j);
            }
        } else {
            tri[i][j] = tri[i - 1][j - 1] + tri[i - 1][j];
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_row() {
        let mut input = 3;
        let mut got = Solution::get_row(input);
        println!("got: {:?}", got);
    }
}

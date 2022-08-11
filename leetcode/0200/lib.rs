use std::collections::VecDeque;

struct Solution;

impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let dx = vec![1, -1, 0, 0];
        let dy = vec![0, 0, 1, -1];
        let (m, n) = (grid.len(), grid[0].len());
        let mut seen = vec![vec![false; n]; m];
        let mut graph = VecDeque::new();
        let mut ans = 0;
        for i in 0..m {
            for j in 0..n {
                if seen[i][j] || grid[i][j] == '0' {
                    continue;
                }
                graph.push_back((i, j));
                while !graph.is_empty() {
                    let (x, y) = graph.pop_front().unwrap();
                    if seen[x][y] {
                        continue;
                    }

                    seen[x][y] = true;
                    for k in 0..4 {
                        if x as i32 + dx[k] < 0
                            || x as i32 + dx[k] >= m as i32
                            || y as i32 + dy[k] < 0
                            || y as i32 + dy[k] >= n as i32
                        {
                            continue;
                        }
                        let (px, py) = ((x as i32 + dx[k]) as usize, (y as i32 + dy[k]) as usize);
                        if seen[px][py] {
                            continue;
                        }
                        if grid[px][py] == '0' {
                            continue;
                        }
                        graph.push_back((px, py));
                    }
                }
                ans += 1;
            }
        }
        ans
    }
}

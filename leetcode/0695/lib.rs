struct Solution;

impl Solution {
    pub fn max_area_of_island(grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        let mut seen = vec![vec![false; n]; m];

        let mut ans = 0;
        for i in 0..m {
            for j in 0..n {
                if seen[i][j] || grid[i][j] != 1 {
                    continue;
                }
                let mut count = 0;
                dfs(&mut seen, &grid, i, j, &mut count);
                ans = std::cmp::max(ans, count);
            }
        }
        ans
    }
}

fn dfs(seen: &mut Vec<Vec<bool>>, graph: &Vec<Vec<i32>>, x: usize, y: usize, count: &mut i32) {
    let (m, n) = (graph.len(), graph[0].len());
    let dx = vec![1, -1, 0, 0];
    let dy = vec![0, 0, 1, -1];

    seen[x][y] = true;
    *count += 1;
    for i in 0..4 {
        if x as i32 + dx[i] < 0
            || x as i32 + dx[i] >= m as i32
            || y as i32 + dy[i] < 0
            || y as i32 + dy[i] >= n as i32
        {
            continue;
        }
        let (px, py) = ((x as i32 + dx[i]) as usize, (y as i32 + dy[i]) as usize);
        if seen[px][py] || graph[px][py] != 1 {
            continue;
        }
        dfs(seen, graph, px, py, count);
    }
}

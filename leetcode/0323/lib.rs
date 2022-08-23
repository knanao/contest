use std::collections::VecDeque;

struct Solution;
impl Solution {
    // BFS
    pub fn count_components(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let mut seen = vec![false; n as usize];
        let mut graph = VecDeque::new();
        let mut ans = 0;

        for i in 0..n {
            if seen[i as usize] {
                continue;
            }
            graph.push_back(i as usize);
            ans += 1;

            while !graph.is_empty() {
                let node = graph.pop_front().unwrap();
                if seen[node] {
                    continue;
                }
                seen[node] = true;

                for e in &edges {
                    let (s, g) = (e[0] as usize, e[1] as usize);
                    if s == node && !seen[g] {
                        graph.push_back(g);
                    }
                    if g == node && !seen[s] {
                        graph.push_back(s);
                    }
                }
            }
        }
        ans
    }

    // DFS
    // pub fn count_components(n: i32, edges: Vec<Vec<i32>>) -> i32 {
    //     let mut seen = vec![false; n as usize];
    //     let mut ans = 0;
    //     for i in 0..n {
    //         if seen[i as usize] {
    //             continue;
    //         }

    //         ans += 1;
    //         Self::dfs(&edges, &mut seen, i);
    //     }
    //     ans
    // }

    // fn dfs(edges: &Vec<Vec<i32>>, seen: &mut Vec<bool>, node: i32) {
    //     seen[node as usize] = true;
    //     for e in edges {
    //         let (s, g) = (e[0], e[1]);
    //         if s == node && !seen[g as usize] {
    //             Self::dfs(edges, seen, g);
    //         }
    //         if g == node && !seen[s as usize] {
    //             Self::dfs(edges, seen, s);
    //         }
    //     }
    // }
}

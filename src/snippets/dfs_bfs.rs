fn bfs(s: usize, G: &Vec<Vec<usize>>) -> Vec<usize> {
    // snippet begins
    use std::collections::VecDeque;
    let bfs_max = usize::MAX / 10;
    let mut dists = vec![bfs_max; G.len()];
    let mut que = VecDeque::new();
    dists[s] = 0;
    que.push_back(s);
    while let Some(s) = que.pop_front() {
        for &nxt in &G[s] {
            if dists[nxt] == bfs_max {
                dists[nxt] = dists[s] + 1;
                que.push_back(nxt);
            }
        }
    }

    // snippet ends
    dists
}

fn dfs(s: usize, G: &Vec<Vec<usize>>) -> Vec<usize> {
    // snippet begins
    use std::collections::VecDeque;
    let bfs_max = usize::MAX / 10;
    let mut dists = vec![bfs_max; G.len()];
    let mut que = VecDeque::new();
    dists[s] = 0;
    que.push_back(s);
    while let Some(s) = que.pop_back() {
        for &nxt in &G[s] {
            if dists[nxt] == bfs_max {
                dists[nxt] = dists[s] + 1;
                que.push_back(nxt);
            }
        }
    }

    // snippet ends
    dists
}

use std::{cmp::Reverse, collections::BinaryHeap};

fn dijkstra(s: usize, E: &Vec<Vec<(usize, usize)>>) -> Vec<usize> {
    let N = E.len();
    let mut dists = vec![usize::MAX / 10; N];
    dists[s] = 0;
    let mut que = BinaryHeap::<Reverse<(usize, usize)>>::new();
    que.push(Reverse((0, s)));
    while let Some(Reverse((c, s))) = que.pop() {
        if dists[s] < c {
            continue;
        }
        for &(nxt, dist) in &E[s] {
            if dists[nxt] > dists[s] + dist {
                dists[nxt] = dists[s] + dist;
                que.push(Reverse((dist, nxt)));
            }
        }
    }
    return dists;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::tester::Tester;

    #[test]
    fn solve_grl_1_a() {
        // https://onlinejudge.u-aizu.ac.jp/courses/library/3/GRL/1/GRL_1_A
        type dist_to = (usize, usize);

        let tester = Tester::new("./assets/GRL_1_A/in", "./assets/GRL_1_A/out");
        tester.solve_by_algorithm(|sc| {
            let n_v: usize = sc.get();
            let n_e: usize = sc.get();
            let s: usize = sc.get();
            let INF = usize::MAX / 10;
            let mut E = vec![vec![]; n_v];
            for _ in 0..n_e {
                let u: usize = sc.get();
                let v: usize = sc.get();
                let c: usize = sc.get();
                let dt: dist_to = (v, c);
                E[u].push(dt);
                let dt: dist_to = (u, c);
                E[v].push(dt);
            }

            let dists = dijkstra(s, &E);
            dists.into_iter().for_each(|x| {
                if x == INF {
                    sc.write(format!("{}\n", "INF"));
                } else {
                    sc.write(format!("{}\n", x));
                }
            });
        });
    }
}

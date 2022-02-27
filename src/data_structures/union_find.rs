struct UnionFindTree {
    par: Vec<usize>,
    rank: Vec<usize>,
    size: usize,
}
trait UnionFind {
    fn new(n: usize) -> UnionFindTree;
    fn find(&mut self, x: usize) -> usize;
    fn unite(&mut self, x: usize, y: usize);
    fn is_same(&mut self, x: usize, y: usize) -> bool;
}
impl UnionFind for UnionFindTree {
    fn new(n: usize) -> Self {
        let mut par = vec![0; n];
        let mut rank = vec![1; n];
        for i in 0..n {
            par[i] = i;
        }
        UnionFindTree {
            par: par,
            rank: rank,
            size: n,
        }
    }
    fn find(&mut self, x: usize) -> usize {
        if self.par[x] == x {
            x
        } else {
            self.par[x] = self.find(self.par[x]);
            self.par[x]
        }
    }
    fn unite(&mut self, x: usize, y: usize) {
        let mut x_root = self.find(self.par[x]);
        let mut y_root = self.find(self.par[y]);
        if x_root == y_root {
            return;
        }
        if self.rank[x_root] < self.rank[y_root] {
            x_root ^= y_root;
            y_root ^= x_root;
            x_root ^= y_root;
        }

        self.par[y_root] = x_root;
        self.rank[y_root] += self.rank[x_root];
    }
    fn is_same(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::tester::Tester;

    #[test]
    fn solve_dsl1_1_a() {
        // https://onlinejudge.u-aizu.ac.jp/courses/library/3/DSL/1/DSL_1_A

        let tester = Tester::new("./assets/DSL_1_A/in/", "./assets/DSL_1_A/out");
        tester.solve_by_algorithm(|sc| {
            let n = sc.get();
            let q = sc.get();
            let mut uf = UnionFindTree::new(n);
            for _ in 0..q {
                let com: usize = sc.get();
                let x = sc.get();
                let y = sc.get();
                if com == 0 {
                    uf.unite(x, y);
                } else {
                    let ans = if uf.find(x) == uf.find(y) { 1 } else { 0 };
                    sc.write(format!("{}\n", ans));
                }
            }
        });
    }
}

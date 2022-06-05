struct mod_combination {
    fac: Vec<usize>,
    inv: Vec<usize>,
    finv: Vec<usize>,
    md: usize,
}

impl mod_combination {
    fn new(N: usize, md: usize) -> mod_combination {
        let mut fac = vec![0; N + 1];
        let mut inv = vec![0; N + 1];
        let mut finv = vec![0; N + 1];
        fac[0] = 1;
        fac[1] = 1;
        inv[0] = 1;
        inv[1] = 1;
        finv[0] = 1;
        finv[1] = 1;
        for i in 2..N + 1 {
            fac[i] = i * fac[i - 1] % md;
            inv[i] = md - inv[md % i] * (md / i) % md;
            finv[i] = finv[i - 1] * inv[i] % md;
        }
        mod_combination {
            fac: fac,
            inv: inv,
            finv: finv,
            md: md,
        }
    }
    fn com(&self, n: usize, k: usize) -> usize {
        if n < k {
            0
        } else {
            self.fac[n] * (self.finv[k] * self.finv[n - k] % self.md) % self.md
        }
    }
    fn multi_choose(&self, n: usize, k: usize) -> usize {
        self.com(n+k-1 ,k)
    }
}

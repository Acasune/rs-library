struct RangeAddQueryTree {
    n: usize,
    bits: Vec<Vec<i64>>,
}
// [l,r)
impl RangeAddQueryTree {
    fn new(n: usize) -> Self {
        let size = n;
        RangeAddQueryTree {
            n: size,
            bits: vec![vec![0; size]; 2],
        }
    }

    fn add_sub(&mut self, bit: usize, mut pos: usize, x: i64) {
        let mut cnt = 0;
        pos += 1;
        while pos <= self.n {
            self.bits[bit][pos - 1] += x;
            pos += pos & pos.wrapping_neg();
            cnt += 1;
        }
    }

    fn add(&mut self, l: usize, r: usize, x: i64) {
        self.add_sub(0, l, -x * (l as i64));
        self.add_sub(0, r, x * (r as i64));
        self.add_sub(1, l, x);
        self.add_sub(1, r, -x);
    }

    fn sum_sub(&self, bit: usize, mut pos: usize) -> i64 {
        let bits = &self.bits;
        let mut s = 0;
        while pos > 0 {
            s += bits[bit][pos - 1];
            pos -= pos & pos.wrapping_neg();
        }
        s
    }

    fn sum(&self, mut pos: usize) -> i64 {
        self.sum_sub(0, pos) + (self.sum_sub(1, pos) * pos as i64)
    }

    fn query(&self, l: usize, r: usize) -> i64 {
        self.sum(r - 1) - self.sum(l - 1)
    }
}

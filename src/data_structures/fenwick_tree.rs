struct FenwickTree<T> {
    n: usize,
    data: Vec<T>,
    e: T,
}

impl<T> FenwickTree<T>
where
    T: Clone,
    T: std::ops::AddAssign<T>,
{
    fn new(n: usize, e: T) -> Self {
        let size = n.next_power_of_two();
        FenwickTree {
            n: size,
            data: vec![e.clone(); size],
            e: e,
        }
    }
    fn add(&mut self, mut pos: usize, x: T) {
        pos += 1;
        while pos <= self.n {
            self.data[pos - 1] += x.clone();
            pos += pos & pos.wrapping_neg();
        }
    }
    fn sum(&self, mut pos: usize) -> T
    where
        T: std::ops::Add<Output = T>,
    {
        let data = &self.data;
        let mut s = self.e.clone();
        while pos > 0 {
            s += data[pos - 1].clone();
            pos -= pos & pos.wrapping_neg();
        }
        s += data[pos].clone();
        s
    }

    fn query(&self, l: usize, r: usize) -> T
    where
        T: std::ops::Sub<Output = T>,
        T: std::ops::Add<Output = T>,
    {
        self.sum(r) - self.sum(l)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_raqt_max() {
        let n: usize = 8;
        let mut tree = FenwickTree::new(n, 0);

        for i in 0..n {
            tree.add(i, i as i64 + 1);
        }
        // tree: [1, 2, 3, 4, 5, 6, 7, 8]

        // test 1
        let expected = 3;
        let actual = tree.query(0, 2);
        assert_eq!(expected, actual);

        // test 2
        let expected = 12;
        let actual = tree.query(2, 5);
        assert_eq!(expected, actual);

        // test 3
        let expected = 36;
        let actual = tree.query(0, n);
        assert_eq!(expected, actual);
    }
}

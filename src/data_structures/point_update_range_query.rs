// Referring to https://atcoder.jp/contests/abc223/submissions/26655204
//PURQ stands for Point Update Range Query
struct PURQ<T, F> {
    n: usize,
    data: Vec<T>,
    e: T,
    op: F,
}

impl<T, F> PURQ<T, F>
where
    T: Clone,
    F: Fn(&T, &T) -> T,
{
    fn new(n: usize, e: T, op: F) -> Self {
        let size = n.next_power_of_two();
        PURQ {
            n: size,
            data: vec![e.clone(); 2 * size],
            e: e,
            op: op,
        }
    }
    fn update(&mut self, pos: usize, v: T) {
        assert!(pos < self.n);
        let mut pos = pos + self.n;
        let data = &mut self.data;
        data[pos] = v;
        pos >>= 1;
        while pos > 0 {
            data[pos] = (self.op)(&data[2 * pos], &data[2 * pos + 1]);
            pos >>= 1;
        }
    }
    fn update_tmp(&mut self, pos: usize, v: T) {
        assert!(pos < self.n);
        self.data[pos + self.n] = v;
    }
    fn update_all(&mut self) {
        let data = &mut self.data;
        for k in (1..self.n).rev() {
            data[k] = (self.op)(&data[2 * k], &data[2 * k + 1]);
        }
    }
    fn find(&self, l: usize, r: usize) -> T {
        assert!(l <= r && r <= self.n);
        if l == r {
            return self.e.clone();
        }
        let mut p = self.e.clone();
        let mut q = self.e.clone();
        let mut l = l + self.n;
        let mut r = r + self.n;
        let data = &self.data;
        while l < r {
            if l & 1 == 1 {
                p = (self.op)(&p, &data[l]);
                l += 1;
            }
            if r & 1 == 1 {
                r -= 1;
                q = (self.op)(&data[r], &q)
            }
            l >>= 1;
            r >>= 1;
        }
        (self.op)(&p, &q)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_purq_max() {
        let arr = vec![1, 3, 4, 7, 9];
        let n = arr.len();
        let mut tree = PURQ::new(n, 0, |&a, &b| a.max(b));
        for i in 0..n {
            tree.update_tmp(i, arr[i]);
        }
        tree.update_all();

        // test 1
        let expected = 7;
        let actual = tree.find(1, 4); // [1, [3, 4, 7], 9];
        assert_eq!(expected, actual);

        //test 2
        let expected = 4;
        let actual = tree.find(1, 3); // [1, [3, 4], 7, 9];
        assert_eq!(expected, actual);

        //test 3
        let expected = 9;
        let actual = tree.find(0, n + 1); // [[1, 3, 4, 7, 9]];
        assert_eq!(expected, actual);

        //test 4
        let expected = 9;
        let actual = tree.find(0, n); // [[1, 3, 4, 7, 9]];
        assert_eq!(expected, actual);

        //test 5
        let expected = 7;
        let actual = tree.find(1, n - 1); // [1, [3, 4, 7], 9];
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_purq_min() {
        let arr = vec![9, 1, 4, 5, 3];
        let n = arr.len();
        let mut tree = PURQ::new(n, i32::MAX / 10, |&a, &b| a.min(b));
        for i in 0..n {
            tree.update_tmp(i, arr[i]);
        }
        // test 1
        tree.update_all();
        let expected = 1;
        let actual = tree.find(1, n); // [9, [1, 4, 5, 3]]
        assert_eq!(expected, actual);

        // test 2
        tree.update_all();
        let expected = 4;
        let actual = tree.find(2, n - 1); // [9, 1, [4, 5], 3]
        assert_eq!(expected, actual);
    }
}

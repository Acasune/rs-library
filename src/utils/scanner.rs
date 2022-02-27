pub struct Scanner<R, W: std::io::Write> {
    reader: R,
    writer: std::io::BufWriter<W>,
}

impl<R: std::io::Read, W: std::io::Write> Scanner<R, W> {
    pub fn new(reader: R, writer: W) -> Scanner<R, W> {
        Scanner {
            reader: reader,
            writer: std::io::BufWriter::new(writer),
        }
    }
    pub fn get<T: std::str::FromStr>(&mut self) -> T {
        use std::io::Read;
        let buf = self
            .reader
            .by_ref()
            .bytes()
            .map(|b| b.unwrap())
            .skip_while(|&b| b == b' ' || b == b'\n' || b == b'\r' || b == b'\t')
            .take_while(|&b| b != b' ' && b != b'\n' && b != b'\r' && b != b'\t')
            .collect::<Vec<_>>();
        unsafe { std::str::from_utf8_unchecked(&buf) }
            .parse()
            .ok()
            .expect("Parse error.")
    }

    pub fn get_vec<T: std::str::FromStr>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.get()).collect()
    }
    pub fn write<S: ToString>(&mut self, s: S) {
        use std::io::Write;
        self.writer.write_all(s.to_string().as_bytes()).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_input() {
        let input = br"10
    0 1 2 3 4 5 6 7 8 9
    abcde fghij
    1.732 -1000
    
    end_of_file";
        let mut sc = Scanner::new(&input[..], Vec::new());

        // test 1
        let n = sc.get::<usize>();
        assert_eq!(10, n);

        // test 2
        let a = sc.get_vec::<i64>(n);
        assert_eq!(&vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9], &a);

        // test 3
        let b = sc.get::<String>();
        let c = sc.get::<String>();
        assert_eq!("abcde", b);
        assert_eq!("fghij", c);

        // test 4
        let f = sc.get::<f64>();
        let ne = sc.get::<i64>();
        assert_eq!(1.732, f);
        assert_eq!(-1000, ne);

        // test 5
        let s = sc.get::<String>();
        assert_eq!("end_of_file", s);
    }

    #[test]
    fn write_test() {
        let mut output = Vec::new();
        {
            let mut sc = Scanner::new(&b""[..], &mut output);
            sc.write(format!("{}\n", 1));
            sc.write(format!("{}\n", 2));
            sc.write(format!("{}\n", 3));
        }

        let output = String::from_utf8(output).expect("Not UTF-8");
        assert_eq!(&output, "1\n2\n3\n");
    }
}

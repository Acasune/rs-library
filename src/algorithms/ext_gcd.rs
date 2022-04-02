fn ext_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (0, 1, b)
    } else {
        let (x, y, g) = ext_gcd(b % a, a);
        (y - b / a * x, x, g)
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::tester::Tester;

    use super::*;

    #[test]
    fn test_ext_gcd() {
        // https://onlinejudge.u-aizu.ac.jp/problems/NTL_1_E

        let tester = Tester::new("./assets/NTL_1_E/in", "./assets/NTL_1_E/out");
        tester.solve_by_algorithm(|sc| {
            let a: i64 = sc.get();
            let b: i64 = sc.get();
            let (mut x, mut y, g) = ext_gcd(a, b);
            if a == b && x > y {
                let tmp = x;
                x = y;
                y = tmp;
            }
            sc.write(format!("{} {}\n", x, y));
        });
    }
}

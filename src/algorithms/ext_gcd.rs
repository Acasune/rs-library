fn ext_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (0, 1, b)
    } else {
        let (x, y, g) = ext_gcd(b % a, a);
        (y - b / a * x, x, g)
    }
}

fn gcd(a: i64, b: i64) -> i64 {
    ext_gcd(a, b).2
}

fn lcm(a: i64, b: i64) -> i64 {
    let g = gcd(a, b);
    (a / g) * b
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

    #[test]
    fn test_find_gcd() {
        //test 1
        let expected = 2;
        let actual = gcd(2, 4);
        assert_eq!(expected, actual);

        // test 2
        let expected = 7;
        let actual = gcd(14, 49);
        assert_eq!(expected, actual);

        // test 3
        let expected = 7;
        let actual = gcd(49, 14);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_find_lcm() {
        //test 1
        let expected = 2 * 2;
        let actual = lcm(2, (2 * 2));
        assert_eq!(expected, actual);

        // test 2
        let expected = 2 * 3;
        let actual = lcm(2, 3);
        assert_eq!(expected, actual);

        // test 3
        let expected = 7 * 2 * 3;
        let actual = lcm(7 * 2, 7 * 3);
        assert_eq!(expected, actual);
    }
}

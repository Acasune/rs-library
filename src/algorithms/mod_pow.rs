fn mod_pow(base: i64, power: i64, md: i64) -> i64 {
    // base^power % md
    let mut ret = 1;
    let mut base = base;
    let mut power = power;
    while power > 0 {
        if power % 2 == 1 {
            ret = (ret % md) * (base % md);
            ret %= md;
        }
        base = (base % md) * (base % md);
        base %= md;
        power >>= 1;
    }
    return ret;
}

#[cfg(test)]
mod tests {
    use crate::utils::tester::Tester;

    use super::*;

    #[test]
    fn test_mod_pow() {
        // https://onlinejudge.u-aizu.ac.jp/problems/NTL_1_B

        let tester = Tester::new("./assets/NTL_1_B/in", "./assets/NTL_1_B/out");
        tester.solve_by_algorithm(|sc| {
            let base: i64 = sc.get();
            let power: i64 = sc.get();
            let md = 1_000_000_007;
            sc.write(format!("{}\n", mod_pow(base, power, md)));
        });
    }
}

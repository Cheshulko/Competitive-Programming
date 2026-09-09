// https://leetcode.com/problems/count-commas-in-range-ii

struct Solution;

impl Solution {
    pub fn count_commas(n: i64) -> i64 {
        let mut ans = 0;
        let mut x = 1000;
        let mut c = 0;
        for cnt in 0.. {
            if x * 10 > n {
                break;
            }

            if cnt % 3 == 0 {
                c += 1;
            }
            let c = c * 9 * x;
            ans += c;
            x *= 10;
        }

        let mut cg = 0;
        let mut nn = n;
        while nn / 1000 > 0 {
            cg += 1;
            nn /= 1000;
        }

        ans + cg * (n - x + 1)
    }
}

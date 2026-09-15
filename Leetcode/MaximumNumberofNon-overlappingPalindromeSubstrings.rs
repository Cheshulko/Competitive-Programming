// https://leetcode.com/problems/maximum-number-of-non-overlapping-palindrome-substrings

struct Solution;

impl Solution {
    pub fn max_palindromes(s: String, k: i32) -> i32 {
        fn z_fn(s: &[u8]) -> Vec<usize> {
            let n = s.len();
            let mut z = vec![0; n];
            let mut l = 0;
            let mut r = 0;

            for i in 1..n {
                if r >= i {
                    z[i] = z[i - l].min(r - i + 1);
                }
                while z[i] + i < n && s[z[i]] == s[z[i] + i] {
                    z[i] += 1;
                }
                if r < i + z[i] - 1 {
                    l = i;
                    r = i + z[i] - 1;
                }
            }

            z
        }

        let k = k as usize;
        let n = s.len();
        let s = s.into_bytes().into_iter().collect::<Vec<_>>();
        let mut s_rev = s.clone();
        s_rev.reverse();

        let mut pal = vec![vec![]; n];
        for i in 0..n {
            let mut s2 = vec![];
            s2.extend_from_slice(&s[i..]);
            s2.push(b'$');
            s2.extend_from_slice(&s_rev[..n - i]);

            let z = z_fn(&s2);
            for j in (n - i)..z.len() {
                if z.len() - j == z[j] {
                    pal[i].push(z[j]);
                }
            }
        }

        let mut dp = vec![0; n + 1];
        for i in 0..n {
            for &p in pal[i].iter() {
                if p >= k && i + p <= n {
                    dp[i + p] = dp[i + p].max(dp[i] + 1);
                }
            }
            dp[i + 1] = dp[i + 1].max(dp[i]);
        }

        dp.iter().max().copied().unwrap()
    }
}

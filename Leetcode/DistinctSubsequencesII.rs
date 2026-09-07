// https://leetcode.com/problems/distinct-subsequences-ii

struct Solution;

impl Solution {
    pub fn distinct_subseq_ii(s: String) -> i32 {
        const M: usize = 1_000_000_000 + 7;

        let s = s.into_bytes().into_iter().collect::<Vec<_>>();
        let n = s.len();

        let mut dp = vec![0; n + 1];
        let mut ndp = vec![0; n + 1];
        dp[0] = 1;

        let mut ans = 0;
        for _ in 1..=n {
            let mut seen = [0; 26];
            ndp.fill(0);

            let mut cnt = 0;
            for i in 1..=n {
                let c = (s[i - 1] - b'a') as usize;

                cnt = (cnt + dp[i - 1]) % M;
                ndp[i] = (M + ndp[i] + cnt - seen[c]) % M;
                seen[c] = (seen[c] + ndp[i]) % M;
                ans = (ans + ndp[i]) % M;
            }

            std::mem::swap(&mut dp, &mut ndp);
        }

        ans as i32
    }
}

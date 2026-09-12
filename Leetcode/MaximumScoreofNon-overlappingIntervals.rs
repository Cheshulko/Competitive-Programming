// https://leetcode.com/problems/maximum-score-of-non-overlapping-intervals

struct Solution;

impl Solution {
    pub fn maximum_weight(intervals: Vec<Vec<i32>>) -> Vec<i32> {
        let compressed = {
            use std::collections::{BTreeSet, HashMap};
            let mut hs = BTreeSet::new();
            let mut mp = HashMap::new();

            for v in intervals.iter() {
                hs.insert(v[0]);
                hs.insert(v[0] - 1);
                hs.insert(v[0] + 1);
                hs.insert(v[1]);
                hs.insert(v[1] - 1);
                hs.insert(v[1] + 1);
            }

            for x in hs.into_iter() {
                if !mp.contains_key(&x) {
                    let len = mp.len() + 1;
                    mp.insert(x, len);
                }
            }

            mp
        };

        let mut intervals = intervals
            .into_iter()
            .enumerate()
            .map(|(ind, v)| {
                (
                    *compressed.get(&v[1]).unwrap(), // end
                    *compressed.get(&v[0]).unwrap(), // start
                    v[2] as i64,
                    ind,
                )
            })
            .collect::<Vec<_>>();

        assert!(intervals.iter().all(|v| v.0 > 0 && v.1 > 0));

        intervals.sort_unstable();

        let l = compressed.len() + 1;

        let mut dp: Vec<[(i64, Vec<usize>); 5]> =
            vec![std::array::from_fn(|_| (-1, vec![])); l + 1];
        dp[0][0] = (0, vec![]);

        let mut best = -1;
        let mut best_v = vec![];

        let mut i = 0;
        for p in 1..=l {
            while i < intervals.len() && intervals[i].0 == p {
                if intervals[i].2 > dp[p][1].0 {
                    dp[p][1].0 = intervals[i].2;
                    dp[p][1].1 = vec![intervals[i].3];
                } else if intervals[i].2 == dp[p][1].0 && vec![intervals[i].3] < dp[p][1].1 {
                    dp[p][1].1 = vec![intervals[i].3];
                }

                let start_prev = intervals[i].1;

                for lp in 1..=4 {
                    if dp[start_prev - 1][lp - 1].0 == -1 {
                        continue;
                    }

                    if dp[p][lp].0 < dp[start_prev - 1][lp - 1].0 + intervals[i].2 {
                        dp[p][lp].0 = dp[start_prev - 1][lp - 1].0 + intervals[i].2;

                        let mut v = dp[start_prev - 1][lp - 1].1.clone();
                        v.push(intervals[i].3);
                        v.sort_unstable();

                        dp[p][lp].1 = v;
                    } else if dp[p][lp].0 == dp[start_prev - 1][lp - 1].0 + intervals[i].2 {
                        let mut v = dp[start_prev - 1][lp - 1].1.clone();
                        v.push(intervals[i].3);
                        v.sort_unstable();

                        if v < dp[p][lp].1 {
                            dp[p][lp].1 = v;
                        }
                    }
                }

                i += 1;
            }

            for l in 1..=4 {
                if dp[p - 1][l].0 > dp[p][l].0 {
                    dp[p][l].0 = dp[p - 1][l].0;
                    dp[p][l].1 = dp[p - 1][l].1.clone();
                } else if dp[p - 1][l].0 == dp[p][l].0 && dp[p - 1][l].1 < dp[p][l].1 {
                    dp[p][l].1 = dp[p - 1][l].1.clone();
                }
            }

            for l in 1..=4 {
                if dp[p][l].0 > best {
                    best = dp[p][l].0;
                    best_v = dp[p][l].1.clone();
                } else if dp[p][l].0 == best && dp[p][l].1 < best_v {
                    best_v = dp[p][l].1.clone();
                }
            }
        }

        best_v.into_iter().map(|i| i as i32).collect()
    }
}

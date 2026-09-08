// https://leetcode.com/problems/count-commas-in-range

struct Solution;

impl Solution {
    pub fn count_commas(n: i32) -> i32 {
        (1000..=n).map(|x| (x as f32).log10() as i32 / 3).sum()
    }
}

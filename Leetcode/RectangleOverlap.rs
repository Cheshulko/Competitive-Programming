// https://leetcode.com/problems/rectangle-overlap

struct Solution;

impl Solution {
    pub fn is_rectangle_overlap(rec1: Vec<i32>, rec2: Vec<i32>) -> bool {
        let x1 = rec1[0] >= rec2[2] || rec1[2] <= rec2[0];
        let x2 = rec1[1] >= rec2[3] || rec1[3] <= rec2[1];

        !(x1 || x2)
    }
}

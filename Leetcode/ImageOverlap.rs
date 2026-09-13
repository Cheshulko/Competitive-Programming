// https://leetcode.com/problems/image-overlap

struct Solution;

impl Solution {
    pub fn largest_overlap(img1: Vec<Vec<i32>>, img2: Vec<Vec<i32>>) -> i32 {
        fn count(
            img1: &Vec<Vec<i32>>,
            img2: &Vec<Vec<i32>>,
            i0: usize,
            j0: usize,
            i1: usize,
            j1: usize,
        ) -> i32 {
            let n = img1.len();

            let mut cnt = 0;
            for ii in 0..n {
                if ii + i0 >= n || ii + i1 >= n {
                    break;
                }
                for jj in 0..n {
                    if jj + j0 >= n || jj + j1 >= n {
                        break;
                    }
                    cnt += (img1[ii + i0][jj + j0] + img2[ii + i1][jj + j1]) / 2;
                }
            }

            cnt
        }

        let n = img1.len();

        let mut ans = 0;
        for i0 in 0..n {
            for j0 in 0..n {
                for i1 in 0..n {
                    for j1 in 0..n {
                        ans = ans.max(count(&img1, &img2, i0, j0, i1, j1));
                    }
                }
            }
        }

        ans
    }
}

use std::cmp::min;

pub struct Solution;

impl Solution {

    // 出现峰值-有升，有降
    pub fn find_peaks(mountain: Vec<i32>) -> Vec<i32> {
        let n = mountain.len();

        let mut peaks = Vec::new();

        if n < 3 {
            return peaks;
        }

        for i in 1..n - 1 {
            if mountain[i] > mountain[i - 1] && mountain[i] > mountain[i + 1] {
                peaks.push(i as i32);
            }
        }

        peaks
    }
}

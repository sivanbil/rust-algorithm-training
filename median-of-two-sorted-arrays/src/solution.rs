pub struct Solution;
impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        // 首先需要排序，其次才能根据位置进行算中位数
        let total_nums = &nums1.len() + &nums2.len();
        // 中间位置为奇数
        let mut is_one = true;
        if total_nums % 2 == 0 {
            // =偶数
            is_one = false;
        }
        let mut merged = Vec::with_capacity(nums1.len() + nums2.len());
        let mut i = 0;
        let mut j = 0;

        while i < nums1.len() && j < nums2.len() {
            if nums1[i] < nums2[j] {
                merged.push(nums1[i]);
                i += 1;
            } else {
                merged.push(nums2[j]);
                j += 1;
            }
        }

        merged.extend_from_slice(&nums1[i..]);
        merged.extend_from_slice(&nums2[j..]);

        if total_nums > 0 {
            if is_one {
                return merged[total_nums /2 ] as f64/ 1.0;
            }

            return (merged[total_nums/2] + merged[total_nums/2 -1]) as f64/2.0;
        }
        0.0

    }
}
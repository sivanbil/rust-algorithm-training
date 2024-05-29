pub struct Solution;

impl Solution {
    // Kadane's Algorithm 的核心思想
    // 初始化：
    //
    // max_current 初始化为数组的第一个元素。这表示当前以第一个元素结尾的最大子数组的和。
    // max_global 也初始化为数组的第一个元素。这表示当前找到的最大子数组的和。
    // 遍历数组：
    //
    // 从第二个元素开始，遍历整个数组。
    // 对于每一个元素，计算它是否应该被包含到当前的子数组中（即，当前子数组和继续累加这个元素是否会更大），还是从这个元素重新开始一个新的子数组（即，这个元素本身比当前子数组和加上这个元素更大）。
    // 更新 max_current 为这两者中的最大值：max(num, max_current + num)。
    // 然后，将 max_current 与 max_global 比较，更新 max_global 为两者中的较大者。
    // 最终结果：
    //
    // 遍历结束后，max_global 中存储的就是数组中具有最大和的连续子数组的和。
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        // max_current和max_global先设定为数组的第一个元素
        let mut max_current = nums[0];
        let mut max_global = nums[0];

        // 遍历数组，从第二个元素开始
        for &num in nums.iter().skip(1) {
            // 当前元素与max_current加上当前元素之间的最大值则为当前最大值
            max_current = i32::max(num, max_current + num);

            // 更新max_global为max_global和max_current之间的最大值
            max_global = i32::max(max_global, max_current);
        }

        max_global
    }
}
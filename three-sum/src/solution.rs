pub struct Solution;

impl Solution {
    // 给定一个包含 n 个整数的数组 nums，判断 nums 中是否存在三个元素 a ，b ，c ，使得 a + b + c = 0 ？请找出所有和为 0 且 不重复 的三元组。
    //
    //
    //
    // 示例 1：
    //
    // 输入：nums = [-1,0,1,2,-1,-4]
    // 输出：[[-1,-1,2],[-1,0,1]]
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        if nums.len() < 3 {
            return vec![];
        }
        let mut result = Vec::new();
        let mut nums = nums;
        nums.sort(); // 先对数组进行排序

        for i in 0..nums.len() {
            if i > 0 && nums[i] == nums[i - 1] {
                continue; // 跳过重复的元素
            }

            let mut left = i + 1;
            let mut right = nums.len() - 1;

            while left < right {
                let sum = nums[i] + nums[left] + nums[right];

                if sum == 0 {
                    result.push(vec![nums[i], nums[left], nums[right]]);

                    while left < right && nums[left] == nums[left + 1] {
                        left += 1; // 跳过重复的元素
                    }

                    while left < right && nums[right] == nums[right - 1] {
                        right -= 1; // 跳过重复的元素
                    }

                    left += 1;
                    right -= 1;
                } else if sum < 0 {
                    left += 1;
                } else {
                    right -= 1;
                }
            }
        }

        result
    }
}

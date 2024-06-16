pub struct Solution;

impl Solution {
    //给定一个包含 n + 1 个整数的数组 nums ，其数字都在 [1, n] 范围内（包括 1 和 n），可知至少存在一个重复的整数。
    //
    // 假设 nums 只有 一个重复的整数 ，返回 这个重复的数 。
    //
    // 你设计的解决方案必须 不修改 数组 nums 且只用常量级 O(1) 的额外空间。
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        let mut slow = 0;
        let mut fast = 0;

        // 找到相遇点 确定是否存在环
        loop {
            slow = nums[slow as usize] as usize;
            fast = nums[nums[fast as usize] as usize] as usize;
            if slow == fast {
                break;
            }
        }

        // 从头开始重新遍历找到环的入口点
        // 一旦确定存在环,我们就可以利用环的一个性质:从环外任意一点出发,与从环的入口点出发,最终会相遇于环的入口点。
        let mut ptr1 = 0;
        let mut ptr2 = slow;
        while ptr1 != ptr2 {
            ptr1 = nums[ptr1 as usize] as usize;
            ptr2 = nums[ptr2 as usize] as usize;
        }

        ptr1 as i32
    }
}

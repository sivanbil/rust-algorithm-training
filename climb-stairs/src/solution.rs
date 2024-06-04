pub struct Solution;
impl Solution {
    // 假设你正在爬楼梯。需要 n 阶你才能到达楼顶。
    //
    // 每次你可以爬 1 或 2 个台阶。你有多少种不同的方法可以爬到楼顶呢？
    pub fn climb_stairs(n: i32) -> i32 {
        if n <= 2 {
            return n;
        }

        // 初始化前两个值
        let mut dp = vec![0; (n + 1) as usize];
        dp[1] = 1;
        dp[2] = 2;

        // 动态规划求解
        for i in 3..=n as usize {
            dp[i] = dp[i - 1] + dp[i - 2];
        }

        dp[n as usize]
    }
}
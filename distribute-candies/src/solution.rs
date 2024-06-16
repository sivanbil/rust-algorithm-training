use std::collections::HashSet;

pub struct Solution;

impl Solution {
    // Alice 有 n 枚糖，其中第 i 枚糖的类型为 candyType[i] 。Alice 注意到她的体重正在增长，所以前去拜访了一位医生。
    //
    // 医生建议 Alice 要少摄入糖分，只吃掉她所有糖的 n / 2 即可（n 是一个偶数）。Alice 非常喜欢这些糖，她想要在遵循医生建议的情况下，尽可能吃到最多不同种类的糖。
    //
    // 给你一个长度为 n 的整数数组 candyType ，返回： Alice 在仅吃掉 n / 2 枚糖的情况下，可以吃到糖的 最多 种类数

    pub fn distribute_candies(candy_type: Vec<i32>) -> i32 {
        // 一个是糖的种类，有几种
        //  一半数量有多少
        let mut candy_type_hash = HashSet::new();

        for i in 0..candy_type.len() {
            candy_type_hash.insert(candy_type[i]);
        }

        let mut candy_target_nums = candy_type.len()/2;
        // 如果还剩余的数量大于种类
        if candy_target_nums > candy_type_hash.len() {
            return candy_type_hash.len() as  i32
        }
        candy_target_nums as i32

    }
}
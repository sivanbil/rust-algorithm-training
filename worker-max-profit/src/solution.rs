use std::cmp::max;

pub struct Solution;

impl Solution {
    // 你有 n 个工作和 m 个工人。给定三个数组： difficulty, profit 和 worker ，其中:
    //
    // difficulty[i] 表示第 i 个工作的难度，profit[i] 表示第 i 个工作的收益。
    // worker[i] 是第 i 个工人的能力，即该工人只能完成难度小于等于 worker[i] 的工作。
    // 每个工人 最多 只能安排 一个 工作，但是一个工作可以 完成多次 。
    //
    // 举个例子，如果 3 个工人都尝试完成一份报酬为 $1 的同样工作，那么总收益为 $3 。如果一个工人不能完成任何工作，他的收益为 $0 。
    // 返回 在把工人分配到工作岗位后，我们所能获得的最大利润 。
    //
    //
    //
    // 示例 1：
    //
    // 输入: difficulty = [2,4,6,8,10], profit = [10,20,30,40,50], worker = [4,5,6,7]
    // 输出: 100
    // 解释: 工人被分配的工作难度是 [4,4,6,6] ，分别获得 [20,20,30,30] 的收益。
    pub fn max_profit_assignment(difficulty: Vec<i32>, profit: Vec<i32>, worker: Vec<i32>) -> i32 {
        // 工人数量-worker
        // 工作数量-difficulty
        // 能完成的工作所对应的单次收入-profit
        //  是不是只要计算单个人的收益，然后累加即可
        let mut result = 0;
        for i in 0..worker.len() {
            let mut worker_max_profit = 0;
            for j in 0..difficulty.len() {
                if worker[i] >= difficulty[j] {
                    worker_max_profit = max(worker_max_profit, profit[j]);
                }
            }
            result += worker_max_profit;

        }

        result
    }

    pub fn max_profit_assignment_extend(mut difficulty: Vec<i32>, mut profit: Vec<i32>, mut worker: Vec<i32>) -> i32 {
        let n = difficulty.len();
        let m = worker.len();

        // 按难度对任务进行排序
        let mut assignments = Vec::with_capacity(n);
        for i in 0..n {
            assignments.push((difficulty[i], profit[i]));
        }
        assignments.sort_by_key(|&(d, _)| d);

        // 按能力对工人进行排序
        worker.sort();

        let mut result = 0;
        let mut best_profit = vec![0; m];

        for i in 0..n {
            let (d, p) = assignments[i];
            let j = worker.partition_point(|&w| w < d);

            for k in j..m {
                best_profit[k] = max(best_profit[k], p);
            }
        }

        result = best_profit.iter().sum();
        result
    }
}

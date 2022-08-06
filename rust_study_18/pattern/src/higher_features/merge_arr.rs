/*
    给你两个按 非递减顺序 排列的整数数组nums1 和 nums2，另有两个整数 m 和 n ，分别表示 nums1 和 nums2 中的元素数目。
    请你 合并 nums2 到 nums1 中，使合并后的数组同样按 非递减顺序 排列。
    注意：最终，合并后数组不应由函数返回，而是存储在数组 nums1 中。为了应对这种情况，nums1 的初始长度为 m + n，其中前 m 个元素表示应合并的元素，后 n 个元素为 0 ，应忽略。nums2 的长度为 n 。
    进阶：你可以设计实现一个时间复杂度为 O(m + n) 的算法解决此问题吗？
*/
use std::cmp::{max, min};
use std::collections::HashMap;
pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    let mut temps: Vec<i32> = Vec::new();
    let mut index: usize = 0;
    let mut i: usize = 0;
    let mut j: usize = 0;
    while ((i as i32) < m && (j as i32) < n) {
        if nums1[i] < nums2[j] {
            temps.push(nums1[i]);
            i += 1;
            index += 1;
        } else {
            temps.push(nums2[j]);
            j += 1;
            index += 1;
        }
    }

    for i2 in i..(m as usize) {
        temps.push(nums1[i2]);
        i += 1;
        index += 1;
    }

    for j2 in j..(n as usize) {
        temps.push(nums2[j2]);
        j += 1;
        index += 1;
    }
    for numIndex in 0..(m+n as i32) {
        nums1[numIndex as usize] = temps[numIndex as usize];
    }
}

/*
    给你 k 枚相同的鸡蛋，并可以使用一栋从第 1 层到第 n 层共有 n 层楼的建筑。

    已知存在楼层 f ，满足0 <= f <= n ，任何从 高于 f 的楼层落下的鸡蛋都会碎，从 f 楼层或比它低的楼层落下的鸡蛋都不会破。

    每次操作，你可以取一枚没有碎的鸡蛋并把它从任一楼层 x 扔下（满足1 <= x <= n）。如果鸡蛋碎了，你就不能再次使用它。如果某枚鸡蛋扔下后没有摔碎，则可以在之后的操作中 重复使用 这枚鸡蛋。

    请你计算并返回要确定 f 确切的值 的 最小操作次数 是多少？
*/

/*pub fn dp(k: i32, n: i32, memo: &mut HashMap<String, i32>) -> i32 {
    if k == 1 {
        // 剩一个鸡蛋的时候只能线性搜索
        return n;
    }
    if n == 0 {
        // 没有鸡蛋的时候无法操作
        return 0;
    }
    let key = k.to_string() + &",".to_string() + &n.to_string();

    if memo.contains_key(&key) {
        return *(memo.get(&key).unwrap());
    }

    let mut res = i32::MAX;

    for i in 1..n + 1 {
        // 在第i层扔鸡蛋
        res = min(res, max(dp(k - 1, i - 1, memo), dp(k, n - i, memo)) + 1);
    }
    memo.insert(key, res);
    return res
}

pub fn super_egg_drop(k: i32, n: i32) -> i32 {
    // * 状态描述为 (K, N), 则当从第 X 楼扔鸡蛋时候: 鸡蛋碎了: dp(K - 1, X - 1), 鸡蛋没碎: dp(K, N - X)
    // * 动态规划: dp(K, N) = min(max(dp(K - 1, X - 1), dp(K, N - X))) + 1; 其中 1 <= X <= N, X表示当前楼层
    let mut memo: HashMap<String, i32> = HashMap::new();
    return dp(k, n, &mut memo);
}*/

// 二分优化
pub fn dp(k: i32, n: i32, memo: &mut HashMap<String, i32>) -> i32 {
    if k == 1 {
        // 剩一个鸡蛋的时候只能线性搜索
        return n;
    }
    if n == 0 {
        // 没有鸡蛋的时候无法操作
        return 0;
    }
    let key = k.to_string() + &",".to_string() + &n.to_string();

    if memo.contains_key(&key) {
        return *(memo.get(&key).unwrap());
    }

    let mut res = i32::MAX;

    let mut low = 1;
    let mut height = n;
    while low <= height {
        let mut mid = low + (height - low) / 2;
        let mut broken = dp(k - 1, mid - 1, memo);
        let not_broken = dp(k, n - mid, memo);
        if broken > not_broken {
            height = mid - 1;
            res = min(res, broken + 1);
        } else {
            low = mid + 1;
            res = min(res, not_broken + 1);
        }
    }
    memo.insert(key, res);
    return res;
}

pub fn super_egg_drop(k: i32, n: i32) -> i32 {
    // * 状态描述为 (K, N), 则当从第 X 楼扔鸡蛋时候: 鸡蛋碎了: dp(K - 1, X - 1), 鸡蛋没碎: dp(K, N - X)
    // * 动态规划: dp(K, N) = min(max(dp(K - 1, X - 1), dp(K, N - X))) + 1; 其中 1 <= X <= N, X表示当前楼层
    let mut memo: HashMap<String, i32> = HashMap::new();
    return dp(k, n, &mut memo);
}
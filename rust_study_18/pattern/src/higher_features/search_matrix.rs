/* 
    编写一个高效的算法来搜索 m x n 矩阵 matrix 中的一个目标值 target 。该矩阵具有以下特性：
        - 每行的元素从左到右升序排列。
        - 每列的元素从上到下升序排列。

    输入：matrix = [[1,4,7,11,15],[2,5,8,12,19],[3,6,9,16,22],[10,13,14,17,24],[18,21,23,26,30]], target = 5
    输出：true

    输入：matrix = [[1,4,7,11,15],[2,5,8,12,19],[3,6,9,16,22],[10,13,14,17,24],[18,21,23,26,30]], target = 20
    输出：false
*/
pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    // * 暴力破解
    // for matrix_outer in matrix {
    //     for number in matrix_outer {
    //         if number == target {
    //             return true;
    //         }
    //     }
    // }
    // return false;

    // * 二叉树法
    // * 从矩阵左下角开始比较(该二维数组类似于一棵排序二叉树, 对于每个数来说, 上方的数都小于它, 右方的数都大于它, 所以把左下角作为根节点开始比较)
    let mut col = 0;
    // usize 不能是负数, 所以这里需要使用isize, 但是 isize无法作为 vec 的键, 因此需要断言(isize 可断言为 usize)
    let mut row: isize = (matrix.len() - 1) as isize;
    while row >= 0 && col < matrix[0].len() {
        if target > matrix[row as usize][col] {
            col += 1;
        } else if target < matrix[row as usize][col] {
            println!("col: {}, row: {}", col, row);
            row = (row - 1) as isize;
        } else {
            return true;
        }
    }
    return false;
}
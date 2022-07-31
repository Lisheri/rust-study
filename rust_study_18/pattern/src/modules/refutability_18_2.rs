/* 
    * Refutability (可反驳性): 模式是否会匹配失效

    模式有两种形式: 
        + refutable(可反驳的): 对某些可能的值进行匹配会失败的模式, 例: if let Some(x) = a_value 表达式中的 Some(x)；如果变量 a_value 中的值是 None 而不是 Some，那么 Some(x) 模式不能匹配。
        + irrefutable (不可反驳的): 能匹配任何传递的可能值的模式, 例: let x = 5; 语句中的 x, 因为 x 可以匹配任何值, 所以不可能会失败。
        
    
    + 函数参数、let语句、for循环只接受不可反驳的模式(就是不可失败的)
    + if let 和 while let 接受可反驳和不可反驳的模式(因为存在可能的失败)
*/

pub fn tst_refutability() {
    // * 不可反驳模式
    let a: Option<i32> = Some(5);
    // * 这里使用 let 跟一个 Some(x), 让 a 赋值给他, 但是Some(x)是一个可反驳的模式, 他是可以失败的, 所以这里就有冲突
    // * 报错提示: 在本地绑定中, 使用了可反驳模式, 没有考虑None
    // let Some(x) = a;
    // ? 修改为 if let即可
    if let Some(x) = a {
        // * 匹配后进入
        println!("Some(x) 满足 a");
    }

    // * 如果 if let 后面使用不可反驳的, 那么会有一个警告, 表示这个匹配总是成功, 如下
    if let x = 5 {
        println!("x 满足 5的匹配");
    }

    // * 对于match 表达式而言, 对于除最后一个分支之外的其他所有分支, 都是可反驳和不可反驳的, 而最后一个分支, 只能是不可反驳分支, 因为需要匹配所有剩余的情况
}
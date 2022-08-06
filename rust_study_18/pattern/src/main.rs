/* 
    18 模式与模式匹配

    模式是Rust中特殊的语法, 用来匹配类型中的结构, 无论类型是简单还是复杂。
    结合模式和 match 表达式以及其他结构可以提供更多对程序控制流的支配权。
    模式由如下要写内容组合而成:
        + 字面值
        + 解构的数组、枚举、结构体或元组
        + 变量
        + 通配符
        + 占位符
    这些部分描述了我们要处理的数据的形状, 接着可以用其匹配值来决定程序是否拥有正确的数据来运行特定部分的代码

    通过将一些值与模式相比较来使用。如果模式匹配这些值，我们对值部分进行相应处理。
*/

use pattern::modules::refutability_18_2::tst_refutability;
use pattern::modules::pattern_language::*;
use pattern::higher_features::unsafe_rust_19::*;
use pattern::higher_features::search_matrix::*;
use pattern::higher_features::merge_arr::*;
use pattern::higher_features::higher_trait::*;
use pattern::higher_features::higher_type::*;
use pattern::higher_features::higher_fn::*;
fn main() {
    // tst_refutability();
    // unsafe_rust_origin_pointer();
    // unsafe_rust_use_unsafe_fn();
    // unsafe_rust_safe_abstract();
    // use_extern();
    // use_static_fn();
    // let mut nums1 = vec![1,2,3,0,0,0];
    // let mut nums2 = vec![2, 5, 6];
    // merge(&mut nums1, 3, &mut nums2, 3);
    // println!("次数: {}", super_egg_drop(4, 5000));
    // use_human();
    // use_out_line_print();
    // use_wrapper();
    // use_kilometers();
    use_hof();
    // println!("nums1: {:?}", nums1);
    // let result = search_matrix(vec![vec![1,4,7,11,15], vec![2,5,8,12,19], vec![3,6,9,16,22], vec![10,13,14,17,24], vec![18,21,23,26,30]], 20);
    // let result = search_matrix(vec![vec![-5]], -10);
    // println!("{}", result);
    // match_literal_value();
    // match_name_variable();
    // multiple_mode();
    // ignore_value();
    // match_guard();
    // bind();
    // * 所有可能会用到模式的位置
    // ? match 分支
    /* 
        一个模式常用的位置是 match 表达式的分支。
        match VALUE {
            PATTERN => EXPRESSION,
            PATTERN => EXPRESSION,
            PATTERN => EXPRESSION,
            PATTERN => EXPRESSION,
        }
        
        match 表达式必须是 穷尽的, 意为 match 表达式所有可能的值都必须被考虑到。
        一个确保覆盖每个可能值的方法是在最后一个分支使用捕获所有的模式：比如，一个匹配任何值的名称永远也不会失败，因此可以覆盖所有匹配剩下的情况。

        有一个特定的模式 _ 可以匹配所有情况，不过它从不绑定任何变量。
    */

    // ?  if let 条件表达式
    /* 
        if let 可以对应一个可选的带有代码的 else 在 if let 中的模式不匹配时运行
        
        if let 和可以 else if、 else 混合使用

        * if let 表达式的缺点在于其穷尽性没有为编译器所检查，而 match 表达式则检查了。如果去掉最后的 else 块而遗漏处理一些情况，编译器也不会警告这类可能的逻辑错误。
    */

    // ? while let 条件循环
    /* 
        一个与 if let 结构类似的是 while let 条件循环，它允许只要模式匹配就一直进行 while 循环
    */

    // ? for循环
    /* 
        在 for 循环中，模式是 for 关键字直接跟随的值，正如 for x in y 中的 x。
    */
    
    // ? let 语句
    /* 
        像 let x = 5; 这样的语句中变量名位于 PATTERN 位置，变量名不过是形式特别朴素的模式。
        我们将表达式与模式比较，并为任何找到的名称赋值。
        所以例如 let x = 5; 的情况，x 是一个代表 “将匹配到的值绑定到变量 x” 的模式。
        同时因为名称 x 是整个模式，这个模式实际上等于 “将任何值绑定到变量 x，不管值是什么”。
    */

    // ? 函数参数
    /* 
        函数参数也可以是模式
    */

    /* 
        现在我们见过了很多使用模式的方式了，不过模式在每个使用它的地方并不以相同的方式工作；
        在一些地方，模式必须是 irrefutable 的，意味着他们必须匹配所提供的任何值。
        在另一些情况，他们则可以是 refutable 的。接下来让我们讨论这两个概念。
    */
}

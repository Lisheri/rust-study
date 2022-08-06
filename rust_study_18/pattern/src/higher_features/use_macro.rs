// 宏 macro

/* 
    1. 宏在Rust里指的是一组相关特性的集合称谓:
        使用 macro_rules! 构建的声明宏(declarative macro)
        3种过程宏:
            - 自定义 #[derive]宏, 用于 struct 或 enum, 可以为其指定随 derive 属性添加的代码
            - 类似属性的宏, 可以在任何条目上添加自定义属性
            - 类似函数的宏, 看起来像函数调用, 对其指定为参数的token进行操作

    函数和宏的差别:
    
    本质上, 宏是用来编写可以生成其他代码的代码(元编程, metaprogramming)
    函数在定义签名时, 必须声明参数的个数和类型, 宏可处理可变的参数
    编译器会在解释代码前展开宏
    宏的定义比函数复杂得多, 难以阅读、理解和维护
    在某个文件调用宏时, 必须提前定义宏或将宏引入当前作用域:
    函数可以在任何位置定义并在任何位置使用
*/

// macro_rules! 声明宏(即将弃用)
/* 
    Rust中最常见的宏形式: 声明宏
        - 类似 match 的模式匹配
        - 需要使用 macro_rules!
*/

// 简化实现 vec! 这个宏
// * 标注意味着他所属的宏需要在他所属的包被引入作用域后才可以被使用
#[macro_export]
// 宏的名字是 vec（不带叹号）
macro_rules! vec {
    // * 宏的定义体
    // $x:expr 表示可以匹配任何的rust表达式, 然后将其命名为 $x
    // , 表示在上述捕获代码后面, 会有一个 ","分隔符
    // "*" 表示这个模式可以匹配0个或者多个前面的模式
    ( $( $x:expr ),* ) => {
        // 有点类似match的模式匹配
        return {
            let mut temp_vec = Vec::new();
            $(
                // * 每次匹配都要走到这里一次
                temp_vec.push($x);
            )*
            return temp_vec;
        };
    };
}

// 2. 基于属性来生成代码的过程宏
/* 
    这种形式更像函数(某种形式的过程)一些
        - 接收并操作输入的Rust代码
        - 生成另一些Rust代码作为结果
    三种过程宏:
        - 自定义派生
        - 属性宏
        - 函数宏
    创建过程宏时:
        - 宏定义必须单独放在他们自己的包中, 并使用特殊的包类型
*/
// pub use proc_macro;
// * some_attribute 用于指定过程宏的占位符
// #[some_attribute]
// * TokenStream 表示一段标记序列, 需要被宏处理的源代码, 就组成了输入的 TokenStream, 宏生成的代码则组成输出的 TokenStream
/* pub fn some_name(imput: TokenStream) -> TokenStream {

} */

// 自定义 derive 宏, 见 hello_macro 包

// 类似属性的宏
/* 
    + 属性宏与自定 derive 宏类似
        - 允许创建新的属性
        - 但不是为 derive 属性生成代码
    + 属性宏更加灵活:
        - derive 只能用于 struct 和 enum
        - 属性宏可以用于任意条目, 例如函数
*/

/* 
    #[route(GET, "/")]
    fn index(){}
    如果是Get类型的请求, 并且url是/, 就直接进入, 而route这个属性宏, 就是一个过程宏定义的

    #[proc_macro_attribute]
    pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {}
*/

// 类似函数的宏

/* 
    函数宏定义类似于函数调用的宏, 但比普通函数更加灵活
    函数宏可以接受 TokenStream 作为参数
    与另外两种过程宏一样, 在定义中使用 Rust代码来操作 TokenStream
*/

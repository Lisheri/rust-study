// 高阶函数

/* 
    函数指针
    + 可以将函数传递给其他函数
    + 函数在传递过程中会被强制转换为 fn 类型
    + fn 类型就是 "函数指针(function pointer)"
*/

// 函数指针
fn add_one(x: i32) -> i32 {
    return x + 1;
}

// * fn(i32) -> i32 为一个函数指针, 表示入参是函数
fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    return f(arg) + f(arg);
}

// * 函数指针与闭包的不同
/* 
    1. 闭包就是实现了三种闭包的trait
    
    + 函数指针
        - fn(函数指针)是一个类型, 不是一个trait
            * 可以直接指定fn为参数类型, 不用声明一个以 Fn trait 为约束的泛型参数
        - 函数指针实现了全部3种闭包trait(Fn, FnMut, FnOnce)
            * 总是可以把函数指针用作参数传递给一个接收闭包的函数
            * 所以, 倾向于搭配闭包trait的泛型来编写函数: 可以同时接收闭包和普通函数
        - 某些情景下, 只想接收fn而不接收闭包:
            - 与外部不支持闭包的代码交互: C 函数
    
    + 返回闭包
        - 闭包使用 trait 进行表达, 无法在函数中直接返回一个闭包, 可以将一个实现了该 trait 的具体类型作为返回值
*/

// * 返回闭包
// * 直接返回一个闭包是不行的, 因为Rust无法推断出需要多大的内存空间
// fn returns_closure() -> Fn(i32) -> i32 会报错, 无法推测具体空间
// ? 修复上述报错, 需要使用到Box智能指针
fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    // return Box<>
    return Box::new(|x: i32| x + 1);
}

pub fn use_hof() {
    let answer = do_twice(add_one, 5);
    println!("answer: {}", answer);

    let list_of_nums = vec![1, 2, 3];
    // * 闭包
    let list_of_strings: Vec<String> = list_of_nums.iter().map(|i| i.to_string()).collect();

    let list_of_nums = vec![1, 2, 3];
    // * 函数指针, map中一号参数只要实现了FnMut皆可
    let list_of_strings: Vec<String> = list_of_nums.iter().map(ToString::to_string).collect();

    // * 利用元组结构体和元组结构枚举变体的实现细节
    enum Status {
        Value(u32),
        Stop
    }

    // * 初始化
    // * 这个小括号和函数调用有些相似, 实际上他们也确实被实现成了函数, 这种函数会接收一个参数并返回一个实例
    // * 所以我们可以把这种构造器也作为实现了闭包Trait的函数指针进行使用
    let v = Status::Value(3);

    let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();
}

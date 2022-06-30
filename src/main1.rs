// * 使用ferris_says中导出的say方法
// use ferris_says::say;
// use std::io::{stdout, BufWriter};
/* fn main() {
    // 1. 变量
    // mut 用于声明一个变量, 表示 a 可变
    // 使用 const 定义的 常量 不能使用 mut
    // 常量可以在任何作用域中声明，包括全局作用域，这在一个值需要被很多部分的代码用到时很有用。
    const WORLD: i32 = 10; // * 常量
    let mut a = 10;
    // ! 这样会报错, 在rust中, 变量默认是 immutable(不可变), 也就是说是一个常量。
    // ! rust认为变量可变会产生bug, 比如一部分代码假设一个值用不变化, 而另一部分代码正在尝试改变这个值, 会导致第一部分代码朝着不可预料的方向运行
    // ! 而这样的bug会难以排查, 因为第二部分代码爱只是 有时 会改变值
    a = a + 1;
    print!("变量 a 的值已经更改为: {}, 常量 {}", a, WORLD);
    let stdout = stdout();
    let message = String::from("Hello fellow Rustaceans!");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();
    // println!("Hello, world!");
} */

// 2. 隐藏
fn main() {
    let x = 5;

    // * 重新定义一个和前变量同名的新变量, 成为第一个变量被第二个 隐藏
    // * let 关键字可以多次隐藏

    // ! 这个计算和使用 mut 关键字不同, 隐藏是创建一个新的变量, 实际上之前的x并没有发生变化, 纯粹是开辟一片新的内存空间, 生成了新的x, 甚至可以改变值的类型
    let x = x + 1;
    let spaces = "   ";
    // ! 隐藏可以改变类型, 但是赋值不行
    let spaces = spaces.len();
    // ! 赋值不行
    // let mut spaces = "   ";
    // 下面这样赋值会报错, 类型一旦定义就不能发生变化
    // spaces = spaces.len();

    {
      // * 内部作用域
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }

    println!("The value of x is: {}, spaces: {}", x, spaces);
}

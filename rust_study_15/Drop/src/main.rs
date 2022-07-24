// ! 使用 Drop Trait 运行清理代码
/*
    Drop Trait
    + 实现 Drop Trait 可以自定义当值将要离开作用域时发生的动作
        - 例如: 文件、网络资源的释放
        - 任何类型都可以实现 Drop Trait
    + Drop Trait 只要求开发人员实现一个 drop 方法
        - 参数: 是对 self 的可变引用
    + Drop Trait 在预导入模块里(prelude)
*/

use::std::mem::drop;
// * 例子
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}
fn main() {
    let c = CustomSmartPointer {
        data: String::from("fuck"),
    };

    // * 手动丢弃c, 在drop调用丢弃c的时候, 实际上就是触发了 c::drop(), 同时释放了c
    drop(c);

    let o: CustomSmartPointer = CustomSmartPointer {
        data: String::from("you"),
    };
    println!("CustomSmartPointers created.");
    // * 从运行结果可以看到, 首先调用的是main里的打印
    // * 然后是打印的 o 的 drop
    // * 最后才是 c 的 drop
    // * 这里就可以看到, 确实是 o 先离开作用域
}

/* 
    ! 使用 std::mem::drop 来提前 drop 值
    + 很难禁用自动的 drop 功能, 当然也没有必要
        - Drop Trait的目的就是为了执行自动的释放处理逻辑 
    + Rust 也不允许手动调用 Drop Trait 里面的 drop 方法
        - 这里会报错, 比如上面的c.drop(), 会提示一个错误信息: 显示调用 destructor(析构函数) 是不被允许的
        - 但是会有一个提示, 就是考虑使用 drop 函数: drop(c)
    + 其实这个drop函数就是标准库的 std::mem::drop函数, 用来提前 drop 值
        - 参数就是要丢弃的那个值
    + 其实drop函数也在预导入(prelude)模块里面, 实际上无需显示引入
    + 同时清理后, 该变量也就无法使用了, 也就不会在作用域结束时再次调用清除而导致两次清除
*/


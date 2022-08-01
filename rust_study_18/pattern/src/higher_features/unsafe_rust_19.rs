use std::slice;
/* 
    + 高级特征
        - 不安全 Rust：用于当需要舍弃 Rust 的某些保证并负责手动维持这些保证
        - 高级 trait：与 trait 相关的关联类型，默认类型参数，完全限定语法（fully qualified syntax），超（父）trait（supertraits）和 newtype 模式
        - 高级类型：关于 newtype 模式的更多内容，类型别名，never 类型和动态大小类型
        - 高级函数和闭包：函数指针和返回闭包
        - 宏：定义在编译时定义更多代码的方式
*/

// + 1. 不安全 Rust
/* 
    目前为止, 讨论过的代码都有Rust在编译时会强制执行的内存安全保证。
    然而, Rust还隐藏有第二种语言, 它不会强制执行这类内存安全保证: 这被称为 不安全 Rust。
        - 和普通的Rust一样, 但提供了额外的"超能力"
    尽管代码可能没问题, 但如果Rust编译器没有足够的信息可以确定, 它将拒绝代码
    + Unsafe Rust 存在的原因:
        - 静态分析是保守的（编译器判断一段代码是否拥有安全保证时, 宁可错删一些合法的程序, 也不会接收仅仅是可能非法的代码, 尽管这些代码是安全的）
            - 使用 Unsafe Rust: 我知道我在做什么, 并且自己承担相应风险
        - 计算机硬件本身就是不安全的, Rust 需要能够进行底层系统编程
    + Unsafe 超能力
        - 使用 unsafe 关键字来切换到 unsafe Rust, 开启一个块, 里面放着 unsafe 代码
        - Unsafe Rust 里可执行的四个动作(unsafe超能力):
            - 解引用原始指针
            - 调用 unsafe 函数或方法
            - 访问或修改可变的静态变量
            - 实现 unsafe trait
        - 注意:
            - unsafe 并没有关闭借用检查或停用其他安全检查(如果在unsafe 代码里使用引用, 那么这个引用依然会被检查)
            - 所以说即便是在unsafe 代码块中, 依然可以获得一定程度的安全性
            - 任何内存安全相关的错误必须留在 unsafe 块里
            - 尽可能隔离 unsafe 代码, 最好将其封装在安全的抽象里, 提供安全的API(某些库是如此实现的)
*/
pub fn unsafe_rust_origin_pointer() {
    // * 解引用原始指针(裸指针)
    /* 
        + 原始指针
            - 可变的: *mut T
            - 不可变的: *const T。意味着指针在解引用后不能直接对其进行赋值
            - 注意: 这里的 * 不是解引用符号, 它是类型名的一部分。
        + 与引用不同, 原始指针:
            - 允许通过同时具有不可变和可变指针 或 多个指向同一位置的可变指针来忽略借用规则
            - 无法保证能指向合理的内存(而引用总是合理)
            - 允许为null
            - 不识闲任何自动清理
        + 放弃保证的安全, 换取更好的性能/与其它语言或硬件接口的能力
    */

    let mut num = 4;
    // * 以下是两个原始指针, 一个不可变, 一个可变
    // * 可以在不安全代码块之外创建原始指针, 但是只能在 unsafe 块内对原始指针进行解引用
    /* 
        r1 和 r2 其实是指向同一块内存地址, 只不过一个是不可变的, 一个是可变的
        此时就可以通过可变的引用 r2, 来修改 r1 的值(但是这个操作非常危险, 需要小心)

        ? 为什么要使用原始指针
            + 与C语言进行接口
            + 构建借用检查器无法理解的安全抽象
    */
    let r1 = &num as *const i32; // 不可变（有效的）
    let r2 = &mut num as *mut i32; // 可变（有效的）

    // * 尝试解引用
    // ? 此时已经报错, 错误为 this operation is unsafe and requires an unsafe function or block(解引用原始指针是不安全的, 需要将代码放在 unsafe 函数或代码块中)
    // print!("r1: {}", *r1);
    // print!("r2: {}", *r2);
    // * 如下
    unsafe {
        println!("r1: {}", *r1);
        println!("r2: {}", *r2);
    }
    // * 创建无法确认其有效性的原始指针
    let address = 0x012345usize; // * 该地址可能有数据, 可能没有数据, 但是依然可以创建原始指针
    let r = address as *const i32;
    unsafe {
        // * 静态编译没有报错, 但是这里如果编译报错, 也是我们自己负责
        // * 这里会报错, 属于非法访问
        println!("r: {}", *r);
    }
}

// * 调用 unsafe 函数或方法
pub fn unsafe_rust_use_unsafe_fn() {
    /* 
        + unsafe 函数或方法: 在定义前加上了 unsafe 关键字
            - 调用前需要手动满足一些条件(文档), 因为 Rust 无法对这些条件进行验证
            - 需要在 unsafe 块里进行调用
    */
    unsafe fn dangerous() {
        println!("使用了unsafe函数")
    }

    // 报错: 调用unsafe 函数的操作是不安全的, 需要在不安全块内或函数内操作
    // dangerous();

    unsafe {
        dangerous();
    }
}

// * 创建 unsafe 代码的安全抽象
pub fn unsafe_rust_safe_abstract() {
    /* 
        + 函数包含 unsafe 代码并不意味着需要将整个函数标记为 unsafe
        + 将 unsafe 代码包裹在安全函数中是一个常见的抽象
    */

    let mut v = vec![1, 2, 3, 4, 5, 6];
    // * 对 vector的完整切片 r
    let r = &mut v[..];
    // * split_at_mut 定义在可变的切片上, 接收一个切片, 将这个切片从给定的索引, 将其分割为两个切片
    // * 这个函数内部其实就是引用了不安全的代码(如下)
    let (a, b) = r.split_at_mut(3);
    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
        let len = slice.len();

        // * as_mut_ptr 会返回一个原始指针， *mut i32, 指向切片本身
        let ptr = slice.as_mut_ptr();

        assert!(mid <= len);

        // * 这里其实第二个slice有错, 因为从规则上不允许这个操作, 也就是重复的可变借用
        // * 但实际上应该是可以的, 他是借用的一个切片的两个不同部分, 是没有交叉的, 所以这里其实可以使用 unsafe
        unsafe {
            return (
                // * 将切片分成两部分
                // ? from_raw_parts_mut函数, 接收一个原始指针 和 一个长度来创建切片, 就是从 ptr 指针处, 创建拥有 mid 个元素的切片(也就是mid所代表的的个数)
                // ? 这个slice 是 std::slice
                // ! 由于下面使用到了原始指针和偏移量, 所以他们肯定是不安全的, 调用他们就需要在不安全的代码块里面
                slice::from_raw_parts_mut(ptr, mid),
                // ? add 方法, 表示在这个指针上, 以 mid 作为偏移量, 得到一个以 ptr 开头的指针, 而长度就是 len - mid
                slice::from_raw_parts_mut(ptr.add(mid), len - mid)
            );
        }
        // ! 该函数内部有不安全的代码块, 但是函数本身并没有被标记为不安全的代码, 所以函数本身就是一个不安全代码的安全抽象
    }
}

// * 使用 extern 函数调用外部代码
pub fn use_extern() {
    /* 
        + extern 关键字: 简化创建和使用外部函数接口(FFI)的过程
        + 外部函数接口(FFI: Foreign Function Interface): 它允许一种编程语言定义函数, 并让其他编程语言能调用这些函数
        + 应用二进制接口(ABI, Application Binary Interface): 定义函数在汇编层的调用方式
        + “C” ABI 是最常见的 ABI, 它遵循 C 语言的 ABI
        ? 任何在extern 中声明的函数都是不安全的
    */
    // * 这个主要是用于定义函数在汇编层面的调用方式
    extern "C" {
        fn abs(input: i32) -> i32;
    }

    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }

    // * 从其他语言调用Rust函数
    // + 可以使用 extern 创建接口, 其他语言通过他们可以调用 Rust的函数
    // + 在fn 前添加 extern 关键字, 并指定 ABI
    // + 还需要添加 #[no_mangle]注解: 避免Rust在编译时改变它的名称(mangle就是对应一个特殊的编译阶段, 在这个阶段, 编译器会修改函数的名称, 从而让其包含更多可用于后续编译的信息)
    //      - 这些改变后的名称通常是难以阅读的, 为了让其他语言正常识别rust函数, 我们就要禁止rust编译器让其改名
}

// * 这个函数就可以被C语言访问了, 这个 extern 功能不需要使用 unsafe
#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}

// * 访问或修改一个可变静态变量

/* 
    + Rust 支持全局变量, 但因为所有权机制可能产生某些问题, 例如数据竞争
    + 在Rust里, 全局变量叫做静态(static)变量
*/
// * 命名规范是大写的 下划线, 并且需要声明类型, 同时他还有一个生命周期
// * 静态变量只能存储拥有一个 'static 这个生命周期的引用, 这也意味着rust编译器可以推断出其生命周期, 所以无需显示标注
// * 访问不可变的静态变量是安全的
pub static HELLO_WORLD: &str = "hello world";

// ? 常量和不可变静态变量的区别
// * 静态变量: 有固定的的内存地址, 使用它的值总是会访问同样的数据
// * 常量: 允许使用他们的时候对数据进行复制
// * 静态变量: 可以是可变的, 访问和修改静态可变变量是不安全(unsafe)的操作

pub static mut COUNTER: u32 = 0;

pub fn add_to_count(inc: u32) {
    unsafe {
        // * 修改操作是不安全的
        COUNTER += inc;
    }
}

pub fn use_static_fn() {
    println!("name is {}", HELLO_WORLD);

    add_to_count(3);
    unsafe {
        // * 访问静态变量也是不安全的
        // * 多线程的话就会出现数据竞争, 针对多线程的情况, 尽量使用并发技术或者线程安全的智能指针, 从而让编译器能够对多线程进行检查
        println!("COUNTER: {}", COUNTER);
    }
}

// * 实现不安全(unsafe) trait
// + 当某个 trait 中存在至少一个方法拥有编译器无法校验的不安全因素时, 就称这个 trait 是不安全的
// + 声明 unsafe trait: 在定义前加 unsafe 关键字
//      - 该trait只能在unsafe 代码块中实现
// * 定义
unsafe trait Foo {

}
// * 实现
unsafe impl Foo for i32 {

}

// * 何时使用 unsafe 代码
// * 编译器是无法保证内存安全, 保证 unsafe 代码正确并不简单
// * 有充足理由使用unsafe 代码时, 就可以这样做
// * 通过显式标记 unsafe, 可以在出现问题时轻松的定位

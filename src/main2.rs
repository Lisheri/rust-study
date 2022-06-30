// * rust 类型系统
// * rust是静态语言
// * 共有两类数据类型子集: 标量(scalar)和复合(compound)
// * 翻转
// fn reverse(pair: (i32, f64, u8)) -> (u8, f64, i32) {
//   // 可以使用 `let` 把一个元组的成员绑定到一些变量
//   let (integer, float, u8) = pair;

//   (u8, float, integer)
// }
fn main() {
    // * 类型转换时必须添加类型注解, 如下
    let guess: u32 = "42".parse().expect("Not a number!"); // 若不添加注解则报错

    println!("值为: {}", guess);

    // ! 标量(scalar): 代表一个单独的值, Rust有四种基本的标量类型: 整形、浮点型、布尔型和字符类型。
    // ? 1.1 整形
    // ? 有符号 和 无符号 代表数字能否为负值, 有负值则为有符号
    // 长度       有符号      无符号
    // len-bit   i${len}    u${len}
    // len取值: 8 16 32 64 128 arch
    // * 每一个有符号的变体可以储存包含从 -(2^(n - 1)) 到 2^(n - 1) - 1 在内的数字，这里 n 是变体使用的位数。(i8 则是 8位)
    // * 无符号则存储 0 到 (2^n) - 1
    // * isize 和 usize 类型依赖运行程序的计算机架构：64 位架构上它们是 64 位的， 32 位架构上它们是 32 位的。
    // ? 可以使用如下任何一种形式编写数字字面值。 可以是多种数字类型的数字字面值允许使用类型后缀, 比如 58u8, 可以作为类型来指定。
    // ? "_" 可以用于分隔十进制数字, 便于读数, 比如1_000 和 1000没有区别
    // * 整数字面值
    // 数字字面值	                  例子
    // Decimal(十进制)             98_222
    // Hex (十六进制)	             0xff
    // Octal (八进制)	             0o77
    // Binary (二进制)	           0b1111_0000
    // Byte (单字节字符)(仅限于u8)	 b'A'

    // * 如果数字超越类型定义的极限, 比如 let x: u8 = 256; 则发生 整形溢出。 会产生两个现象
    // * 1.debug模式下, rust检查这类问题, 并使程序直接 panic(退出)。
    // * 2.release 构建, rust不检测溢出, 但是会进行二进制补码包装, 比如 256 -> 0, 257 -> 1
    // * 在标准库中有一个类型显示提供此功能, Wrapping。 为了显示的处理溢出的可能性, 可以使用标准库在原生数值类型上提供的以下方法:
    // * 所有模式下都可以使用 wrapping_* 方法进行包装, 如 wrapping_add
    // * 如果 check_* 方法出现溢出, 则返回 None值
    // * 用 overflowing_*方法返回值和一个布尔值, 表示是否出现溢出
    // * 用 saturating_* 方法在值的最小值或最大值处进行饱和处理

    // ? 1.2 浮点型
    // * rust一共两个浮点型, f32 和 f64, 默认类型是f64, 因为在现代CPU中, 它与 f32 的速度几乎一模一样, 不过精度更高。
    // ! 所有浮点型都是有符号的
    // * f32 是单精度浮点数, f64是双精度浮点数(采用 IEEE-754标准)

    let f64 = 2.0; // f64
    let f32: f32 = 3.0; // f32
                        // * 整数除以整数还是整数
    println!("浮点数: {}, {}, {}", f64, f32, 4 / 3);

    // ? 1.3 布尔值
    // * bool, true 和 false
    const a1: bool = true;
    const a2: bool = false;
    println!("boolean: {}, {}", a1, a2);

    // ? 1.4 字符类型(char)
    // * 声明char, 用单引号(双引号用于声明字符串)
    // * char类型的大小为四个字节(four bytes)，并代表了一个 Unicode 标量值（Unicode Scalar Value）
    // ? 在 Rust 中，拼音字母（Accented letters），中文、日文、韩文等字符，emoji（绘文字）以及零长度的空白字符都是有效的 char 值
    // ? “字符” 并不是一个 Unicode 中的概念, 在 RUST 中他和字符串是有区分的, 字符串用于存储 UTF-8 编码文本
    const a3: char = 'Z';
    println!("字符串: {}", a3);

    // ! 2. 符合类型 可以将多个值组合成一个类型, Rust有两个原生的复合类型: 元组(tuple) 和 数组(array)

    // ? 2.1 元组类型
    // * 元组是一个将多个其他类型的值组合进一个复合类型的主要方式。
    // * 元组长度固定: 一旦声明, 长度不会增大或缩小
    // * 创建元祖: (元组中每一个位置都有一个类型, 而且这些不同值的类型也不必是相同的)
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // * 通过 . 操作符, 可以访问具体值
    println!("元组: {}", tup.0);
    // * 整体打印（但是元组长度不能太长, 否则无法打印）
    println!("tuple of tuples: {:?}", tup);
    // * 元组可以使用 "()" 解构
    let (tupx, tupy, tupz) = tup;
    println!("tuple of tuples: {}, {}, {}", tupx, tupy, tupz);

    // * 没有任何值的元组: (), 是一种特殊类型, 只有一个值, 也写成(), 该类型被称为: 单元类型, 而其中如果只有一个值, 这个值叫做: 单元值。 如果表达式不返回任何其他值, 则会隐式返回单元值

    // ? 2.2 数组类型, 在堆栈上分配的已知固定大小的单个内存块
    // ! 数组中每一个元素的类型必须相同
    // ! 同时rust中数组的长度是固定的
    // ! [1, 2, 3]
    // * 标准库提供了一个可以增减长度的类似数组的集合类型, 叫做 vector
    // * 因此在确保长度不变时, 使用 array 非常方便, 如果长度不确定, 则可以使用 vector
    // * 数组定义
    let arr1: [i32; 5] = [1, 2, 3, 4, 5];
    println!("item of arrays: {:?}", arr1);
    // * 如果每个元素都相同
    let arr2 = [3; 100]; // * 100个3
    println!("item of arrays: {:?}", arr2);

    // * 数组无效访问会报错, 直接panic

    // ? 3. 函数
    // * 函数中赋值语句并不会返回目标值, 因此不能 x = y = 18, 在rust中会报错, 但是代码块可以赋值

    let tst_block = {
        let x = 3;
        x + 1 // * 这里不能加分号, 加了分号则变为语句, 语句没有返回值, 会直接报错
    };

    println!("The value of y is: {}", tst_block); // * 4

    // ? 4. if else
    // * 条件必须是bool, rust不会自动转bool
    // * 如果把if else 当三元表达式用, 需要拿到返回值, 则类型需要统一, 否则报错

    // ? 5. 循环
    // * rust主要有三种循环: loop、while和for
    // * loop 会不停执行, 直到退出
    // * 如下所示就会一直执行, 直到command + c
    /* loop {
        println!("again!");
    } */
    // * 当然, break 和 continue对loop依然生效
    // * 如下循环, 给外层循环定义一个循环标签, 叫做 counting_up, 于是break: counting_up则退出外层循环, 单独的break, 退出内层循环
    /* let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {}", count); */

    // * break + 值, 可以作为该作用域的返回值
    // * 比如如下返回值为20
    /*
      let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    */
    println!("华氏度转换摄氏度为: {}", resolveTemp(12.0, true));
}

// * 摄氏度 华氏度 互转
fn resolveTemp(origin: f64, isToF: bool) -> (f64) {
    return if isToF {
        (9.0 * origin) / 5.0 + 32.0
    } else {
        5.0 * (origin - 32.0) / 9.0
    };
}

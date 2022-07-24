/* 
    1. 通过 Deref trait 将智能指针当作常规引用处理
    * 实现 Deref trait 允许开发者重载解引运算符 * 
*/
use::std::ops::Deref;
/* 
    + 1. 解引运算符处理常规引用
    + 2. 定义类似Box<T>, 查看为何解引运算符不能像引用一样工作
    + 3. 如何实现 Deref trait 使得智能指针以类似引用的方式工作变为可能
    + 4. Rust 的 Deref 强制转换（deref coercions）功能
    - 如何处理引用
    - 如何处理智能指针
*/
fn main() {
    // ! MyBox<T> 与 Box<T>有很大的区别, 我们不会在堆上存储数据
    // ! 重点关注 Deref，所以其数据实际存放在何处，相比其类似指针的行为来说不算重要。
    
    // 1. 通过解引运算符追踪指针的值
    // 常规引用是一个指针类型，一种理解指针的方式是将其看成指向储存在其他某处值的箭头。
    let x = 5;
    let y = &x; // y 是 x的一个引用
    assert_eq!(5, x);
    assert_eq!(5, *y); // 如果希望对y的值做出断言, 必须将y解除引用, 从而访问y指向的整形值
    // assert_eq!(5, y); // ! 这个比较是不允许的, 因为数字的引用和数字并不是同一个类型

    // * 像引用一样使用Box<T>
    let x = 5;
    // 使用Box<T>来代替上述引用, 解引运算符也同样能够工作
    /* 
        此处与上面不同的是, 此处的y指向的是x值拷贝的box实例, 而不是指向x的引用。
        在最后的断言中，可以使用解引用运算符以 y 为引用时相同的方式追踪 box 的指针
    */
    let y = Box::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y);

    // * 使用MyBox<T>
    let x = 5;
    let y = MyBox::new(x);
    assert_eq!(5, x);
    // 此处会报错, 因为此时的Rust不知道如何对MyBox<T>的实例解引用
    // MyBox不能解引用是因为此时我们还没有实现该类型的解引用trait, 也就是Defer trait
    // 为了启用 * 运算符的解引用功能，需要实现 Deref trait。
    assert_eq!(5, *y);
    // * 强制类型转换
    let name = MyBox::new(String::from("fuck"));
    // * 这里 name 入参为 &String, 但是 hello 接收的函数为 &str
    // * 这里使用 &m 调用 hello 函数，其为 MyBox<String> 值的引用。
    // * MyBox上实现了 deref, Rust可以通过 deref 调用, 将 &MyBox<String>转换为 &String
    // * 标准库提供了 String的 Deref实现, 其会返回字符串 slice，这可以在 Deref 的 API 文档中看到
    // * 于是Rust再次调用 deref, 将 &String 转换为 &str, 这样就符合 hello 的函数定义了
    hello(&name);

    // * 如果String上没有实现Deref强制类型转换, 则需要如下调用来转换 &String -> &str
    hello(&(*name)[..]);
    // * (*name) 将 MyBox<String> 解引用为 String;
    // * 接着 & 和 [..] 获取了整个 String 的字符串 slice 来匹配 hello 的签名
    // * 没有 Deref 强制转换所有这些符号混在一起将更难以读写和理解。Deref 强制转换使得 Rust 自动的帮我们处理这些转换。

    // * 当所涉及的类型定义了Deref trait, Rust会分析这些类型并使用任意多次 Deref::deref 调用以获得匹配类型的参数
    // * 这些解析都发生在编译时, 所以利用Deref的强制类型转换并没有运行时的损耗

    // * Deref 强制转换与可变性交互
    // * 类似于如何使用 Deref trait重载不可变引用的 * 运算符, Rust提供了 DerefMut trait 用于重载可变引用的 * 运算符
    // * Rust 在发现类型和 trait 实现满足三种情况时会进行 Deref 强制转换(不可变到不可变, 可变到可变, 可变到不可变)：
        // - 当 T: Deref<Target=U> 时从 &T 到 &U。
        // - 当 T: DerefMut<Target=U> 时从 &mut T 到 &mut U。
        // - 当 T: Deref<Target=U> 时`从 &mut T 到 &U。
    // * 头两个情况除了可变性之外是相同的: 第一种情况表明如果有一个 &T, 而 T 实现了返回 U类型的 Deref, 则可以得到 &U
    // * 第二种情况表明可变引用和不可变引用之间的转换是一样的
    // * 第三个情况有些微妙：Rust 也会将可变引用强转为不可变引用。但是反之是 不可能 的
    // * 不可变引用永远也不能强转为可变引用。因为根据借用规则，如果有一个可变引用，其必须是这些数据的唯一引用（否则程序将无法编译）。
    // * 将一个可变引用转换为不可变引用永远也不会打破借用规则。将不可变引用转换为可变引用则需要初始的不可变引用是数据唯一的不可变引用，而借用规则无法保证这一点。
    // * 因此，Rust 无法假设将不可变引用转换为可变引用是可能的。
}

// 自定义智能指针
// 智能指针与引用有什么不同?
// 从根本上说，Box<T> 被定义为包含一个元素的元组结构体
// * 此处声明了一个元祖结构体MyBox, 包含了一个泛型参数T, 本质上来说, MyBox可以存放任意类型
// * MyBox::new 获得一个T类型的参数, 返回一个存放传入值的MyBox实例
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        return MyBox(x);
    }
}

// * 通过 Deref trait 将某类型像引用一样处理
// * Deref trait 由标准库提供, 要求实现一个名为 deref 的方法, 其借用 self 返回一个内部数据的引用
// * 下面将在MyBox上实现一个 Deref
impl<T> Deref for MyBox<T> {
    // ? 关联类型是一个稍有不同的定义泛型参数的方式
     // 定义了用于此 trait 的关联类型
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // * 这里就是通过0这个下标来访问元组的第一个元素
        // * 现在使用*解引运算符就可以通过编译了
        // * Deref trait 中的 deref 方法告诉编译器如何解引用运算, 在没有Deref trait之前, 编译器只能解除 & 的引用
        // * 实际上在使用 *y 时, 编译器在底层就是在执行 *(y.deref())
        // * 这一步代理实际上是统一了解引用的过程, 让开发者可以写出一致性的代码, 而不是针对引用和实现了Deref的类型单独处理解引用
        // * 这里返回一个引用的原因也是在于所有权, 如果这里返回的不是一个引用而是一个值, 那么当 deref 执行完毕后, MyBox的值(所有权)将被回收
        &self.0
    }
}

// ! 注: 在使用解引运算符时, * 都会被替换成调用 deref 方法, 在接着使用 * 的操作, 且只发生一次, 不会无限递归解引, 只要计算得出 类型<T>的值, 就停止操作

// * 函数和方法的隐式 Deref 强制转换

/* 
    Deref 强制转换是 Rust 在函数传参上的一种便利
    Deref 强制转换只能用于实现了 Deref trait的类型
    Deref 强制转换将这样一个类型的引用转换为另一个类型的引用
    比如说 &String -> &str, 因为 String 实现了 Deref trait, 因此可以返回 &str
    当这种特定类型的引用作为实参传递给形参类型不同的函数或方法时, Deref强制转换将自动发生,
    此时会有一系列deref 方法被调用, 直到将入参类型转换为形参类型

    Deref 强制转换的加入使得 Rust 程序员编写函数和方法调用时无需增加过多显式使用 & 和 * 的引用和解引用。
    这个功能也使得我们可以编写更多同时作用于引用或智能指针的代码。
*/

// * Deref 强制类型转换
fn hello(name: &str) {
    println!("名字是: {}", name);
}

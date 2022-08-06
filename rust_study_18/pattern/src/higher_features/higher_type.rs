// 高级类型


// 使用 newtype 模式实现类型安全和抽象
/* 
    + newtype 模式可以:
        - 用来静态的保证各种值之间不会混淆并表明值的单位
        - 为类型的某些细节提供抽象能力
        - 通过轻量的封装来隐藏内部的实现细节

    使用类型别名创建类型同义词
    + Rust 提供了类型别名的功能
        - 为现有类型产生另外的名称(同义词)
        - 并不是一个独立的类型
        - 使用type关键字
    + 主要用途: 减少代码字符重复

    + Never 类型
    + 有一个名为 ! 的特殊类型:
        - 它没有任何值, 行话称为空类型 (empty type)
        - 我们倾向于叫他 never 类型, 因为它在不返回的函数中充当返回类型
    + 不返回值的函数也被称作发散函数 (diverging function)


    + 动态大小和Sized Trait
    + Rust 需要在编译时确定为一个特定类型的值分配多少空间
    + 动态大小的类型(Dynamically Sized Types, DST)的概念"
        - 编写代码时使用只有在运行时才能确定大小的值
    + str 是动态大小的类型(注意, 不是&str, &str是字符串切片): 只有运行时才能确定字符串的长度
    下面代码无法正常工作:
    let s1: str = "Hello there!";
    let s2: str = "How's it going?";
    因为rust必须在编译时确定一个类型占据多少空间, 而同一类型所有的值, 必须使用等量的内存空间
        - 使用 &str 来解决:
            * str的地址
            * str的的长度
            ? 所以&str的大小肯定是固定的
    
    + Rust 使用动态大小的通用方式:
        + 附带一些额外的元数据来存储动态信息的大小
            - 使用动态大小类型时总会把它的值放在某种指针后面
    + 另外一种动态大小的类型: trait
        - 每个trait都是一个动态大小的类型, 可以通过名称对其进行引用
        - 为了将trait 用作 trait 对象, 必须将它放置在某种指针之后
            ? 例如 &dyn Trait 或 Box<dyn Trait>(Rc<dyn Trait>)之后
    + Sized trait
        + 为了处理动态大小的类型, Rust提供了一个 Sized trait 来确定一个类型的大小在编译时是否已知
            - 编译时可计算出大小的类型会自动实现这一trait
            - Rust还会为每一个泛型函数隐式的添加Sized约束
            fn generic<T>(t: T) {} => fn generic<T: Sized>(t: T) {}
        + 默认情况下, 泛型函数只能被用于编译时已经知道大小的类型, 可以通过特殊语法解除这一限制
        + ?Sized trait 约束
        fn generic<T: ?Sized>(t: &T) {}
        这个?Sized表达了一种不确定性, 就是这个T, 可能是Sized类型, 也可能不是, 不过这个?, 只能用于Sized上, 不能用于其他Trait
        注意, 上面的T, 已经变成了 &T, 因为他可能是不确定的, 只能放在某种指针后面, 这里使用的是 引用
*/

type Kilometres = i32;

pub fn use_kilometers() {
    let x: i32 = 5;
    // * 他其实只是个类型别名, 根本上还是i32
    let y: Kilometres = 5;
    println!("x + y: {}", x + y);
}

// * 例2
type Thunk = Box<dyn Fn() + Send + 'static>;
fn takes_long_type(f: Thunk) {

}

fn returns_long_type() -> Thunk {
    Box::new(|| println!("hi"))
}

pub fn use_box_type() {
    let f: Thunk = Box::new(|| println!("hi"));
}

// 减少 Result<T, E>中E的重复
// 写死一个Error, 当然下面这个其实是写死在 std::io::Result<T>中的
// type Result<T> = Result<T, std::io::Error>;

// 发散函数
// 但是发散函数并不是说什么都不返回, 只是类型未知(never)而已
// 但其实这个函数签名确实是表示bar函数永远不会返回值
/* fn bar() -> ! {

} */

fn use_diverging_fn() {
    let guess = "";
    loop {
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            // * match必须返回对应的类型, 而这个continue就是never(!)类型
            // * 而never无法返回一个可供返回的值, 所以这个match返回的就是上面那个num的类型, 也就是u32类型
            // * never类型的表达式就被强制的转换为其他的任意类型了
            // * never类型可以被强制转换为其他的任意类型
            // * 也就是说下面continue被强制转换为了u32, 所以这两个分支的返回类型是一样的
            Err(_) => continue
        };
    }

    // panic! 的返回类型也是 never
    /* 
        match self {
            Some(val) => val,
            这个 panic! 就是一个 never, 也就被强制转换为了val
            None => panic!("...")
        }
    */
}

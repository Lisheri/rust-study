// TODO 枚举
fn main() {
    #[derive(Debug)]
    enum IpAddrKind {
        v4(String),
        v6(String),
    }
    let four = IpAddrKind::v4(String::from("127.0.0.1"));
    let six = IpAddrKind::v6(String::from("::6"));
    println!("v4: {:?}, v6: {:?}", four, six);

    // enum UsState {
    //     Alabama,
    //     Alaska,
    //     // --snip--
    // }

    // ? match: 控制流运算符，允许我们将一个值与一系列的模式相比较，并根据相匹配的模式执行相应代码。
    // enum Coin {
    //     Penny,
    //     Nickel,
    //     Dime,
    //     Quarter(UsState),
    // }

    // fn value_in_cents(coin: Coin) -> u8 {
    //     match coin {
    //         Coin::Penny => 1,
    //         Coin::Nickel => 5,
    //         Coin::Dime => 10,
    //         Coin::Quarter => 25,
    //     }
    // }

    // println!("tst coin: {}", value_in_cents(Coin::Quarter));

    /*
        空值尝试表达的概念仍然是有意义的：空值是一个因为某种原因目前无效或缺失的值。
        问题不在于概念而在于具体的实现。为此，Rust 并没有空值，不过它确实拥有一个可以编码存在或不存在概念的枚举。
        也是是options

        enum Options<T> {
          Some(T),
          None,
        }

        他被定义在了标准库中, 无需引入

        如果使用 None 而不是 Some，需要告诉 Rust Option<T> 是什么类型的
        let none: Option<i32> = None;
        let somet = Some(5);

        同时: Option<T> 和 T（这里 T 可以是任何类型）是不同的类型, 因此任何运算都会报类型错误, 主要是为了安全, 可以避开空值
    */

    // ? 匹配 Options<T>, match一个Option 枚举必须对 None 做处理, 缺少 空值 处理直接报错
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            // * 这个地方一定是匹配的, 只要 i 还包含在 i32内, 就一定匹配, 无论i是什么值
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("five: {:?}, six: {:?}, none: {:?}", five, six, none);
    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        // _ 做default匹配
        _ => {
            println!("进入默认值")
        }
    }

    // ? if let 简单控制流, 可以看做 match 的语法糖
    // * 如果只是一个简单的匹配, 如下所示:
    /*
        let some_u8_value = Some(0u8);
        match some_u8_value {
          Some(3) => println!("three"),
          _ => (),
        }
    */
    // * 上述路径过于繁琐, 而我们只关心对 Some(3)的匹配, 但是不想处理任何其他的值, 为了满足 match 表达式(穷尽性)的要求, 必须在处理完这一唯一的成员后加上 _ => (), 这样也要增加很多样板代码。
    // * 于是我们可以使用if let, 这种更短的方式编写, 如下:
    let some_u8_value = Some(0u8);
    // ! some_u8_value 的类型必须与 let 后的类型一致, 否则报错
    if let Some(3) = some_u8_value {
        println!("three");
    }

    // * 可以包含一个else块, else 块中的代码与 match 表达式中的 _ 分支块中的代码相同
    // fn value_in_cents(coin: Coin) -> u8 {
    //     match coin {
    //         Coin::Penny => 1,
    //         Coin::Nickel => 5,
    //         Coin::Dime => 10,
    //         Coin::Quarter(state) => {
    //             println!("State quarter from {:?}!", state);
    //             25
    //         }
    //     }
    // }
    // * 不加调试模式无法打印枚举变量
    #[derive(Debug)]
    enum UsState {
        Alabama,
        Alaska,
        // --snip--
    }

    // ? match: 控制流运算符，允许我们将一个值与一系列的模式相比较，并根据相匹配的模式执行相应代码。
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }
    // fn tst_valid_in_count(coin: Coin) -> u8 {
    //     let mut count = 0;
    //     match coin {
    //         Coin::Quarter(state) => {
    //             println!("State quarter from {:?}!", state);
    //             25
    //         }
    //         _ => {
    //             count += 1;
    //             return count;
    //         }
    //     }
    // }
    let coin = Coin::Quarter(UsState::Alabama);
    let mut count = 0;
    if let Coin::Quarter(UsState) = coin {
        println!("State quarter from {:?}!", UsState);
    } else {
        count += 1;
        println!("count is: {}!", count);
    }
}

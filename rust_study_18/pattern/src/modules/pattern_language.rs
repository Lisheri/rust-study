// * 匹配字面值
pub fn match_literal_value() {
    // * 模式可以直接匹配字面值
    let x = 1;
    match x {
        // * 这些 1 2 3 就是字面值, 可以直接匹配
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("other"),
    }
}

// * 匹配命名变量
pub fn match_name_variable() {
    // * 命名变量是可匹配任何值的无可辩驳模式
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        // * 这个 y 其实是一个新的变量, 存在于match的块级作用域中
        // * 这个表示x是一个 Option枚举的变量, 所以到这里就匹配成了, 而 y 就是 5, 并且外部的y并没有进入作用域内, 也就不会被回收了
        Some(y) => println!("Matched, y = {:?}", y),
        _ => println!("Default case, x = {:?}", x),
    }

    // * 所以到这里y还存在
    println!("at the end: x = {:?}, y = {:?}", x, y);
}

// * 多重模式
// * 范围匹配: 使用 ..= 来匹配某个范围的值
// * 解构以分解值
pub fn multiple_mode() {
    // + 在 match 表达式中, 使用 | 语法(就是或的意思), 可以匹配多种模式
    let x = 1;
    // let x = 4;
    match x {
        // * 表示 1 和 2都可以匹配上
        1 | 2 => println!("中了, 是 1 或者 2"),
        // * 表示 x 是 3 4 5
        3..=5 => println!("是 3 到 5"),
        _ => println!("滚你吗")
    }

    let y = 'c';
    match y {
        // * 字符也可以是范围匹配
        'a' ..= 'j' => println!("early ASCII letter"),
        'k' ..= 'z' => println!("late ASCII letter"),
        _ => println!("其他")
    }

    // * 可以使用模式来解构 struct、enum、tuple, 从而引用这些类型值的不同部分
    struct Point {
        x: i32, 
        y: i32
    }

    let p = Point {x: 0, y: 8};
    // * 解构并重命名, 也可以直接解构 let { x, y } = p;
    let Point {x: a, y: b} = p;
    assert_eq!(0, a);
    assert_eq!(8, b);

    match p {
        // * 匹配 x 为任意值, 但是 y 必须是0的 Point 实例 
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y)
    }


    // * 解构枚举
    /* enum Message {
        Quit,
        Move {x: i32, y: i32},
        Write(String),
        ChangeColor(i32, i32, i32)
    }

    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.");
        },
        // 匹配到枚举中的 Move, 同时解构x, y
        Message::Move {x, y} => {
            println!("Move in the x direction {} and in the y direction {}", x, y);
        },
        // * 元组里有一个元素, 可以取出来
        Message::Write(text) => println!("the message is {}", text),
        Message::ChangeColor(x, y, z) => {
            println!("Change the color to red {}, green {}, blur {}", x, y, z);
        }
    } */

    // * 解构嵌套的结构体和枚举
    enum Color {
        Rgb(i32, i32, i32),
        Hsv(i32, i32, i32)
    }

    enum Message {
        Quit,
        Move {x: i32, y: i32},
        Write(String),
        ChangeColor(Color)
    }

    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        // * 层层匹配, 然后解构元组
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change the color to red {}, green {} and blue {}", r, g, b);
        },
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("Change the color to hue {}, saturation {} and value {}", h, s, v);
        },
        _ => ()
    }

    // * 解构结构体和元组
    struct Point2 {
        x: i32,
        y: i32
    }

    // * 一一对应即可
    let ((feet, inches), Point2 { x, y }) = ((3, 10), Point2 { x: 3, y: -10});
}

// * 在模式中忽略值
pub fn ignore_value() {
    // * 有几种方式可以模式中忽略整个值或部分值:
    /* 
        - _ 就是忽略整个值
        - _ 配合其他模式, 可以用于忽略部分值
        - 使用以 _ 开头的名称, 忽略未使用的变量
        - ..(忽略值的剩余部分) 
    */
    // + 使用 _ 忽略整个值
    // * 这个函数的第一个参数用不上, 所以用 _ 忽略整体
    fn foo (_: i32, y: i32) {
        println!("使用第二入参: {}", y);
    }
    // * 这个10 就是没有用的
    foo(10, 100);

    // + 使用 嵌套的_ 忽略值的一部分
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        // * 这个表示匹配到这两个元素都是Some类型就可以了, 忽略里面的值
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        },
        _ => {
            setting_value = new_setting_value;
        }
    }
    println!("setting is {:?}", setting_value);

    // + 使用以 _ 开头的名称, 忽略未使用的变量
    let _x = 5; // * 正常情况创建了未使用, 就会给警告, 表示未使用, 但是给了_, 可以忽略这个未使用的变量
    // let y = 10;

    let s = Some(String::from("Hello"));
    // * 这里其实已经将 s 的所有权移交给了 _s, 所以后面打印 s, 已经没有所有权了, 也就报错了
    // if let Some(_s) = s {
    // * 但是像这样使用_, 就不会发生绑定操作, 也就不会移交所有权, 后面就可以继续使用s
    if let Some(_) = s {
        println!("found a string");
    }
    println!("{:?}", s);

    // + ..(忽略值的剩余部分) 
    let nums = (2, 3, 4, 5, 6, 8, 10);
    match nums {
        // * 无论多少元素都可以使用..表示
        // * 但是不能 (first, .., third, .., last), 中间的元素不确定, 于是有歧义会报错
        (first, .., last) => {
            println!("num: {}, {}", first, last);
        }
    }
}

// * 使用 match 来提供额外的条件
pub fn match_guard() {
    // * match的守卫就是 match arm 模式后额外的 if 条件, 想要匹配该条件也必须满足
    // * match 守卫适用于比单独的模式更复杂的场景
    /* let num = Some(4);
    match num {
        // * 守卫
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("{}", x),
        None => ()
    } */

    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("got 50"),
        // * 这个守卫不是一个模式, 不会引入新的变量
        Some(n) if n == y => println!("Matched, n = {:?}", n),
        _ => println!("default case, x = {:?}", x)
    }

    println!("at the end: x = {:?}, y = {:?}", x, y);
}

// @ 绑定
pub fn bind() {
    // * @ 符号让我们创建一个变量, 该变量可以在测试某个值是否与模式匹配的同时保存该值
    enum Message {
        Hello { id: i32 },
    }
    let msg = Message::Hello { id: 5 };
    match msg {
        Message::Hello {
            // * 这个 @ 符号, 表示将匹配到的值, 存储到 id_variable 变量中
            id: id_variable @ 3 ..= 7,
        } => {
            println!("Found an id in range: {}", id_variable);
        },
        Message::Hello {
            id: 10 ..= 12
        } => {
            println!("Found an id in another range")
        },
        Message::Hello {id} => println!("Found Some other id: {}", id)
    }
}

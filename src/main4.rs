// TODO 结构体
fn main() {
    // * 定义结构体, 使用 struct &关键字
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }

    // * 使用结构体, 需要给每个字段指定具体值来创建结构体的 实例
    let mut user1 = User {
        email: String::from("fuck@qq.com"),
        username: String::from("fucker"),
        sign_in_count: 100,
        active: true,
    };
    // * rust的整个实例必须是可变的, rust并不允许只将某个字段标记为可变
    user1.email = String::from("fuck1@qq.com");

    // * 也可以在函数体的最后一个表达式中构造一个结构体的新实例, 来隐式的返回这个实例
    fn build_user(email: String, username: String) -> User {
        return User {
            username,
            email,
            sign_in_count: 1,
            active: true,
        };
    }

    let user2: User = build_user(String::from("cao@qq.com"), String::from("cao"));

    let user3: User = User {
        email: String::from("fuck@qq.com"),
        username: String::from("fucker"),
        ..user1 // 剩余参数接收, 和js不同的是, 不会覆盖, 只是将未显示设置的字段做对应的替换
    };
    println!("我的名字是: {}, 邮箱是: {}", user2.username, user3.email);

    // * 元组结构体
    struct Color(i32, i32, i32);
    let black = Color(0, 0, 0);
    println!("tuple of black: {}", black.0);

    // ? 方法, 有点类似于类和类的实例成员, &self就是this指针, 指向实例本身
    // * 和函数类似, 使用 fn 关键字和名称声明, 可以拥有参数和返回值, 同时包含在某处调用该方法时执行的代码。步古沟方法和函数是不同的, 因为他们在结构体的上下文中被定义(或者是枚举或trait对象的上下文), 并且第一个参数总是 self, 代表调用该方法的结构体实例。
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        // * 方法
        fn area(&self) -> u32 {
            return self.width * self.height;
        }

        fn can_hold(&self, other: &Rectangle) -> bool {
            return (self.width > other.width) && (self.height > other.height);
        }

        // * 关联函数 square
        fn square(size: u32) -> Rectangle {
            return Rectangle {
                width: size,
                height: size,
            };
        }
    }

    let rect1 = Rectangle {
        width: 30,
        height: 30,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 20,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };
    println!(
        "The area of the rectangle is {} square pixels.",
        // 这里其实有一个  自动引用和解引用, 等效于 (&rect1).area(), 所以之后可以继续打印
        rect1.area()
    );
    println!(
        "The area of the rectangle2 is {} square pixels., {}, {}",
        rect1.area(),
        rect1.can_hold(&rect2),
        rect1.can_hold(&rect3),
    );

    // ? 关联函数: 允许在 impl 块中定义 不 以 self 作为参数的函数。(有点类似于静态方法)
    // * 它们仍是函数而不是方法，因为它们并不作用于一个结构体的实例, 比如 String::from, 就是一个关联函数
    // * 使用结构体名和 :: 语法来调用这个关联函数：比如 let sq = Rectangle::square(3);。这个方法位于结构体的命名空间中：:: 语法用于关联函数和模块创建的命名空间
    let rect4 = Rectangle::square(30);
    println!("关联函数执行结果, rect4.width: {}", rect4.width);

    // * 一个结构体可以拥有多个 impl 块
}

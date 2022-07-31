/* 
    使用 Trait 对象来存储不同类型的值
    有这样一个需求:
    + 创建一个GUI工具:
        - 它会遍历某个元素的列表, 依次调用元素的draw方法进行绘制
        - 例如: Button、TextField等元素
    + 在面向对象语言里:
        - 定义一个 Component父类, 里面定义了draw方法
        - 定义 Button、TextField等类, 继承 Component 类
    + 但是在Rust中没有继承, 因此我们需要使用trait 来定义共有行为

    * 为共有行为定义trait
    + Rust避免将struct或enum 称为对象, 因为他们与impl块是分开的
    + trait有些类似于其他语言中的对象:
        - 它们某种程度上组合了数据与行为
    + trait对象与传统对象不同的地方:
        - 无法为trait对象添加数据
    + trait对象 实际上被专门用于抽象某些共有行为, 它没有其他语言中的对象那么通用

    * Trait 对象执行的是动态派发
    + 将 trait 约束作用于泛型时, Rust编译器会执行单态化:
        - 编译器会为我们用来替换泛型类型参数的每一个具体类型生成对应函数和方法的非泛型实现
    + 通过单态化生成的代码会执行静态派发(static dispatch), 在编译过程中确定调用的具体方法
    + 动态派发(dynamic dispatch):
        - 无法在编译过程中确定调用的究竟是哪一种方法
        - 编译器会产生额外的代码以便在运行时找出希望调用的方法
    + 使用trait对象, 会执行动态派发:
        - 产生运行时开销
        - 阻止编译器内联方法代码, 使得部分优化操作无法进行
    
    * Trait 对象必须保证对象安全
    + 只能把满足对象安全(object-safe)的trait转换为trait对象
    + Rust 采用一系列规则来判定某个对象是否安全, 只需记住两条:
        - 方法的返回类型不是Self
        - 方法中不包含任何泛型类型参数

    例见lib.rs
*/

use trait_obj::{Button, Screen};
use trait_obj::Draw;

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>
}

impl Draw for SelectBox {
    // * 在lib中说了, 只要这个SelectBox实现了 Draw这个trait, 就都可以放到Screen里面
    fn draw(&self) {
        // 绘制一个选择框
    }
}


fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("yes"),
                    String::from("Maybe"),
                    String::from("fuck"),
                ]
            }),
            Box::new(Button {
                width: 50,
                height: 20,
                label: String::from("mother fucker"),
            })
            // 能放到components里面的, 只有实现了 Draw这个 trait的才能放入
        ]
    };
    screen.run();
}

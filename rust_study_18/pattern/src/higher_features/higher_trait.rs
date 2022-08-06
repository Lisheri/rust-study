/*
    关联类型在 trait 定义中指定占位符类型
    关联类型: 一个将类型占位符与trait相关联的方式。

    泛型:
    1. 每次实现 Trait 时标注类型
    2. 可以为一个类型多次实现某个Trait(不同的泛型参数)

    关联类型:
    1. 无需标注类型(但是内部需要指明某个关联类型)
    2. 无法为单个类型多次实现某个trait

    默认泛型参数和运算符重载
    1. 可以在使用泛型参数时为泛型指定一个默认的具体类型
    2. 语法 <PlaceholderType=ConcreteType> (<占位类型=具体类型>, 右边的就是默认类型)
    3. Rust不允许创建自己的运算符以及重载任意的运算符
    4.但是可以通过实现 std::ops 中列出的哪些 trait 来重载一部分相应的运算符

    默认泛型参数的主要应用场景:

    1. 扩展一个类型而不破坏现有代码
    2. 允许在大部分用户都不需要的特定场景下进行自定义

    完全限定语法
    调用同名方法
    <Type as Trait>::function(receiver_if_method, next_arg, ...);
    - 可以在任何调用函数或方法的地方使用
    - 允许忽略哪些从其他上下文推导出来的部分
    - 当Rust无法区分期望调用哪个具体实现的时候才需使用这种语法

    使用supertrait 来要求 trait 附带其他 trait 的功能
    需要在一个trait中使用其他trait的功能
        需要被依赖的 trait 也被实现
        哪个被间接依赖的 trait 就是当前 trait 的 supertrait

    实现的 trait: OutlinePrint
    依赖的 trait: fmt::Display
    trait OutlinePrint: fmt::Display {}

    所以实现了 OutlinePrint 这个trait的struct必须还要实现 fmt::Display

    使用 newtype 模式在外部类型上实现外部 trait
    + 孤儿规则: 只有当 trait 或类型定义在本地包时, 才能为该类型实现这个trait
    + 可以通过 newtype 模式来绕过这一规则
        - 利用 tuple struct(元组结构体) 创建一个新的类型
*/

pub trait Iterator {
    // * 关联类型Item, 也就是所谓的类型占位符
    type Item;
    
    fn next(&mut self) -> Option<Self::Item>;
}

pub trait Iterator2<T> {
    fn next(&mut self) -> Option<T>;
}

struct Counter {}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

// * 第一次实现
impl Iterator2<String> for Counter {
    fn next(&mut self) -> Option<String> {
        None
    }
}

// * 第二次实现
impl Iterator2<u32> for Counter {
    fn next(&mut self) -> Option<u32> {
        None
    }
}

// 重载
use std::ops::Add;

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32
}

impl Add for Point {
    type Output = Point;

    // * 重载了 加号这个运算符
    fn add(self, other: Point) -> Point {
        return Point {
            x: self.x + other.x,
            y: self.y + other.y
        };
    }
}
// * 运算符重载运用
fn use_add() {
    assert_eq!(Point { x: 1, y: 2 } + Point { x: 2, y: 3 }, Point { x: 3, y: 5 });
}

// * 毫米和米相加

struct Millimeters(u32);
struct Meters(u32);

// * 显示设定Add后的泛型参数为米
impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    // * self代表就是自己, 也就是Millimeters(毫米)
    fn add(self, other: Meters) -> Millimeters {
        return Millimeters(self.0 + (other.0 * 1000));
    }
}

// * 完全限定语法

trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up.");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

// * 上述三个fly函数的签名是一样的, 需要指定Trait来调用对应的fly函数
pub fn use_human() {
    let person = Human;
    // * 这个fly为Human默认的fly函数
    person.fly();
    // * Pilot的fly调用（这个入参 &person是Human的实例, 因此他这里可以识别到是Human的实现）
    Pilot::fly(&person);
    Wizard::fly(&person);

    // * 下面的函数实现没有参数
    println!("a baby dog is called a : {}", Dog::baby_name());
    // * 下面的函数由于没有参数, 因此无法确定是哪个struct实现的当前trait
    // * 所以需要使用完全限定语法 <Type as Trait>::function(receiver_if_method, next_arg, ...);
    println!("an animal dog is called a: {}", <Dog as Animal>::baby_name());

    // 使用 supertrait 来要求 trait 附带其它trait的功能（就是一个Trait 继承于 其它Trait）
}

use std::fmt;

// * 依赖的Trait就是 fmt::Display
trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        // * 要求 OutlinePrint 在实现时具有to_string方法, 也就是他的类型必须实现Display
        let output = self.to_string();
        let len = output.len();

        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

struct Point2 {
    x: i32,
    y: i32
}

impl OutlinePrint for Point2 {}

impl fmt::Display for Point2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "({}, {})", self.x, self.y);
    }
}

pub fn use_out_line_print() {
    let point = Point2 {x: 12, y: 13};
    point.outline_print();
}

// * 使用newtype绕过孤儿规则
// ! 其实类似于高阶组件, 并没有修改内部内容, 而是在外部扩展
// 为 Vector 实现 Display这个 trait

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "[{}]", self.0.join(", "));
    }
}

pub fn use_wrapper() {
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
}


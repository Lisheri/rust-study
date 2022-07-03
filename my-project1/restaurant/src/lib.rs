// * 一个包含了其它内置了函数的模块的 front_of_house 模块
/* mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}
        fn server_order() {}
        fn take_payment() {}
    }
} */

// * 使用use关键字引入模块, 如下所示, 相当于在最外层引入了 hosting 模块, 调用将简化为从 hosting开始
// ? 在作用域中增加 use 和路径类似于在文件系统中创建软连接
// use crate::front_of_house::hosting;
// * 也可以使用一个相对路径来引入模块, 如下所示(或者: use front_of_house::hosting;)
// use self::front_of_house::hosting;

/*
  ? 在功能上可以使用 use 直接引入一个函数, 但是这样的表达不够明确, 也不符合习惯
  习惯上, 我们是将爹模块引入作用域
  因为这样可以清晰地表明函数不是在本地定义的，同时使完整路径的重复度最小化
  另一方面，使用 use 引入结构体、枚举和其他项时，习惯是指定它们的完整路径。
  这种习惯用法背后没有什么硬性要求：它只是一种惯例，人们已经习惯了以这种方式阅读和编写 Rust 代码。
*/

/*
  有时候在使用use引入时, 两个不同的模块可能会有相同的成员, 这样会造成误解, 因此可以使用 as 关键字重命名
  比如
  use std::fmt;
  use std::io;

  * 像这样是可以区分的, 但如果直接引入 Result, 则会让 Rust产生困扰, 难以区分
  fn function1() -> fmt::Result {
      // --snip--
  }

  fn function2() -> io::Result<()> {
      // --snip--
  }

  * 引入时
  use std::fmt::Result;
  * 使用 as 重命名
  use std::io::Result as IoResult;
  fn function1() -> Result {
      // --snip--
  }

  fn function2() -> IoResult<()> {
      // --snip--
  }

  在第二个 use 语句中，我们选择 IoResult 作为 std::io::Result 的新名称，它与从 std::fmt 引入作用域的 Result 并不冲突。
*/

/* pub fn eat_at_restaurant() {
    // * 绝对路径
    // crate::front_of_house::hosting::add_to_waitlist();

    // * 相对路径
    // front_of_house::hosting::add_to_waitlist();

    // * 经过 use 关键字引入 hosting后
    hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("Rye");

    meal.toast = String::from("Wheat");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
    println!(
        "I' d like {} toast please, {:?}, {:?}",
        meal.toast, order1, order2
    );
}

fn server_order() {}

mod back_of_house {
    // * 使用 pub 创建公有的结构体和枚举, 不过在一个结构体定义的前面使用了 pub, 这个结构体会变成公有的, 但是这个结构体的字段仍然是私有的。可以根据情况决定每个字段是否公有。
    pub struct Breakfast {
        pub toast: String,      // * 公有字段
        seasonal_fruit: String, // * 私有字段(rust 默认私有)
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            return Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            };
        }
    }

    // * 枚举如果是公有, 那么它的所有成员都将变成公有（因为枚举成员默认是公有的）
    #[derive(Debug)]
    pub enum Appetizer {
        Soup,
        Salad,
    }

    fn fix_incorrect_order() {
        cook_order();
        // * 使用 super 开头来构建从父模块开始的相对路径
        // * super 可以进入 模块 back_of_house 的父模块, 也就可以调用 server_order 了
        super::server_order();
    }

    fn cook_order() {}
} */

// * 当使用 use 关键字将名称导入作用域时, 在新作用域中可用的名称是私有的。如果为了让调用你编写的代码的代码能够像在自己的作用域内引用这些类型, 可以结合 pub 和 use。
// * 这个技术被称为: "重导出(re-exporting)", 因为这样做将项引入作用域并同时使其可供其他代码引入自己的作用域
// * 这样做可以让一个 index.rs 文件作为一个公共的导出文件存在, 一个库下面的所有模块, 都走同一个出口文件出去, 引入时, 也通过这一个文件引入即可
// * 使用 pub use，我们可以使用一种结构编写代码，却将不同的结构形式暴露出来。这样做使我们的库井井有条，方便开发这个库的程序员和调用这个库的程序员之间组织起来。

// ? 嵌套路径

/*
   * 不使用嵌套路径时
    use std::cmp::Ordering;
    use std::io;

  ? 使用嵌套路径, 显著减少use的独立语句数
    use std::{comp::Ordering, io};

  ? 嵌套路径引入自己
    use std::io;
    use std::io::Write;

  ? 嵌套简化, 这个self代表的就是io本身
    use std::{self, Write};
*/

// ? 通过 glob 运算符将所有的公有定义引入作用域
// * 系统将一个路径下所有公有项引入作用域, 可以指定路径后跟 * (glob运算符):
// * use std::collections::*;


// ? 这里使用分号, 而不是代码块, 是告诉 Rust 在另一个与模块同名的文件中加载模块的内容
// * 这个modules, 加载是 src/modules.ts, 而 modules.rs内部的 mod front_of_house, 对应的就是 src/modules/front_of_house.rs
// ! 这样做的原因主要在于, 文件夹本身不能成为一个模块
mod modules;

pub use crate::modules::front_of_house::hosting;
pub use crate::modules::collections::use_vector;
pub fn eat_at_restaurant() {
  hosting::add_to_waitlist();
  // use_vector::createVector();
  // use_vector::useEnumSaveValue();
  // use_vector::create_string();
  use_vector::create_hash_map();
}

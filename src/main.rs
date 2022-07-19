/*
    Rust 有许多功能可以让你管理代码的组织，包括哪些内容可以被公开，哪些内容作为私有部分，以及程序每个作用域中的名字。这些功能。
    这有时被称为 “模块系统（the module system）”，包括：

    + 包（Packages）： Cargo 的一个功能，它允许你构建、测试和分享 crate。
    + Crates ：一个模块的树形结构，它形成了库或二进制项目。
    + 模块（Modules）和 use： 允许你控制作用域和路径的私有性。
    + 路径（path）：一个命名例如结构体、函数或模块等项的方式
*/
fn main() {
  /* 
    ! 1. create
    create 是一个二进制项或者库。 create root 是一个源文件, Rust编译器以它为起始点, 并构成开发者的 crate 根模块
    包(package)是提供一系列功能的一个或者多个 crate。 一个包会包含有一个 Cargo.toml文件, 阐述如何去构建这些crate。

    文中所包含的内容由几条规则来确立。一个包中最多 只能 包含一个 库crate(library crate); 包中可以包含任意多个 二进制crate(binary crate); 包中至少包含一个 crate, 无论是库还是二进制的。
  */

  // 创建包的命令: cargo new my-project
  /* 
    这个命令输入完成后, 就会创建一个新的包, 名为my-project, 内部会创建一个Cargo.toml文件。 
    在Cargo.toml文件中, 没有提到src/main.rs
    因为Cargo遵循一个约定: src/main.rs就是一个与包同名的二进制crate的crate根。同样的, Cargo知道如果包目录中包含 src/lib.rs, 则包带有与其同名的库crate, 且 src/lib.rs 是 crate根。
    crate根文件将由Cargo传递给 rustc 来实际构建库或者二进制项目。

    在此, 我们有了一个只包含 src/main.rs 的包, 意味着它只含有一个名为 my-project 的二进制crate。如果一个包同时含有src/main.rs 和 src/bin.rs, 则它有两个crate: 一个库和一个二进制项, 且名字都与包相同。
    如果将文件放在src/bin目录下, 一个包可以拥有多个二进制crate: 每个 src/bin 下的文件都会被编译成一个独立的二进制 crate

  */

  /* 
    一个 crate 会将一个作用域内的相关功能分组到一起，使得该功能可以很方便地在多个项目之间共享。
  */

  /* 
    ! 2. 模块系统: 定义模块来控制作用域与私有性

    主要包含:
    ? 路径: paths(项所有)
    ? use关键字: 用于将路径引入作用域
    ? pub: 使项变为公有的(public)

    * 模块: 让开发者可以将一个 crate 中的代码进行分组, 提高可读性和重用性。模块还可以控制项的 私有性
    * 即 项 是可以被外部代码使用的(public), 还是作为一个内部实现的内容, 不能被外部代码使用(private)

    * 定义模块, 以 mod 关键字为起始, 然后指定模块的名字, 比如 restaurant/src/lib.rs 中的 mod front_of_house {}, 这个模块的名字就叫做: front_of_house
    * 在模块内, 还可以以 mod 关键字来定义其他的模块, 比如内部的 hosting 和 serving 模块。 模块还可以保存一些定义的其他项, 比如结构体、枚举、常量、特性或者函数

    ? 前面说过, src/main.rs 和 src/lib.rs 叫做 crate 根, 之所以这样叫他们, 是因为这两个文件的 内容都分别在 crate 模块结构的根组成了一个名为 crate 的模块, 该结构被称为 模块树(module tree)
  */

  /* 
    ! 3. 路径: 用于引用模块树中的项

    路径有两种形式:
      * 绝对路径(absolute path): 从 crate 根开始, 以 crate名或者字面值 crate 开头
      * 相对路径(relative path): 从当前模块开始, 以 self、super或当前模块的标识符开头

    绝对路径和相对路径都后跟一个或多个由双冒号(::) 分割的标识符

    回到上面定义的front_of_house 项模块

    在模块外部重新定义一个公有API, 用于调用 front_of_house下的模块成员

    pub fn eat_at_restaurant() {
      * 绝对路径
      crate::front_of_house::hosting::add_to_waitlist();

      * 相对路径
      front_of_house::hosting::add_to_waitlist();
    }

    当然, 上面的例子无法通过编译, 后面再说

    由于add_to_waitlist 函数与 eat_at_restaurant 被定义在同一个 crate 中, 因此, 开发者可以使用 crate 关键字为起始的绝对路径调用函数

    而add_to_waitlist 函数被定义在了模块 front_of_house 下面, 而 front_of_house 与 eat_at_restaurant同一层级, 因此, 相对路径以 front_of_house开头, 向下寻找 add_to_waitlist

    使用绝对路径还是相对路径还是看开发人员是否会移动模块或者函数, 一般来说更倾向于 绝对路径, 因为把代码定义和项的调用各自独立地移动是非常常见的。
  */

  /* 
    如果此时编译 restaurant, 此时会报错, 因为 hosting 模块是私有的, 不能访问私有片段

    ? 模块不仅对组织代码很有用, 而且还定义了 Rust 的 私有性边界(privacy boundary): 这条界线不允许外部代码了解、调用和依赖被封装的实现细节。所以, 如果希望创建一个私有函数或结构体, 可以将其放入模块

    Rust 中默认所有项（函数、方法、结构体、枚举、模块和常量）都是私有的。
    父模块中的项不能使用子模块中的私有项，但是子模块中的项可以使用他们父模块中的项。这是因为子模块封装并隐藏了他们的实现详情，但是子模块可以看到他们定义的上下文。

    当然, 可以通过使用 pub 关键字来创建公共项，使子模块的内部部分暴露给上级模块。

    将 hosting 暴露出来, 即可在外部被调用: pub mod hosting {...}
  */
  println!("Hello, world!");
}

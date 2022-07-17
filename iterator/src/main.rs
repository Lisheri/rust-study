/*
  * 什么是迭代器
  + 迭代器模式: 对一系列项执行某些任务
  + 迭代器负责:
    - 遍历每个项
    - 确定序列(遍历)何时完成
  + Rust的迭代器
    - 懒惰的: 除非调用消费迭代器的方法, 否则迭代器本身没有任何效果(就是说迭代器不用的时候, 就是啥也不干, 开始消耗迭代器方法, 才会起到迭代器的作用)
*/

fn main() {
    let v1 = vec![1, 2, 3];
    // * 调用iter方法, 就可以产生一个迭代器
    // * 这个迭代器就可以用于遍历vector
    let v1_iter = v1.iter();

    // * 使用迭代器
    // ? 这里不需要声明mut变量, 是因为 for循环取得了 v1_iter的所有权, 并且在内部已经将其变成了可变
    for val in v1_iter {
        println!("Got: {}", val);
    }

    // * 迭代器(1)
    // - iterator trait 和 next 方法
    /*
      + 所有的迭代器都实现了 iterator trait
      + iterator trait 定义于标准库, 定义大致如下:
      + pub trait iterator {
        type item;
        这里有一个next方法, 返回的是 Option枚举
        fn next(&mut self) -> Option<Self::item>;
      }

      + type item 和 Self::item 定义了与此这个 trait 关联的类型
        - 实现 iterator trait 需要定义一个 item 类型, 它用于 next方法的返回类型(迭代器的返回类型)。

      + iterator trait
        - iterator trait 仅要求实现一个方法: next
        - next:
          - 每次返回迭代器中的一项
          - 返回结果包裹在 Some里
          - 迭代结束, 返回None
        - 可直接在迭代器上调用 next 方法(例见lib.rs)
        ? 每一次调用next就是消耗了迭代器中的一个元素, 返回当前元素并将指针移动到下一位
          - iter 方法生成的是一个不可变引用的迭代器, 是在不可变引用上创建迭代器
          - 通过调用next方法所取得的值, 实际上是指向vector中的元素的不可变引用, 当然, 它是指元素是不可变引用, 而非迭代器本身, 迭代器本身是一个可变的引用, 他就是通过不可变引用来迭代元素的
        - into_iter 方法: 会将元素移动到新的作用域内, 创建迭代器并获得其所有权
        - iter_mut 方法: 迭代可变的引用
    * 消耗/产生迭代器
        + 消耗迭代器的方法
          - 在标准库中, iterator trait 有一些带默认实现的方法
          - 其中有一些方法会调用 next 方法
            ? 这也是为什么要实现 iterator trait 时必须实现 next 方法的原因之一, 否则这些默认实现的方法就无法按使用
          - 调用 next 的方法就叫做 "消耗性适配器", 因为这些方法会将迭代器中的元素一个一个吃掉
            * 因为调用他们会把迭代器消耗尽
          - 例如: sum 方法(就会耗尽迭代器)
            * 该方法会取得迭代器的所有权
            * sum就是通过反复调用 next, 遍历所有元素
            * 每次迭代, 把当前元素添加到一个总和里, 迭代结束, 返回总和(例 见lib.rs)
          - collect 方法: 将结果收集到一个集合类型中
        + 产生迭代器的方法
          - 定义在 iterator trait 上的另外一些方法叫做 "迭代器适配器"
            * 将迭代器转换为不同种类的迭代器
          - 可以通过链式调用使用多个迭代器适配器来执行复杂的操作, 这种调用可读性较高
          - 如: map
            * 接受一个闭包, 闭包作用于每个元素, 将每个元素转换为另一个元素
            * 产生一个新的迭代器(例 见lib.rs)
    * 使用闭包捕获环境（配合迭代器）
      + filter 方法, 属于是一种迭代器适配器, 通过它来演示闭包捕获环境的一种常见做法
        - 接收一个闭包
        - 闭包在遍历迭代器每个元素时, 返回一个 bool 类型
        - 如果闭包返回 true: 当前元素将会包含在 filter产生的迭代器中
        - 如果闭包返回false: 新的迭代器将不会包含当前元素
        - 例 见 lib.rs
    * 创建自定义迭代器
      + 使用 iterator trait 来创建自定义迭代器
        - 实现 next方法
    */
}

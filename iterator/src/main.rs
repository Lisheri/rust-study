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
        - 可直接在迭代器上调用 next 方法
    */
}

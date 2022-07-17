use std::fmt::Display;
// ! 结构体(struct)定义中的生命周期标注
/*
  struct 里可包括:
  - 自持有的类型
  - 引用: 需要在每个引用上添加生命周期标注
*/
// * 这里有一个struct, 里面只有一个字段
struct ImportantExcerpt<'a> {
    // part的类型是引用类型, 他是引用类型, 所以需要标注生命周期
    // 标注完成后意味着, part这个字段的生命周期, 必须要比结构体 ImportantExcerpt 存活的时间要长才行
    // * 因为只要 ImportantExcerpt 他的实例存在, 他就会对 part 产生一个引用, 如果part先没有, 那就产生空指针出错了
    part: &'a str,
}
fn main() {
    let novel = String::from("call me Ishmael. Some years ago...");

    // * split返回分割的迭代器(成员为引用), next 调用一次返回迭代器中的一个项, 最后一个项为 None枚举
    let first_sentence = novel.split('.').next().expect("Could not found a '.'");

    // * 这样写是有效的, 因为 first_sentence这个引用, 从18行开始到结尾都是有效的, 而实例的生命周期从 21 行开始
    let i = ImportantExcerpt {
        part: first_sentence,
    };

    // * 生命周期的省略
    /*
      * 我们知道:
        每个引用都有生命周期
        需要为使用生命周期的函数或struct指定生命周期参数

      之前写过这样一个函数, 没有任何生命周期标识, 但是依然可以通过编译
      * 其实在 Rust1.0版本, 下面的代码是无法编译通过的, 因为当时要求的是, 每个引用都必须有一个显示生命周期
      * 但是在编写了很多代码后, rust 团队发现在某些情况下, rust 程序员总是在一遍又一遍的编写相同的生命周期标注
      * 而且这些场景都是可预测的, 有一些明确的模式, 于是 rust 团队就将这样一些模式写入了编译器代码, 在一些情况下可以自动推导, 而无需显式标注
      * 这样来说, 再未来可能会有越来越多的模式被加入编译器代码, 而不需要显示标注生命周期
      fn first_word<'a>(s: &'a str) -> &'a str {
        let bytes = s.as_bytes();

        for(i, &item) in bytes.iter().enumerate() {
          if item === b'' {
            return &s[0..i];
          }
        }
        return &s[..];
      }

      ? 上面说的在 Rust 引用分析中所编入的模式, 就被称为: 生命周期省略规则
        - 这些规则无需开发者来遵守
        - 他们是一些特殊情况, 由编译器来考虑
        - 如果代码符合上述情况, 那么久无需显式标注生命周期
      ? 生命周期省略规则, 不会提供完整的推断:
        - 如果应用规则后, 引用的生命周期仍然模糊不清 -> 将直接编译错误
        - 解决办法: 添加生命周期标注, 表明引用间的相互关系

      ! 输入、输出生命周期

      生命周期位置:
        - 函数/方法的参数: 叫做输入生命周期
        - 函数/方法的返回值: 叫做输出生命周期


      生命周期省略的三个规则

      + 编译器使用三个规则在没有显式标注生命周期的情况下, 来确定引用的生命周期
        - 规则1应用于输入生命周期
        - 规则2、3引用于输出生命周期
        - 如果编译器应用完3个规则后, 依然无法确定生命周期的引用 -> 直接报错
        - 这些规则适用于fn定义 以及 impl块
      + 规则1: 每个引用类型的参数都有自己的生命周期(主要是数量匹配, 单参数函数拥有一个生命周期参数, 双参数函数拥有两个不同的生命周期参数, 以此类推)
      + 规则2: 如果只有 1 个输入生命周期参数, 那么该生命周期参数被赋予给所有的输出生命周期参数
      + 规则3: 如果有多个输入生命周期参数, 但其中一个是 &self 或 &mut self(是方法), 那么self的生命周期会被赋给所有的输出生命周期参数

      例子:

      + 假设我们是编译器:
      1.
        + fn first_word(s: &str) -> &str {
        + 规则1应用: fn first_word<'a>(s: &'a str) -> &str {
          - 规则1的效果就是给每个参数都带上生命周期
        + 由于只有一个参数, 那么规则2也适用: fn first_word<'a>(s: &'a str) -> &'a str {
        规则2应用完之后, 所有参数已带上生命周期, 那么也就不需要开发者主动标注生命周期了
      2.
        fn longest(x: &str, y: &str) -> &str {}
        fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &str {}
        由于有两个输入生命周期, 因此规则2就不适合, 并且longest是个函数, 不是方法, 所以规则3也不合适
        到此为止, 三个规则都已经使用结束, 依然无法获得返回值的生命周期, 此时编译器就会报错
    */

    // * 看个例子, 同时使用trait bound 和生命周期
    fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where
        T: Display,
    {
      println!("Announcement! {}", ann);
      if x.len() > y.len() {
        return x;
      } else {
        return y;
      }
    }
}

/*
  ! 方法定义中的生命周期标注
  上面说过, 规则3: 如果有多个输入生命周期参数, 但其中一个是 &self 或 &mut self(是方法), 那么self的生命周期会被赋给所有的输出生命周期参数
  这个规则, 仅适用于方法

  方法仅适用于 struct
  + 在struct 上使用生命周期实现方法, 语法和泛型参数的语法一样
  + 在哪声明和使用生命周期参数, 依赖于:
    - 生命周期参数是否和字段、方法的参数和返回值有关
  + struct字段的生命周期名:
    - 在 impl后声明
    - 在struct名后使用
    - 这些生命周期是struct类型的一部分
  + impl块内的方法签名中:
    - 引用必须绑定于struct 字段引用的生命周期, 或者引用是独立的也行
    - 生命周期省略规则经常使得方法中的生命周期标注不是必须的
  如下所示:
*/
struct ImportantExcerpt<'a> {
    part: &'a str,
}

// * 声明                  应用
impl<'a> ImportantExcerpt<'a> {
    // * 这里声明了一个 level的方法, 他的参数是&self, 返回值是i32, 不会引用任何东西
    // * impl后和结构体后的生命周期是不能省略的, 但是根据规则1, 不需要为 &self标注生命周期
    fn level(&self) -> i32 {
        return 3;
    }

    // * 再添加一个方法
    // * 根据规则1, 会为方法的两个参数添加各自的生命周期, 然后由于其中一个参数是 &self, 根据规则3, 会给返回值添加 &self的生命周期, 因此所有的生命周期已经计算出来了, 也就编译通过了
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        return self.part;
    }
}

/*
  ! 静态生命周期
  + 'static' 是一个特殊的生命周期: 表示整个程序的持续时间(或执行期)
    - 例如: 所有的字符串字面值都拥有 'static 生命周期
      let s: &'static str = "I have a static lifetime.";
    这个 字符串字面值, 他是直接存储在二进制程序中, 所以他总是可用的, 因此他的生命周期总是 'static
    有时候可能会在报错信息中看到 使用 ’static的建议
  + 为引用指定 'static 之前要三思:
    - 是否需要引用在整个生命周期内都要存活(毕竟大部分报错都是开发者在尝试创建一个悬垂引用, 或者是可用生命周期不匹配, 此时应该是尝试解决问题, 而非搞一个 'static )
*/

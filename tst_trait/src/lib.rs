// * 例: 此处有多个存放了不同类型和属性文本的结构体: 结构体 NewsArticle 用于存放发生于世界各地的新闻故事, 而结构体 Tweet 最多只能存放 280个字符的内容, 以及像是否转推或者是否对推友的回复这样的元数据
// 创建一个名为 aggregator 的多媒体聚合库用来显示可能存储在 NewsArticle 或 Tweet实例中的数据的总结。每一个结构体都需要的行为是他们是能够被总结的, 这样的话就可以调用示例的 summarize 方法来请求总结。

// Summary trait的声明
pub trait Summary {
  // * 和 interface 类似, 这里只是提供函数(方法)的定义, 而没有他的实现
  // fn summarize(&self) -> String;
  fn summarize(&self) -> String {
    return String::from("read more...");
  }

  // * 默认实现很自由, 也可以在默认实现中调用其他实现
}

pub trait Display {
  fn fuck();
}

// * 为类型实现 trait

pub struct NewsArticle {
  pub headline: String,
  pub location: String,
  pub author: String,
  pub content: String,
}

impl Summary for NewsArticle {
  fn summarize(&self) -> String {
    return format!("{}, by {} ({})", self.headline, self.author, self.location);
  }
}

pub struct Tweet {
  pub username: String,
  pub content: String,
  pub reply: bool,
  pub retweet: bool,
}

impl Summary for Tweet {
  fn summarize(&self) -> String {
    return format!("{}: {}", self.username, self.content);
  }
}

// * 在类型上实现 trait 类似于与 trait 无关的方法。 区别在于 impl 关键字之后, 需要提供实现 trait 的名称, 接着是 for 和需要实现 trait 的类型的名称。
// * 在 impl 块中, 使用 trait 定义中的方法签名, 不过不再后跟分号, 而是需要在大括号中编写函数体来为特定类型实现 trait 方法所拥有的的行为

// ! 默认实现
/* 
  有时为 trait 中的某些或全部方法提供默认的行为, 而不是在每个类型的每个实现中都定义自己的行为是很有用的。这样当为某个特定类型实现 trait 时, 可以选择保留或重载每个方法的默认行为。
  也就是在定义时, 留下一个默认的函数体, 假如在实现 trait 时, 没有实现该方法, 则该方法走默认实现, 若实现该方法则重载
*/

// ! trait 作为参数
// * 知道了如何定义 trait 和在类型上实现这些 trait 之后，我们可以探索一下如何使用 trait 来接受多种不同类型的参数。
// * 其实 trait 和ts go之类语言的 interface 极其类似, 完全可以作为参数类型使用, 如下
pub fn notify(item: &impl Summary) {
  println!("breaking news! {}", item.summarize());
}

// ! trait Bound语法
// * 其实就是用trait约束泛型, 泛型必须满足trait(interface)的条件, 然后不需要在多个参数上写trait约束, 如下
pub fn notify2<T: Summary>(item: &T) {
  println!("breaking news! {}",  item.summarize());
}

// ! 通过 + 指定多个 trait
// pub fn notify3(item: &(impl Summary + Display)) 或者 pub fn notify<T: Summary + Display>(item: &T) {}

// ! where从句简化(收归) trait bound
// * fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {} 如下收归, 效果是一样的
/* 
  pub fn some_function<T, U>(t: &T, u: &U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
  {
    ...
  } 
*/

// ! 实现 trait作为返回类型
// 也可以在返回值中使用 impl Trait 语法，来返回实现了某个 trait 的类型
/* 
  fn returns_summarizable() -> impl Summary {
      return Tweet {
          username: String::from("horse_ebooks"),
          content: String::from(
              "of course, as you probably already know, people",
          ),
          reply: false,
          retweet: false,
      };
  }
*/

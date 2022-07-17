/*
  ! 泛型
  * 每一个编程语言都有高效处理重复概念的工具。Rust中工具之一就是: 泛型
  泛型: 指具体类型或其他属性的抽象替代

  ! trait
  定义泛型行为的方法。
  trait可以与泛型结合来将泛型限制为拥有特定行为的类型, 而不是任意类型

  ! 生命周期
  它是一类允许开发人员向编译器提供引用如何相互关联的泛型。Rust的生命周期功能允许在很多场景下借用值的同时仍然使编译器能够检查这些引用的有效性。

  ! 不使用泛型的情况
  在不使用泛型语法之前, 一般使用提取公有函数来处理重复逻辑。之后, 开发人员可以使用相同的机制来提取一个泛型函数
*/
use std::fmt::Display;
use tst_trait::{Summary, Tweet};
fn get_largest(list: &[i32]) -> i32 {
    let mut largest = list[0];
    for &number in list {
        if number > largest {
            largest = number;
        }
    }
    return largest;
}
fn main() {
    println!("Hello, world!");
    // * 提取函数减少重复
    let num_list = vec![10, 20, 5, 100, 1001, 1002];
    // * 使用函数抽象
    println!("{}", get_largest(&num_list));
    // * 也可以使用泛型抽象。与函数体可以在抽象list而不是特定值上操作的方式相同, 泛型允许代码对抽象类型进行操作
    // ? 比如一个需要寻找一个 i32 值的slice中的最大项, 而另一个寻找 char 值的 slice中的最大项, 如果不使用泛型操作, 那么就会出现由于入参类型不同而需要定义两个函数的需求了
    // Rust 类型名的命名规范是骆驼命名法(CamelCase)
    /* fn largest_f<T: std::cmp::PartialOrd>(list: &[T]) -> T {
      let mut largest = list[0];
      for &item in list {
        if item > largest {
          largest = item;
        }
      }
      return largest;
    }
    let list1 = vec!([1, 2, 3, 4, 100]);
    let list2 = vec!(['y', 'm', 'q']);
    println!("i32: {}", largest_f::<i32>(&list1));
    println!("i32: {}", largest_f::<char>(&list2)); */

    // ? 结构体中的泛型
    struct Point<T> {
        x: T,
        y: T,
    }

    let integer = Point::<i32> { x: 5, y: 10 };
    let float = Point::<f64> { x: 1.0, y: 4.0 };

    // ? 枚举中的泛型
    // 最典型的枚举泛型, 就是Option枚举 和 Result<T, E>枚举
    /*
      enum Option<T> {
        Some<T>,
        None,
      }
      enum Result<T, E> {
        Ok<T>,
        Err<E>,
      }
    */

    // ? 方法中的泛型
    impl<T> Point<T> {
        fn x(&self) -> &T {
            return &self.x;
        }
    }

    // * 方法定义还可以选择用于某些限制(constraint) 的泛型类型, 比如方法是为了 Point<f32>实现实例方法, 而不是为了泛型 Point实例
    impl Point<f32> {
        fn distance_from_origin(&self) -> f32 {
            return (self.x.powi(2) + self.y.powi(2)).sqrt();
        }
    }

    // * 上述代码意味着 Point<f32> 类型会有一个方法 distance_from_origin, 而其他 T 不是f32类型的 Point<T> 实例则没有定义此方法。这个方法计算点实例与坐标(0.0, 0.0)之间的距离, 并使用了只能用于浮点型的数学运算符。
    println!("{}", Point { x: 5, y: 10 }.x());

    let p1 = Point2 { x: 10, y: 10.4 };
    let p2 = Point2 {
        x: "hello",
        y: "world",
    };

    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    // * 泛型性能
    /*
      Rust 实现了泛型，使得使用泛型类型参数的代码相比使用具体类型并没有任何速度上的损失。
      Rust 通过在编译时进行泛型代码的 单态化（monomorphization）来保证效率。单态化是一个通过填充编译时使用的具体类型，将通用代码转换为特定代码的过程。
      编译器的工作就是寻找所有泛型代码被调用的位置并使用泛型代码针对具体类型生成代码。

      比如说
      let integer = Some(5);
      let float = Some(5.0);
      编译的时候会单太化展开, 展开为两种Option, 一个是 enum Option_i32 { Some: i32, None }, 一个是 enum Option_f64 { Some(f64), None } let integer = Option_i32::Some(5); let float = Option_f64::Some(5.0);
    */

    // ! trait: 定义共享的行为
    // trait 告诉 Rust编译器某个特定类型拥有可能与其他类型共享的功能。可以通过 trait 以一种抽象的方式定义共享的行为。可以使用 trait bounds 指定泛型是任何拥有特定行为的类型。
    // ? trait 类似于其他语言中常被称为 接口(interfaces)的功能, 虽然有一些不同

    // * 定义 trait（在lib.rs中）
    // 一个类型的行为由其可供调用的方法构成。如果可以对不同类型调用相同的方法的话，这些类型就可以共享相同的行为了。

    // * 使用
    let tweet = Tweet {
        username: String::from("house_ebooks"),
        content: String::from("of course, you stupid child!"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    // 其他依赖 tst_trait crate的 crate也可以将 Summary 引入作用域以便为其自己的类型实现该trait
    // ! 注: 只有当至少一个 trait 或者要实现 trait 的类型位于 crate 的本地作用域时, 才能为该类型实现 trait。例如, 可以为 tst_trait crate的自定义类型 Tweet实现如标准库中的 Display trait, 这是因为 Tweet 类型位于 tst_trait crate 本地的作用域中。
    // * 不能为外部类型实现外部 trait。这种限制叫做 相干性, 或者更具体的说是: 孤儿规则, 其得名在于不存在父类型。这条规则确保了其他人编写的代码不会破坏你的代码, 反之亦然。
    // * 没有这条规则存在, 可以两个 crate 分别对相同类型实现相同的trait, 而 Rust将无从得知应该使用哪一个实现。

    // ! 使用 trait bounds 修复 largest 函数
    // * 标准库中包含 std::cmp::PartialOrd, 这是一个默认的trait, 这种trait实现了支持 大于运算符(>), 比较两个T类型的值。同时 PartialOrd 位于 prelude, 不需要手动引入作用域。
    // * 这里必须将Copy的类型也带上, 对于非泛型版本的 largest 函数, 只是尝试寻找最大的i32和char。像 i32 和 char这样已知类型和大小的数据可以存储在栈上, 可以他们实现了 Copy trait。
    // * 但是当我们将 largest 改造为泛型函数后, 现在 list 参数的类型就有可能是没有实现 Copy trait 的, 意味着可能不能将list[0]的值移动到 largest变量中, 就会导致错误: cannot move out of type [T], a non-copy slice
    // * 所以这里的核心是一定要显示声明泛型变量是包含 Copy的, 而不是单纯的PartialOrd
    // ? 如果并不希望限制 largest 函数只能用于实现了 Copy trait的类型, 可以在T的 trait bounds中指定 Clone 而不是 Copy。并克隆slice的每一个值使得largest函数有用其所有权。使用 clone 函数意味着对于类似 String 这样拥有堆删该数据的类型, 会潜在分配更多的堆内存
    // ? 而堆内存的分配在涉及大量数据的时可能会相当缓慢。
    fn largest_f<T: PartialOrd + Copy>(list: &[T]) -> T {
        let mut largest = list[0];
        for &item in list {
            if item > largest {
                largest = item;
            }
        }
        return largest;
    }
    let list1 = vec![1, 2, 3, 4, 100];
    let list2 = vec!['y', 'm', 'q'];
    println!("i32最大值: {}", largest_f::<i32>(&list1));
    println!("char最大值: {}", largest_f::<char>(&list2));

    // ! 使用 trait bound 有条件的实现方法
    // ? 通过使用带 trait bound 的泛型参数的 impl 块, 可以有条件的只为那些实现了特定 trait 的类型实现方法。
    // 比如 类型Pair<T> 总是实现了 new 方法并返回了一个 Pair<T>的实例, Self 是一个 impl 块类型的类型别名（type alias）
    // 不过在下一个 impl 块中, 只有那些为 泛型T 实现了 partialOrd trait 和 Display trait(允许打印) 的 Pair<T> 才会实现 cmp_display 方法
    struct Pair<T> {
        x: T,
        y: T,
    }

    impl<T> Pair<T> {
        fn new(x: T, y: T) -> Self {
            // Self 是一个 impl 块类型的类型别名（type alias）, 这里指的是 Pair<T>
            Self { x, y }
        }
    }

    // * 这里就表示, 仅仅为泛型T实现了 Display 与 PartialOrd struct Pair实现了 cmp_display
    impl<T: Display + PartialOrd> Pair<T> {
        fn cmp_display(&self) {
            if self.x >= self.y {
                println!("The largest member is x = {}", self.x);
            } else {
                println!("The largest member is y = {}", self.y);
            }
        }
    }

    // * 也可以对任何实现了特定 trait 的类型有条件的实现 trait。
    // * 对任何满足特定 trait bound 的类型实现 trait 被称为 blanket implementations,这种方案被广泛应用于 Rust 标准库中。
    // * 例如为标准库为任何实现了 Display trait的类型实现了 ToString trait。看起来像这样
    // impl<T: Display> ToString for T {}

    // ? 还有一种泛型, 其实一直在使用, 但是没有察觉到它的存在, 这就是 生命周期(lifetimes)。 不同于其他泛型帮助开发人员确保累心个拥有期望的行为, 生命周期则有助于确保引用在需要他们的时候一直有效。
    // ! 生命周期与引用有效性
    // ? 在之前说 引用与借用时, 遗漏了一个重要的细节: Rust 中每一个引用都有其生命周期, 也就是引用保持的作用域。
    // ? 大部分时候, 生命周期是可以推断的, 正如大部分时候类型也是可以推断的一样
    // ? 类似于当因为有多重可能类型的时候必须注明类型, 也会出现引用的生命周期以一些不同方式相互关联的情况, 所以Rust需要使用泛型生命周期参数来注明关系, 确保运行时实际使用的引用绝对有效的。

    // ! 生命周期避免了悬垂引用
    // * 生命周期的主要目标: 避免悬垂
    // * 悬垂会导致程序引用了一个非预期引用的数据。
    // 如下, 有一个外部作用于和一个内部作用域
    {
        // * 这里声明了一个变量但是没有给初始值, 乍一看好像和 Rust 不允许空指针冲突, 实际上在给他赋值之前使用, 编译就会报错, 但是赋值之后却不会, rust确实是不允许空指针的, 主要是因为生命周期的存在。
        /*
            let r;
            {
                let x = 5;
                r = &x;
            }
            println!("{}", r);
        */
        // * 上面的代码不能通过编译, 因为 r 引用的值在尝试使用之前, 就直接离开了作用域, 所以得到了下面的错误信息:
        /*
          borrowed value does not live long enough
          - `x` dropped here while still borrowed
          - borrow later used here
          变量没有存在的足够久, 原因在于 x 在抵达这一行内部作用域结束时就离开了作用域。
          不过 r 在外部作用于仍然是有效的;
          作用域越大就说它 "存在的越久"。 如果 Rust允许这段代码工作, 在内部作用域执行结束后, x被回收, &x指向空, 此时r也指向空, 那么后续任何关于r的操作, 都是无异议的。
        */
    }
    // * rust如何决定上述代码不被允许? 主要依赖于: 借用检查器
    // ! 借用检查器
    // Rust编译器有一个 借用检查器(borrow checker), 它比较作用域来确保所有的借用都是有效的。
    // 如下(和上面不同的是, 他有生命周期的注释)
      /* {
          let r;                ---------+-- 'a
                                          |
          {                               |
              let x = 5;         -+-- 'b  |
              r = &x;             |       |
          }                      -+       |
                                          |
          println!("r: {}", r);           |
      }                        */  // ---------+
      /* 
        上述代码将 r的生命周期标记为 'a, 而 x的生命周期标记为 'b
        内部的 'b 生命周期块要比外部的 'a生命周期块小得多,
        在编译时, Rust比较这两个生命周期的大小, 并发现 r 拥有生命周期 'a, 不过他引用了一个拥有生命周期 'b 的对象。
        此时程序会拒绝编译, 因为生命周期 'b 比生命周期 'a要小: 被引用的对象比它的引用者存在的时间更短。
      */

      // ? 下面给出一个无悬垂且正确编译的情况
      {
        let x = 5;            // ----------+-- 'b
                              //           |
        let r = &x;           // --+-- 'a  |
                              //   |       |
        println!("r: {}", r); //   |       |
                              // --+       |
      }                         // ----------+
      /* 
        可以看出来 生命周期 'b 是大于 生命周期 'a的, 也没有出现被引用对象比他的引用者生命周期更短的情况, 因此可以正常编译, 而不会被借用检查器抛错
      */
      
}

// ? 结构体定义中的泛型类型参数并不总是与结构体方法签名中使用的泛型是同一类型。如下
struct Point2<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point2<X1, Y1> {
    // * mixup 这个函数用调用者实例 x 与入参实例的 y, 组合成一个新的Point实例, 这个self, 代表的就是调用者本身
    fn mixup<X2, Y2>(self, other: Point2<X2, Y2>) -> Point2<X1, Y2> {
        return Point2 {
            x: self.x,
            y: other.y,
        };
    }
}

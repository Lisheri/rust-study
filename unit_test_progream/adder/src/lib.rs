#[derive(Debug)]
pub struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    pub fn can_hole(&self, other: &Rectangle) -> bool {
        return self.length > other.length && self.width > other.width;
    }
}

pub struct Guess {
  value: u32,
}

impl Guess {
  pub fn new(value: u32) -> Guess {
    if value < 1 {
      panic!("Guess value must be larger or equal to 1, got {}", value);
    } else if value > 100 {
      panic!("Guess value must be less than or equal to 100, got {}", value);
    }
    return Guess {
      value
    };
  }
}

#[cfg(test)]
mod tests {
    // * 使用外部模块的内容需要导入, 而是用通配符 *, 可将外部模块的所有内容全部导入
    use super::*;
    // * 这个 it_works 是一个测试函数是因为上面使用了 #[test]这个 attribute 进行了修饰, 而不是因为他是一个 test模块, 因为test模块中, 还可以有普通函数
    // * 运行cargo test, 就会执行所有带有 #[test]标记的函数或方法
    // * Doc-tests adder, 下面是文档测试的结果, rust能够编译出现在API文档中的代码, 这一特性可以帮助我们保证文档总是和实际代码同步
    // ? 只要函数内触发panic, 就说明测试失败了
    #[test]
    fn it_works() {
        // * it_works 就是这个测试函数的名称, 打印出来的样子就是 test::it_works
        let result = 2 + 2;
        // * 这是一个断言, 判断 result 和 4是否相等
        assert_eq!(result, 4);
    }

    // #[test]
    // fn other() {
    //  * 这个函数必失败, test的结果就是一个通过一个失败
    //   panic!("filed")
    // }

    /*
      * 使用assert! 宏检查测试结果
      + assert! 宏, 来自标准库, 用来确定某个状态是否为true
        - true: 测试通过
        - false: 调用panic!, 测试不通过
    */

    #[test]
    fn largest_can_hold_smaller() {
        let larger = Rectangle {
            length: 8,
            width: 7,
        };
        let smaller = Rectangle {
            length: 5,
            width: 1,
        };

        assert!(larger.can_hole(&smaller));
    }

    #[test]
    fn smaller_can_hold_larger() {
        let larger = Rectangle {
            length: 8,
            width: 7,
        };
        let smaller = Rectangle {
            length: 5,
            width: 1,
        };

        assert!(!smaller.can_hole(&larger));
    }

    /*
      * 使用 assert_eq!(判相等) 和 assert_ne!(判不等) 测试相等性
      这两个宏都可以传入两个参数, 并且可以判断两个参数是否相等
      通常我们把被测试代码的执行的结果作为一个参数传入, 而将所期待的结果作为另一个参数传进去, 然后对比他们是否相等
      * 实际上就相当于 assert! 的宏使用 == 或者 !=
      * 但是他们断言失败, 会自动打印两个参数的值
      * - 使用 debug 格式打印参数
      * - 要求参数实现了 PartialEq 和 Debug Traits(所有的基本类型和标准库里大部分类型其实都实现了)
    */

    /*
      ! 添加自定义错误信息
      + 可以向 assert!、assert_eq!、assert_ne! 添加可选的自定义信息
        - 这些自定义消息和失败消息都会打印出来
        - 自定义信息, 分别是第二个参数、3号参数、3号参数
        - 自定义信息内部使用format!宏, 因此可以像 println!("{}")一样使用 占位符{}, 再往后填充对应的参数
    */

    /* 
      ! 验证错误处理的情况
      + 测试除了验证代码的返回值是否正确, 还需验证代码是否如预期的处理了发生错误的情况
      + 可验证代码在特定情况下是否发生了panic
      + should_panic 属性(attribute):
        - 函数panic: 测试通过
        - 函数没有panic: 测试不通过

      ! 让 should_panic 更精确
      + 为 should_panic 属性添加一个可选的 expected 参数:
        - 将检查失败消息中是否包含所指定的文字(包含的才会通过, 否则不通过, 尽管都是panic, 但是缩小了查找问题的范围)

      ! 在测试中使用 Result<T, E>
      无需panic, 可使用 Result<E, E>, 作为返回类型编写测试:
        - 返回Ok, 测试通过
        - 返回Err, 测试失败
      
      ! 控制测试如何运行
      + 改变 cargo test 的行为: 添加命令行参数
      + 默认行为:
        - 并行运行
        - 所有测试
        - 捕获(不显示)所有输出, 使读取与测试结果相关的输出更容易.(他这个不显示是指测试通过的情况下, 不显示所有的输出, 但是通过的情况还是会打印println!这样的输出)
      + 命令行参数:
        - 针对 cargo test 的参数: 紧跟 cargo test 后
        - 针对 测试可执行程序: 放在 -- 之后
      + cargo test --help (显示所有可用命令) 
      + cargo test -- --help (显示所有可用在两个短横线之后的参数), 表示执行哪些测试信息

      ! 并行运行测试
      + 运行多个测试: 默认使用多个线程并运行。
        - 运行快
      + 确保测试之间:
        - 不会互相依赖
        - 不依赖于某个共享状态(环境、工作目录、环境变量等等), 防止多个测试共同使用同一个状态
      + --test-threads 可以控制测试时启动的线程数
        - 传递给 二进制文件
        - 不想以并行方式运行测试, 或相对线程数进行细粒度控制
        - 可以使用 --test-threads 参数, 后边跟着线程的数量
        - 例如: cargo test -- --test-threads = 1表示使用一个线程
      ! 显式函数输出
        + 默认, 如果测试通过, Rust的test库会捕获所有打印到标准输出的内容
          - 比如println!, 测试通过时将不打印
          - 失败则打印, 以及错误信息
        + 如果想在成功的测试中看到打印的内容: --show-output
      ! 按测试的名称运行测试的子集
      + 添加到cargo test的参数中
      + 运行单个测试: 指定测试明
      + 运行多个测试: 指定测试名的一部分(模块名也可以)
      ! 忽略测试
      忽略某些测试, 运行剩余的测试
      ignore属性标注 -> 
        #[test]
        #[ignore]
        fn ...() {...}
      这样上述测试函数就忽略了
      + 运行被忽略(ignore)的测试:
        --cargo test --ignored

      ! 测试分类
      + 单元测试:
        - 小、专注
        - 一次对一个模块进行隔离的测试
        - 可以测试private接口
        - #[cfg(test)]标注, 主要目的是将一小块代码隔离出来, 从而迅速的确认这段代码功能是否符合预期
        - 约定每个代码都有个test部分, 放到src目录下的同一个文件中, 约定每个源代码文件, 都有一个test文件, 用#[cfg(test)]进行标注
          - 只有运行cargo test才编译和运行代码
          - 运行 cargo build 则不会
      + 集成测试
        - 在库外部。和其他外部代码一样使用接口
        - 只能使用public接口
        - 可能在每个测试中使用到多个模块
        - 在不同目录中, 无需使用 #[cfg(test)]标注
      + cfg: configuration(配置)
        - 告诉Rust下面的条目, 只有在指定的配置选项下才被包含, cfg(test), 就只包含了test, 对应的就是 cargo test 里面的 test

      + 测试私有函数
        - Rust允许测试私有函数
    */
    // * 验证错误处理的情况如下所示
    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn greater_than_100() {
      // * 产生恐慌的条件是<= 1 || >= 100, 所以必然产生恐慌, 因此这个测试是通过的
      Guess::new(200);
    }
}

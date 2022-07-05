use std::fs::File;
use std::io::{ self, Read, ErrorKind };
fn main() {
    println!("Hello, world!");
    // panic发生时候, 有两种情况
    // 默认情况下, 当panic发生时候,
    // 程序展开调用栈(工作量大), rust沿着调用栈往回走, 清理每个遇到的函数中的数据
    // 或者立即中止调用栈: 不进行清理, 直接停止程序; 内存需要OS进行清理

    // * 如果想让二进制文件更小, 可以把设置从展开改为中止;
    // - 在 Cargo.toml 中适当的 profile 部分设置:
    // panic = "abort"

    // panic! 宏运行, 直接打印错误信息, 执行panic操作
    // 其实很多panic, 就是panic!宏在内部函数执行, 只是没有暴露出来罢了
    // 通过调用 panic! 函数的回溯信息来定位引起问题的代码
    // * 就是通过设置环境变量 RUST_BACKTRACE 可得到回溯信息
    // ! 为了获取带有调试信息的回溯, 必须启用调试符号(cargo run)(不带--release, 其实默认就不带)

    /* 
      ! 2. Result 枚举
      大部分情况下, 错误都没有严重到需要停止整个应用程序的地步, 通常情况下, 某个函数运行失败或者遇到错误
      通常是由于一些可以简单解释并做出响应的原因引起的, 比如说想读取某个文件, 但是文件又不存在, 此时就应该是考虑创建文件, 而非终止程序
      
      * 在rust中, 可以使用 Result枚举 , 来处理这种可能失败的情况
      * 有两个泛型参数, 一个是T, 代表成功的类型, 一个是E代表失败的类型
      enum Result<T, E> {
        OK<T>,
        Err<E>
      }
    */
    // * 展示如下, 因为文件可能不存在, 所以打开这个文件, 他可能存在, 也可能不存在
    // * 此时open函数的返回值 f, 他就是一个Result枚举
    // * 成功时的返回类型就是std::fs::File类型, 是个文件, 失败时, 返回的就是std::io::Error, 是个io错误
    let f = File::open("hello.text");
    // * 使用match处理Result枚举
    // * 和 Option枚举一样, Result极其变体都是由 prelude(预导入)模块带入作用域, 不需要外部引入
    /* let f = match f {
      Ok(file) => file,
      /* Err(err) => {
        * 这里err 为 Error opening file Os { code: 2, kind: NotFound, message: "No such file or directory" }
        panic!("Error opening file {:?}", err);
      } */
      // * 这里针对err, 再次match, match的是 err.kind()方法, pub fn kind(&self) -> ErrorKind
      // * 通过该方法可以获得 ErrorKind这个类型值, 他还是一个枚举, 内部变体用于描述IO操作可能引起的不同错误, 来源于 std::io::ErrorKind
      Err(err) => match err.kind() {
        // * 这里的 NotFound, 就表示是文件不存在, 因此, 在文件不存在的情况下, 我们就尝试使用File::create去创建一个文件
        // * create的返回值还是一个 Result枚举
        ErrorKind::NotFound => match File::create("hello.txt") {
          // * 正常情况下, 都会走入到这里, 创建成功
          Ok(fc) => fc,
          Err(e) => panic!("Error creating the file: {:?}", e),
        },
        // * 针对其他错误类型, 而非找不到, 就需要其他的错误分支去处理了
        other_error => panic!("Error opening the file: {:?}", other_error),
      }
    }; */

    // ? 使用闭包处理Result
    let f = File::open("hello.txt").unwrap_or_else(|error| {
      if error.kind() == ErrorKind::NotFound {
        File::create("hello.txt").unwrap_or_else(|error| {
          panic!("Error creating the file: {:?}", error);
        })
      } else {
        panic!("Error opening the file: {:?}", error);
      }
    });

    // ? unwrap, match 表达式的一个快捷方法
    // * unwrap是 Result钩子中match表达式的快捷实现, 如果Result结果是OK, 则返回OK中的值, 如果是Err, 则调用panic!宏
    /* 
      let f = File::open("hello.txt").unwrap();

      就相当于下面这个, 正确的时候返回file, 错误的时候, 自动调用panic!宏去处理错误信息

      let f = match f {
        Ok(file) => file,
        Err(err) => {
          panic!("Error opening file {:?}", err);
        }
      }
    */
    // * 但是 unwrap只能panic固定错误信息, 于是 Rust 就提供了 expect方法
    // * 功能和 unwrap非常类似, 但是他可以在 unwrap的基础上, 指定 panic!宏所附带的错误信息
    // * let errMsg = String::from("fuck");
    // * let f = File::open("hello.txt").expect(&errMsg); 这样 panic打印出来就是 fuck 

    // ! 传播错误
    // 在出现错误时, 除了立即处理错误, 还可以将错误继续传递, 让函数的调用者来决定是否处理错误, 或者如何进一步处理错误
    // * 传播错误主要就是将错误返回给调用者
    // * 有如下函数, 该函数主要是从文件中读取用户名信息
    /* fn read_username_from_file() -> Result<String, io::Error> {
      let f = File::open("hello.txt");

      let mut f = match f {
        Ok(file) => file,
        // 由于在函数体中, 因此这里其实是函数执行返回, 返回的就是 Err(e), 他就是Result的变体, 他就是一个 io:Error
        Err(e) => return Err(e),
      };

      // * 执行到此处, 表示上面的File::open已经操作成功
      let mut s = String::new();
      return match f.read_to_string(&mut s) {
        // * 这里表示函数返回, 如果读取成功, 就将string s 封装到OK中返回
        Ok(_) => Ok(s),
        // * 这个错误类型, 也是个io:Error
        Err(e) => Err(e),
      };
      // * 最终这个函数就达到了传播错误的目的
    } */

    let result = read_username_from_file();
    println!("err: {:?}", result);
    // * 其实在Rust中, 传播错误是一个普遍的操作, 所以 Rust 还专门提供了一个 ? 运算符, 用于简化传播错误的操作
    // !  "?"操作符, 传播错误的快捷方式, 表达式带上?, 如果执行是Ok, 那么就作为这一句表达式的返回值, 如果是Err, 那么将Err(e), 整体作为函数返回值
    // ! 注: ?操作符, 只能应用于 返回类型为 Result 或者 Option, 或者实现了 "Try" 的类型函数

    // ! ? 与 from 函数
    /* 
      * Trait std::convert::From 上的 from函数
        -  用于错误之间的转换, 将一个错误类型转换为另一个错误类型
      ? 而被 "?"所应用的错误, 会隐式的被 from 函数处理
      ? 当 "?" 调用 from函数时
        - 他所接收的错误类型会被转化为当前函数返回类型所定义的错误类型, 像上面的例子, 就是io:Error, 如果不是 io:Error, 也会转换为其他的类型
        - 当然也不是任意转换, 只是从 EA -> EB时, 中间有一个 from函数, 处理了这个转换过程, 这个from函数接收参数 EA, 返回结果为EB罢了
      ? 这个from函数主要针对不同的错误原因, 返回同一种错误类型, 在错误层做统一拦截处理
        - 前提是要 每个错误类型实现了转换为所返回的错误类型的from函数
    */

    /* 
      ! panic 使用规则
      
      总体原则:
      主要是当开发者认为可以代替调用代码的调用者, 来决定某些情况是不可恢复时, 那么就可以使用 panic! 宏, 来结束调用
      而函数是返回Result, 那就将错误的处理权, 返回给了代码调用者, 他就可以根据实际情况来决定是否需要 panic!

      定义一个可能失败的函数时, 优先考虑返回 Result, 否则就 panic!
    */
}

// * 如下代码和上面的功能是一样的, 但是代码量少了很多
fn read_username_from_file() -> Result<String, io::Error> {
  /* let mut f = File::open("hello.txt")?; // * 这段代码和上面的match是一样的
  * 加了这个 问号, 就表示他如果读取结果, 是一个Ok, 那么就将Ok作为表达式的结果, 如果是Err, 那么就将Err(e)作为函数返回值
  let mut s = String::new();
  f.read_to_string(&mut s)?; // * 这里成功的返回值用不到, 于是继续往下执行, 但是错误的返回值就是 Err(e), 依然是作为函数返回值
  * 继续走到这里, 就返回Ok(s)即可
  return Ok(s); */
  // ! 链式调用, 一定要 ?.到底, 只要中间有一次失败, 整个函数就返回了
  let mut s = String::new();
  let mut f = File::open("hello.txt")?.read_to_string(&mut s)?;
  return Ok(s);
}

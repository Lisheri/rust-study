// * 读取命令行参数
// * env.args函数会返回一个迭代器, 上面有一个collect方法, 可以将这些值转换为一个集合
use std::env;
// * 处理和文件相关的事务
use std::fs;
fn main() {
    // * 这个集合就是一个vector
    // * 这个env.args()无法处理非法unicode字符
    // * args_os(); 返回的是 OsString, 可以处理非法字符, 但是为了简单, 不考虑这个情况
    // + 这个args取出来, 这个vector内部, 第一个参数是当前执行的二进制程序, 后面开始就是输入的命令行参数了
    let args: Vec<String> = env::args().collect();

    let query = &args[1]; // * 也就是第二个元素的引用

    let filename = &args[2]; // * 文件名

    // ! 读取文件
    println!("Search for {}", query);
    println!("In file {}", filename);

    // * 读取可能会发生错误, 而read_to_String这个方法返回的是 Result枚举, 所以可以使用expect接收
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file!");

    println!("With text: \n{}", contents);
}

/* 
  ! 上面这个代码有四个问题
  + main函数负责的功能比较多, 一般来说一个函数只负责一个功能
  + 配置变量应该放到一个struct中, 让配置变得更加清晰
  + 读取文件使用expect处理的错误, 我们只会打印出后面的信息, 而不是正确的错误信息, 毕竟文件读取失败的错误原因很多
  + 用户使用错误, 不能够清晰的呈现, 应当在同一个地方统一收归错误信息(以及状态码)

  * 二进制程序关注点分离的指导性原则
  + 将程序拆分为 main.rs 和 lib.rs, 将业务逻辑放入 lib.rs
  + 当命令行解析逻辑较少时, 将它放在 main.rs也行
  + 当命令行解析逻辑变复杂时, 需要将它从main.rs提取到lib.rs

  + 经过上述拆分后, 留在main的功能又:
    - 使用参数值调用命令行解析逻辑
    - 进行其他配置
    - 调用lib.rs中的run函数
    - 处理run函数可能出现的错误
*/

// * 读取命令行参数
// * env.args函数会返回一个迭代器, 上面有一个collect方法, 可以将这些值转换为一个集合
use std::env;
use std::process;
use minigrep::Config;

fn main() {
    // ? 其实这个 env::args()这个函数, 本身就产生了一个迭代器, 与其将其使用collect变成一个集合, 不如直接将其传递给new函数
    // let args: Vec<String> = env::args().collect();
    let args = env::args();
    // * unwrap_or_else, 这个方法如果调用者是Ok, 那么就取出Ok的值, 如果是Err, 则执行回调函数
    // * 这里定义匿名回调函数, 参数就是 "| |" 中间的内容, 也就是 err, 花括号里面是函数体
    let config = Config::new(args).unwrap_or_else(|err| {
        // ? 这里使用eprintln! 输出到标准错误
        // ? 这样使用 cargo run > output.txt, 收集标准输出时, 就不会收集到错误信息了, 而错误信息就走到了终端上
        eprintln!("解析参数错误: {}", err);
        // ? 使用process::exit(1);立即终止程序, 1 为程序退出的状态码
        process::exit(1);
    });
    // * 这个函数不需要返回值, 所以我们只需要处理错误信息就可以了
    // * 这里使用if let来匹配 Result.Err
    if let Err(e) = minigrep::run(config) {
      eprintln!("应用程序发生错误: {}", e);
      process::exit(1);
    };
}

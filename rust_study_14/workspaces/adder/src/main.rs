// * cargo 工作空间(Workspaces)
// * 随着代码越来越多, 越来越臃肿, 通常会将代码拆分为多个包
// * 针对上述需求, cargo就提供了一个叫做工作空间的功能
/*
    * cargo工作空间: 帮助管理多个相互关联且需要协同开发的crate
    * cargo工作空间: 是一套共享同一个cargo.lock和输出文件夹的包
    
    * 创建工作空间
    + 有多种方式来组建工作空间例: 1个二进制crate, 2个库crate
        - 二进制 crate: main, 依赖于其他2个库 crate
        - 其中1个库crate提供 add_one 函数
        - 另一个库crate提供 add_two 函数  
    * 整个工作空间只有一个 cargo.lock文件, 他在根目录
    * cargo.lock保证工作空间下所有依赖版本均相同
    * 如有不同版本则解析成一个都可以使用的版本, 保证其相互兼容

    * 从 crates.io 安装二进制 crate
    + 命令: cargo install
    + 来源: https://crates.io
    + 限制: 只能安装具有二进制目标(binary target) 的 crate
    + 二进制目标 binary target: 是一个可运行程序
        - 由拥有 src/main.rs 或其他被指定为二进制文件的 crate 生成
    + 通常: README 里有关于 crate 的描述
        - 拥有 library target(库目标) 
        - 拥有 binary target(二进制目标)
    + cargo install 命令
        - cargo install 安装的二进制存放在根目录的 bin 文件夹
        - 如果你用 restup 安装的 Rust, 没有任何自定义配置, 那么二进制存放目录是 $HOME/.cargo/bin
    + 使用自定义命令来扩展 cargo
        - cargo 被设计成可以使用子命令来扩展
        - 例: 如果 $PATH 中的某个二进制是 cargo-something, 你可以像子命令一样运行:
            * cargo something
        - 类似这样的自定义命令可以通过该命令列出: cargo --list
        - 优点: 可使用 cargo install 来安装扩展, 像内置工具一样来运行
*/
use add_one;
use add_two;
fn main() {
    let num = 10;
    println!("Hello, world! {}, {}, {}", num, add_one::add_one(num), add_two::add_two(num));
}

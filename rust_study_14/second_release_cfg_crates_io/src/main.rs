//  * 如何发布 crates 到 crates.io这个位置
/* 
  + crates.io
    - 通过发布包来共享你的代码
    - crate 的注册表在 https://crates.io/中
      * 它会分发已经注册的包的源码
      * 主要托管的是开源代码(其实和npm类似)
  + 文档注释
    - 文档注释: 用于生成文档
      * 是HTML文档
      * 显式公共API的文档注释: 如何使用API
      * 使用 ///
      * 支持 Markdown
      * 放置在被说明条目之前
  + 生成HTML文档命令
    - 他会运行 rustdoc 工具(Rust安装包自带)
    - 把生成的HTML文档放在 target/doc 目录下
    - 使用cargo doc 生成文档
    - 一般不会手动打开, 而是使用命令 cargo doc --open 可以生成并浏览文档
      * 构建当前 crate 文档(也包含 crate 依赖项的文档)
      * 在浏览器打开文档
  + 常用章节
    - # Examples
    - 其他章节
      * Panics: 函数可能发生 panic 的场景
      * Errors: 如果函数返回 Result枚举, 描述可能的错误种类, 以及可导致错误的条件
      * Safety: 如果函数处于 unsafe 调用, 就应该解释函数 unsafe 的原因, 以及调用者确保的使用前提
  + 文档注释作为测试
    - 实例代码块的附加值:
      * 运行 cargo test: 将把文档注释中的实例代码作为测试来运行
  + 为包含注释的项添加文档注释
    - 符号: //! 可以为包裹当前注释的外层条目添加文档注释
    - 这类注释通常用描述crate和模块: 
      * crate root(按惯例 src/lib.rs)
      * 一个模块内, 将 crate 或模块作为一个整体进行记录

  + pub sub导出方便使用的公共API
    - 面向开发者特别友好的形式, 但是这种组织结构对代码库最终的用户并不一定友好
    - 问题: crate的程序结构在开发时对于开发者很合理, 但是对于它的使用者来说不够方便
      - 开发者会把程序结构分成很多层, 使用者想找到这种深层结构中的某个类型很费劲
    - 例如:
      * 麻烦: my_crate::some_module::another_module::UsefulType;
      * 上面这个使用起来, 就难以找到, 层次过于太深, 对于用户来说, 比较方便的是下面这种方式
      * 方便: my_crate::UsefulType;
    - 解决方案: 
      * 不需要重新组织内部代码结构
      * 使用 pub use: 可以重新导出, 创建一个与内部私有结构不同的对外公共结构(会取得某个位置的公共条目, 并将其公开到另外一个位置)
      * 例
  + 发布 crate
    - 1. 设置 crate.io的账号
      * 在发布 crate 之前, 需要在crate.io创建账号并获得API Token
      * 运行命令: cargo login [API Token]
      * 通知 cargo, 你的 API Token 存储在本地 ~/.cargo/credentials
      * API Token 可以在 https://crates.io/ 进行撤销
    - 为新的crate添加元数据
      * 在发布 crate 之前需要在 Cargo.toml的[package]区域为 crate 添加一些元数据:
        ? crate需要唯一的名称: name
        ? description: 一两句话即可, 会出现在 crate搜索的结果里
        ? license: 需提供许可证标示值(可到http://spdx.org/licenses/ 查找)
        ? 多个许可可以使用 OR 连接
        ? version: 版本
        ? author: 作者
    - crate 一旦发布, 就是永久性的: 该版本无法覆盖, 代码无法删除
      * 目的: 依赖于该版本的项目可继续正常工作
    - 发布已存在 crate 的新版本
      * 修改后, 跟换 version 重新发布即可
    - 使用 cargo yank 从 crates.io 撤回版本
      * 不能删除
      * 但可以防止其他项目把他作为新的依赖: yank(撤回) 一个 crate版本
      * 可以防止新项目依赖于该版本
      * 已经存在项目可继续将其作为依赖
    - yank意味着
      * 已生成了lock文件可以继续用
      * 没有生成当前版本的lock文件, 将无法使用
    - 使用
      * cargo yank --vers x.x.x
      * 取消 cargo yank --vers x.x.x --undo
*/  

// * 导入 pub use 重新导出的模块
use lisher_test::PrimaryColor;
use lisher_test::SecondaryColor;
use lisher_test::mix;
fn main() {
    println!("Hello, world!");
}

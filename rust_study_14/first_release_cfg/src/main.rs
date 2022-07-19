/* 
  * 本章内容
  + 通过 release profile(发布配置) 来自定义构建
  + 在 https://crates.io/上发布库
  + 通过 workspaces 组织大工程
  + 在 https://crates.io/上安装库
  + 使用自定义命令来扩展cargo
*/

// * 1. 通过 release profile(发布配置) 来自定义构建
fn main() {
    // println!("Hello, world!");
    /* 
      + release profile
        - 是预定义的
        - 可自定义的: 可使用不同的配置, 对代码编译拥有更多的控制
      + 每个 profile 的配置都独立于其他的 profile
      + cargo 主要的两个profile:
        - dev profile: 适用于开发, cargo build
        - release profile: 适用于发布, cargo build --release

      + 自定义profile
        - 针对每个 profile, cargo都提供了默认的配置
        - 如果想自定义 xxx profile 的配置:
          - 可以在 Cargo.toml里添加[profile.xxx]区域, 在里面覆盖默认配置的子集(一般只是覆盖想要修改的位置)
      
    */
}

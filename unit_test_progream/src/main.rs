// ! 1. 编写与运行测试
/* 
  测试(函数):
    - 函数
    - 验证非测试代码的功能是否和语气一致
  
  测试函数体(通常)执行3个操作(3A操作):
    - 准备数据/状态
    - 运行被测试的代码
    - 断言(Assert)结果, 是否与预期一致
  
  解析测试函数
  + 测试函数需要使用 test 属性(attribute)进行标注
    - Attribute 就是一段 Rust代码的元数据(仅对代码进行标注)
    - 在函数上增加 #[test](之前用过, 不加不能打印出内容)
    - 使用 cargo test 命令运行所有测试函数
      - Rust会构建一个 Test Runner可执行文件
      - 它会运行标注了test的函数, 并报告其运行是否成功
  + 当使用cargo 创建 library 项目时, 会生成一个 test module, 里面有一个 test 函数
    - 你可以添加任意数量的 test module 或函数
*/
fn main() {
    // println!("Hello, world!");
    // * cargo new adder --lib
    // * 创建一个 library 项目作例子
}

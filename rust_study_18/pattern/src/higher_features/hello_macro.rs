// 自定义 derive 宏

/* 
    需求:
        创建一个 hello_macro 包, 定义一个拥有关联函数 hello_macro 和 HelloMacro trait
        我们提供一个能自动实现 trait 的过程宏
        在他们的类型上标注 #[derive(HelloMacro)], 进而得到 hello_macro 的默认实现
*/

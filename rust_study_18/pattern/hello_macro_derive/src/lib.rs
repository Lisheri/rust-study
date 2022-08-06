extern crate proc_macro;

use carte::proc_macro::TokenStream;
use quote::quote; // 将 syn 产生的数据结构重新转换为rust代码
use syn; // 将rust代码从字符串转换为可供我们操作的数据结构

// 用户在他的类型上标注 #[derive(HelloMacro)] 时, 函数 hello_macro_derive, 就会被自动调用
#[proc_macro_derive(HelloMacro)]
// * 负责解析 TokenStream, 这个代码其实对于所有的创建的过程宏都是一样的, 不同的还是内部的 impl_hello_macro
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // parse 函数接收一个 TokenStream 作为输入, 返回一个 deviceInput 做为结果, 代表解析后的rust代码
    let ast = syn::parse(input).unwrap();

    // * 只负责转换语法树
    return impl_hello_macro(&ast);
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident; // 这里有被标注类型的名称, 也就是ident

    let gen = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                // * stringify! 内置宏, 接收一个表达式, 并在编译的时候将这个表达式转换为字符串的字面值, 比如传入 1 + 2 => "1 + 2"
                println!("Hello, Macro! My name is {}", stringify!(#name));
            }
        }
    };

    return gen.into();
}



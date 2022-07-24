/*
    ! Rc<T>: 引用计数智能指针

    所有权在大多数情况下都是比较清晰的, 对于一个给定的值, 可以准确的判断出哪个变量拥有它
    但是在某些场景中, 单个值也有可能被多个所有者所持有
    例如一个图数据结构, 其中一个节点, 可能就被多个所有者持有
    而一个节点, 只要存在一个指向他的边还没有被清理掉, 他就不应该被清理
    * 这个就是多重所有权
    + 为了支持多重所有权: Rust提供了 RC<T>
        - reference counting(引用计数), 他会在值的内部, 维系一个用于记录值的引用次数的一个计数器（GC回收机制中常见的引用计数法, 就是基于此）
        - 可以追踪所有到这个值的引用
        - 当引用归零, 说明这个值应该被清理了, 而且不会触发引用失效的问题
    + RC<T>使用场景
        - 当需要在heap上分配数据, 这些数据被程序的多个部分读取(只读), 但在编译时候无法确定哪个部分最后使用完这些数据。
        - 如果我们知道哪个部分最后释放这些数据, 那么只需要让这部分代码成为数据的所有者即可, 这样依靠编译时的所有权规则即可保障程序的正确性
    + 注: RC<T>只能用于 单线程场景
        - 第十六章并发会涉及到如何在多线程程序中进行引用计数。
    + RC<T>不在预导入模块(prelude), 需手动引入
    + RC::clone(&a)会增加 a的引用计数
    + RC::strong_count(&a)函数, 获得 a 的(强)引用计数
        - 与之相对的是 RC::weak_count(&a)函数 获取 a 的弱引用计数
    + 例: 一共有a, b, c三个List, 其中b 和 c共享a的所有权
*/
// * 首先定义List枚举
enum List {
    // Cons(i32, Box<List>),
    // * 使用Rc<T>
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};

use std::rc::Rc;
use rc_test::LimitTracker;
fn main() {
    // * list_a
    // let list_a = Cons(5,
    //     Box::new(Cons(10,
    //         Box::new(Nil)
    //     ))
    // );

    // ? 使用Rc
    let list_a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));

    println!("计数: {}", Rc::strong_count(&list_a));
    // let list_b = Cons(3, Box::new(list_a));
    // ? 使用Rc
    // * 这样 list_b 就不在获得 list_a 的所有权, 这样list_b 就可以共享 list_a 中的数据
    let list_b = Cons(3, Rc::clone(&list_a));
    println!("计数: {}", Rc::strong_count(&list_a));
    // * 直接这样写实际上会报错, use of moved value: `list_a`
    // * 说的是使用了已移动的值 list_a, 因为在list_b时 list_a已经发生了移动, 如果在下面还要使用 list_a, 就会报错了
    // * 因为 list_a 的所有权已经移动到了 list_b 中
    // * 这里有两种解决方案:
    // - 第一种是改变 List 中Cons的定义, 让他不在持有 Box<List>的所有权, 而是持有一个引用, 并且要指定一个生命周期参数
    // - 但是这个生命周期参数会要求List中元素的存活时间, 必须要和List本身一样
    /*
        换句话说, 借用检查器会阻止我们编写如下代码
        let a = Cons(10, &Nil);
        因为这里临时创建的 &Nil这个值, 会在a取得其引用钱就会被丢弃
        所以第一种解决方案是不可行的
    */
    // - 还有一种方法, 就是将Box<T>换为Rc<T>
    // let list_c = Cons(4, Box::new(list_a));
    let list_c = Cons(4, Rc::clone(&list_a));
    println!("计数: {}", Rc::strong_count(&list_a));

    drop(list_c);
    // * 这里可以发现, 只要变量被回收, 计数引用会自动减少
    println!("计数: {}", Rc::strong_count(&list_a));

    // * 每次调用 Rc::clone都会使引用增加
    // * 而 list_a这个引用计数器, 只有当他的引用计数归零, 他才会被清理
    // ? 还可以 list_a.clone(), 这个方法和 Rc::clone()的区别在于: Rc::clone()不会深拷贝, 仅仅是直接拷贝了一个指针, 指向的地址不变, 然后增加计数
    // ? 而 list_a.clone()会执行深拷贝, 同时增加计数
    // ? 所以惯例上使用Rc::clone()而非深拷贝

    // ! Rc引用可以保证在所有者只要还存在引用就一直有效, 同时在引用离开作用域时, 计数会自动减少, 当计数清零, 他就自动被回收了


    // * Rc<T>实际上是通过不可变引用, 使你可以在程序不同部分之间共享只读数据
    // ? 如果使用可变引用, 那么Rc<T>就会违反借用规则, 就是多个指向同一区域的可变引用会出现数据竞争以及数据的不一致
    // ? 所以他只能是通过不可变引用, 而在实际开发中, 允许数据可变无疑是非常有用的
}

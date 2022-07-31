/* 
    16. 无畏并发
    
    Concurrent(并发): 程序的不同部分之间可以独立的执行
    Parallel(并行): 程序的不同部分同时执行（有时这种也叫做并发）

    !Rust号称无畏并发: 允许你编写没有细微bug的代码(诡异bug), 并且在不引入新bug的情况下易于重构
    注: 此处的并发其实泛指 Concurrent 和 Parallel两种
*/

// Multithreading(多线程)运行代码

use std::{thread, time::Duration};

/* 
    + 进程
        在大部分现代 OS 中, 代码都是运行在进程(Process)中, OS 同时管理多个进程
    + 线程
        在开发者的程序里, 各独立部分可以同时运行, 运行这些独立部分的就是线程(thread)
        由于多个线程是可以同时运行的, 所以一般会把程序的计算拆分为多个线程来同时运行
    + 多线程运行
        - 提升性能表现
        - 增加复杂性: 无法保障各线程执行顺序
    
    + 多线程可导致的问题
        - 竞争状态: 线程以不一致的顺序访问数据或资源
        - 死锁: 两个线程彼此等待对方使用完所持有的资源, 然后线程无法继续
        - 只在某些情况下发生的bug, 而且这些bug很难可靠的复现也很难修复
    + 实现线程的方式
        - 通过调用 OS 的API来创建线程: 1:1模型(一个操作系统的线程对应一个语言的线程)
            - 优点是: 需要较小的运行时
        - 语言自己实现的线程(绿色线程): M:N模型(M个绿色线程对应N个系统线程)
            - 需要比较大的运行时
    
    + Rust: 需要权衡运行时的支持

    实际上除了汇编语言之外, 其余所有的语言都有运行时, 可能有人会说C++和C没有运行时, 实际上不然, 只是他们的运行时比较小罢了
    他们的运行时功能是比较少的, 并且生成比较小的二进制文件, 同时在多种场景下都可以与其他语言组合使用
    而有一些语言是增加运行时来提供更多的功能, 如 JAVA, C#, 或者是go等

    而对于rust来说, 他是尽可能的保持几乎没有运行时这样的状态, 这样就可以方便的与C语言进行交互, 并且获得比较高的性能

    所以 Rust标准库, 仅提供了1:1这种模型的线程

    但是 社区存在大量优良的支持M:N线程模型的第三方包

    * 创建线程:
    + 通过 thread::spawn 函数可以创建新线程:
        - 参数: 一个闭包(新线程里运行的逻辑)

*/
fn main() {
    // * 多线程例1

    // * spawn新增子线程实例
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawn thread!", i);
            // * 这个sleep指的是当前线程暂停, 内部的时间表示暂停多久, Duration::from_millis(1)表示1毫秒
            thread::sleep(Duration::from_millis(1));
        }
    });
    // * 如果在此处执行join方法, 那么再此之后的主线程将被阻塞执行, 知道handle对应的子线程执行完
    // * 才会继续往下执行
    // handle.join().unwrap();

    // * 主线程运行
    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    // * 主线程执行到此时, 无论子线程是否结束, 都会结束运行, 线程执行从主线程开始
    // * 其实主线程到4的时候就结束了, 但是此时距离主线程结束执行还有一点时间, 所以子线程还可以执行一会儿
    // * 但是这样不太好, 因为子线程的逻辑, 也许是我们需要的

    // ! 通过 join Handle 来等待所有线程的完成
    /* 
        + thread::spawn() 函数的返回值类型是 joinHandle
        + joinHandle持有值的所有权
            - 通过调用其 join 方法, 就可以等待对应的其他线程的完成
        + join方法: 调用 Handle 上的 join 方法, 会阻止当前运行线程的执行, 直到handle所表示的这些线程的终结
    */

    // * 这里调用join方法后, 就会阻塞主线程的执行, 直到handle所对应线程的结束
    // * 但是主线程的for循环已经开始, 所以不会被完全阻塞, 而是每一次sleep会切换线程, 但是主线程执行完毕后, 会等待子线程执行, 执行完毕后才会退出
    handle.join().unwrap();

    /* 
        ! 使用 move 闭包
        + move闭包通常和 thread::spawn 函数一起使用, 它允许你使用其他线程的数据
        + 其实就是在创建线程的时候, 将值的所有权从一个线程转移到另一个线程
        例：
    */

    let v = vec![1, 2, 3];
    // * 这里如果不加move, 就会报错, 因为v的所有权并不在线程handle中, 而是在主线程上
    // * 此处使用move关键字+回调函数, 就是将 所有权 转移到了线程 handle中
    let handle = thread::spawn(move || {
        println!("Here is a vector: {:?}", v);
    });

    // * 如果上面不适用move关键字移动变量v, 那么到此处可以执行如下代码:
    // * 而执行 drop回收v后, 那么子线程里的 v, 将无法使用
    // * 但是 move 可以强制让闭包拥有所有权, 也就是将v移动到子线程中
    // * 这样以后, 如下代码就会报错, 因为所有权已经移动到子线程handle中, 如下代码是无法执行的, 主线程已经不能使用v了
    // drop(v);
    
    handle.join().unwrap();
}
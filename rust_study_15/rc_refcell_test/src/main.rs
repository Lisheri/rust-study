/*
    !将RefCell<T>与Rc<T>结合使用

    这种做法很常见

    Rc<T>允许多个持有者持有同一个数据, 但只能提供对数据的不可变访问
    如果在Rc<T>中存储了 RefCell<T>, 那么就可以定义出拥有多个所有者, 并且进行修改的值    
*/

use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

fn main() {
    // * 创建一个 value, 被Rc和RefCell包裹
    let value = Rc::new(RefCell::new(5));

    // * 这里创建一个Rc列表, 其中Cons的第一个值使用Rc::clone value, 下一个列表为 Rc<Nil>
    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    // * 这里使用Rc包裹RefCell创建一个3作为当前值, 下一个值为clone的列表a
    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    // * 这里使用Rc包裹RefCell创建一个4作为当前值, 下一个值为clone的列表a
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    // * 此处自动解引用将Rc<T>解引用为RefCell<T>, 然后调用 borrow_mut()返回一个RefMut<T>
    // ! 这种用法非常常见, Rc<RefCell<T>>, 然后解引用, 就可以修改值了
    *value.borrow_mut() += 10;

    // * 结果其实很明显, 也没有违背借用规则, 因为从根本上来说, a这个指针是不变的, 只是他指向的值发生了变化, 而这个变化, 会同时响应到 b和c所依赖的a上
    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);

    // ! 除此之外, Rust还提供了其他的可实现内部可变性的类型
    /* 
        + Cell<T>: 通过复制来访问数据(RefCell<T>是通过借用)
        + Mutex<T>: 用于实现跨线程情景下的内部可变性模式(下一章说)
    */
}

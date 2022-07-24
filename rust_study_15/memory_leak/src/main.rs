/* 
    循环引用可导致内存泄漏

    Rust 的内存安全&性保证使其难以意外地制造永远也不会被清理的内存（被称为 内存泄漏（memory leak）），但并不是不可能。

    与在编译时拒绝数据竞争不同， Rust 并不保证完全地避免内存泄漏，这意味着内存泄漏在 Rust 被认为是内存安全的。

    这一点可以通过 Rc<T> 和 RefCell<T> 看出：创建引用循环的可能性是存在的。

    这会造成内存泄漏，因为每一项的引用计数永远也到不了 0，其值也永远不会被丢弃。

    + 结论: Rust可能发生内存泄漏
*/

use std::rc::{Rc, Weak};
use std::cell::RefCell;
use crate::List::{Cons, Nil};

#[derive(Debug)]
enum List {  
    Cons(i32, RefCell<Rc<List>>),
    Nil
}

impl List {
    // * 获取List的第二个值
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

fn main() {
    // 循环引用
    // let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
    // println!("a initial rc count = {}", Rc::strong_count(&a)); // 1
    // println!("a next item = {:?}", a.tail()); // Some(RefCell { value: Nil })

    // let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));
    // println!("a rc count after b creation = {}", Rc::strong_count(&a)); // 2
    // println!("b initial rc count = {}", Rc::strong_count(&b)); // 1
    // println!("b next item = {:?}", b.tail()); // Some(RefCell { value: Cons(5, RefCell { value: Nil }) })

    // if let Some(link) = a.tail() {
        // * 创建循环引用
        // * 这里使用 borrow_mut 将其原来存储的 Nil这个值, 改变为 b 存储的List
        // * 其实到这里, 就已经创建了一个循环引用了
        // *link.borrow_mut() = Rc::clone(&b);
    // }
    // println!("b rc count after changing a = {}", Rc::strong_count(&b)); // 2
    // println!("a rc count after changing a = {}", Rc::strong_count(&a)); // 2

    // * 这里实际上就形成了一个循环, a -> b -> a ....不停指下去
    // * 假如说main要走完了, 他会先释放b, 他就会将 b 中的循环计数 Rc<List>减少到1
    // * 因为 a 始终存在一个指向了 b 的引用
    // * 所以b的引用数量是1, 而不是0
    // * 那么这个List在堆内存上, 就不会得到释放
    // ! 这里获取a的第二个元素, 也就是b, 而b指向的又是a, 然后就循环了, 直接栈溢出
    // stack overflow
    // println!("a next item = {:?}", a.tail())
    // 所以说在Rust中, 并不是不能创建出循环引用, 他只是不容易

    /* 
        + 防止内存泄漏的方法
            - 1. 依靠开发者来保障, 而不是 Rust
            - 2. 重新组织数据结构: 一些引用来表达所有权, 一些引用不能表达所有权(就是将数据结构拆成两种所有权持有情况)
                - 循环引用中的一部分具有所有权关系, 另一部分不涉及所有权关系
                - 而只有所有权关系才影响值的清理
        + 防止循环引用, 把Rc<T>(强引用) 换成 Weak<T>(弱引用)

        + Rc::clone()为Rc<T>实例的strong_count +1, Rc<T>的实例只有在 strong_count 为 0 的时候才会被清除
        + Rc<T>实例通过调用Rc::downgrade 方法可以创建值的 Weak Reference(弱引用)
            - 返回类型是Weak<T>(智能指针)
            - 每次调用 Rc::downgrade都会为 weak_count +1
        + Rc<T>使用 weak_count 来追踪存在多少 Weak<T>
        + weak_count 不为0并不影响Rc<T>实例的清理

        + Strong VS Weak
            - Strong Reference(强引用) 是关于如何分享 Rc<T> 实例的所有权
            - Weak Reference(弱引用) 并不表达上述意思
            - 使用 weak reference 并不会创建循环引用
        + 使用 Weak Reference 并不会创建循环引用
            - 当强引用值为0时, 弱引用会自动断开
        + 在使用 Weak<T>之前, 需要保证他指向的值依然存在:
            - 在 Weak<T>实例上调用 upgrade方法, 返回 Option<Rc<T>>
    */
    let leaf = Rc::new(Node {
        value: 3,
        children: RefCell::new(vec![]),
        // * 这里需要一个指向他爹的引用, 但如果使用Rc, 就会创建循环引用, 因此这里使用Weak<T>, 避免循环引用
        parent: RefCell::new(Weak::new())
    });

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

    // * 通过parent 来访问爹
    // ? borrow返回不可变引用, 然后通过 upgrade来返回一个 Option<Rc<T>>, 也就是对应的爹的Option枚举
    // println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)])
        });

        // * 需要将leaf的爹指向 branch
        // ? 通过Rc::downgrade创建弱引用
        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
        
        // * 创建完branch后, branch有 1个强引用(本身的), 1个弱引用(leaf关联出来的)
        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );
        
        // * 此时leaf 有 2个强引用(1个是本身的, 一个是branch的children对leaf带来的), 0个弱引用
        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    }
    // * 爹在上述作用域被回收, 可以看到弱引用不会影响回收, 而强引用归零0了, 所以爹就被回收了
    // * 此处可以看到 parent 已经被回收了, 变成了 None
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    // * 此时的leaf和创建时强弱引用一致
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

    // let branch = Rc::new(Node {
    //     value: 5,
    //     parent: RefCell::new(Weak::new()),
    //     children: RefCell::new(vec![Rc::clone(&leaf)])
    // });

    // * 需要将leaf的爹指向 branch
    // ? 通过Rc::downgrade创建弱引用
    // *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    // println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

}

// 例: 创建一棵树
#[derive(Debug)]
struct Node {
    value: i32,
    // 这里使用 Rc<Node>是为了让他所有的子节点都能共享所有权
    // 为了灵活修改父子关系, 外面又套了一个 RefCell
    children: RefCell<Vec<Rc<Node>>>,
    parent: RefCell<Weak<Node>>
}


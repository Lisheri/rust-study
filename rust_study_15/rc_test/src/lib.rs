/*
    ! RefCell<T>和内部可变性

    + 内部可变性(Interior mutability)
        - 内部可变性是 Rust 的设计模式之一
        - 它允许你在只持有不可变引用的前提下对数据进行修改(通产来说类似的行为会被借用规则所禁止)
            - 但是 RefCell在它的数据结构中使用了 unsafe 代码来绕过Rust中正常的可变性和借用规则
*/

/*
    ? RefCell<T>

    + 与Rc<T>不同, RefCell<T>类型代表了其持有数据的唯一所有权

    ? 借用规则:
    ? 1. 在任何给定的时间里, 要么只能拥有一个可变引用, 要么只能拥有任意数量的不可变引用
    ? 2. 引用总是有效的

    ? RefCell<T> 与 Box<T>的区别:
    + Box<T>:
        - 在编译阶段强制代码遵守借用规则
        - 不满足则抛错
    + RefCell<T>:
        - 只会在运行时检查借用规则
        - 否则触发panic

    ? 借用规则不同阶段检查比较
    + 编译阶段:
        - 尽早暴露问题
        - 没有任何运行时开销
        - 对大多数场景都是最佳选择
        - Rust的默认行为
    + 运行时:
        - 问题暴露延后, 延后到了运行时, 甚至问题暴露到了生产环境
        - 因借用计数会产生些许性能损失
        - 实现某些特定的内存安全场景(不可变环境中修改自身数据)
            * rust 编译器, 是在编译阶段就检查所有的代码
            * 其中大部分代码都可以分析明白, 如果没有问题就通过, 有问题就抛错
            * 而且 rust 编译器本质上来说非常保守, 因为某些代码并不是能够在编译阶段就能分析明白的
            * 而rust编译器会简单拒绝所有不符合所有权规则的代码, 哪怕这些代码根本没有问题, 只是rust编译器没有分析明白
            * 他之所以这样做的原因在于他要保证他的安全性, 如果他放行了有问题的代码, 那么rust对安全性的保证将直接破产, 失去用户的信任
            * 虽然拒绝掉某些正确的程序会对开发者造成不便, 但至少这样不会产生任何灾难性后果
            * 然而针对这些编译器无法理解的代码, 如果开发者可以保证借用规则能被满足, 那么这个时候 RefCell<T>就有了用武之地


    + 与 Rc<T>一样, 只能用于 单线程 场景

    ? 选择 Box<T>、Rc<T>、RefCell<T>的依据
        + 同一数据所有者
            - 一个: Box<T>
            - 多个: Rc<T>
            - 一个: RefCell<T>
        + 可变性、借用检查
            - 可变、不可变借用(编译时检查): Box<T>
            - 不可变借用(编译时检查): Rc<T>
            - 可变、不可变借用(运行时检查): RefCell<T>
    ? 由于RefCell<T>允许开发者在运行时检查, 所以即便RefCell<T>本身是不可变的, 但我们依然可以修改它内部存储的值
*/
// ? 内部可变性: 可变的借用一个不可变的值
// ? 借用规则有一个推论: 无法可变的借用一个不可变的值
// let x = 5;
// * 这里是会报错的, 因为不可以可变的借用一个不可变的值
// let y = &mut x;
// * 但有时候我们需要一个这样的情况, 他对外部是不可变的, 但他同时可以在方法内部修改自身的值
// * 当然, 除了这个值本身的方法, 其余的值均不能修改这个值
// * 而是用 RefCell<T>就是获得一种内部可变性的方法, 不过这个 RefCell<T>并没有完全绕开借用规则
// * 虽然是使用了内不可变性通过了编译阶段的借用检查, 但是借用检查的工作仅仅是延后到了运行时
// * 如果违反借用规则, 那么会得到一个panic, 而不是编译时的错误

// test double
// * 是指测试替代这个意思, 是一个通用编程概念, 代表了测试工作中被用作其他类型的替代品
// * 而 test double, 叫做测试替代, 内部有一种特定的类型, 叫做 模拟对象(mock object), 他会承担起记录测试过程中的工作
// * 我们可以使用这些记录, 来断言 assert 这个测试工作的运行是否正确
// * 而在 rust中, 没有这种类似的概念, 在标准库里面也没有提供这种模拟对象, 也就是 mock object的测试功能
// * 但是可以自定义一个结构体来实现和 mock object 相同的功能

// 例： 一个记录某个值与最大值差距的库，并根据此值的特定级别发出警告
pub trait Messenger {
    // * send 方法接收的是self的不可变引用, 和一个文本消息
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    // messenger 就是 Messenger类型
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        return LimitTracker {
            messenger,
            value: 0,
            max,
        };
    }

    pub fn set_value(&mut self, value: usize) {
        // * 此方法不会返回任何值, 也就是说不会提供任何结果进行断言
        // * 而我们要测试的是, 当某一个实现了 Messenger 这个 trait 的值, 和一个 max 值来创建一个 LimitTracker时
        // * 传入不同的value, 就能够触发 messenger 发送不同的消息

        // * 而我们这个模拟对象 mock object, 在调用 send 的时候, 只需要将收到的消息存档记录即可, 不需要真正的发邮件或发短信
        // * 而我们使用这个 mock object 这个模拟对象来创建 LimitTracker 实例之后, 我们便可以通过调用 set_value 这个方法来检查模拟对象中是否存储了我们希望看到的这个消息
        self.value = value;

        // * 这里就是看 value 占用了 max 的百分比
        let percentage_of_max = self.value as f64 / self.max as f64;
        
        // * 如果占用大于了1, 则给出一个错误信息
        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            // * 占用大于了 90%, 给出紧急警告快要超过阈值了
            self.messenger
                .send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            // * 大于75%, 则给出提示
            self.messenger
                .send("Warning: You've used up over 75% of your quota!");
        }
    }
}

// * 按照上述思路, 实现了下面的测试代码
#[cfg(test)]
mod tests {
    // 引入 RefCell
    use std::cell::RefCell;

    use super::*;

    struct MockMessenger {
        // send_messages: Vec<String>,
        // * 使用 RefCell实现内部可变性
        send_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            return MockMessenger {
                // * 这个用于存储已经发送过的消息
                send_messages: RefCell::new(vec![]),
            };
        }
    }

    // * 实现了 Messenger 之后, 这个MockMessenger就可以用于创建 LimitTracker
    impl Messenger for MockMessenger {
        // ! 在这里会编译报错, 因为我们定义的send方法, 他的self值, 应该是不可变的, 但是在后续使用过程中, 它实际上是可变的
        // ! 这个, 就叫做内部可变性, 其实只需要将 Vec套在一个 RefCell里即可
        // fn send(&mut self, message: &str) {
        // ! 改造成 RefCell之后, 这个 mut 就可以去掉了, self其实就是一个不可变引用
        fn send(&self, message: &str) {
            // * 我们的意图就是在执行send方法时, 他的消息, 可以存储到 send_messages 里
            // self.send_messages.push(String::from(message));
            // ! 上述代码也需要改造, 需要使用 borrow_mut() 方法
            // ! borrow_mut 方法可以获得内部值的可变引用, 然后调用push, 将消息记录下来
            self.send_messages.borrow_mut().push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        // * 这个测试就是测试超过75%的这一段
        // * 首先创建一个 MockMessenger实例
        let mock_messenger = MockMessenger::new();
        // * 然后创建一个 LimitTracker 实例
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);
        // * 设置值为80
        limit_tracker.set_value(80);

        // * 断言发送的消息长度是1
        // ! 这里调用 borrow方法, 获取内部值的不可变引用
        assert_eq!(mock_messenger.send_messages.borrow().len(), 1);
    }
}


// * 使用 RefCell<T>在运行时记录借用信息
// + 两个方法(安全接口):
//      - borrow方法: 返回智能指针 Ref<T>, 实现了 Deref
//      - borrow_mut 方法: 返回智能指针 RefMut<T>, 也实现了 Deref

/* 
    * 使用 RefCell<T>在运行时记录借用信息
    + RefCell<T> 会记录当前存在多少个活跃的 Ref<T>, 和 RefMut<T> 智能指针:
        - 每次调用 borrow() 方法, 不可变借用计数加1
        - 任何一个 Ref<T>离开作用域被释放(或主动调用drop方法), 不可变借用计数就会减1
        - 每次调用 borrow_mut(), 可变借用计数就会加1
        - 而任何一个 RefMut<T>离开作用域被释放(或主动调用drop方法释放), 可变借用计数就会减1
    + Rust就是通过上述技术, 来维护借用检查规则:
        - 在任何给定时间里, 只允许拥有多个不可变借用和一个可变借用
        - 当我们违背上述借用规则, RefCell<T>就会在运行时触发panic
    + 将Rc<T>和RefCell<T>结合使用来实现一个拥有多重所有权的可变数据
    例见 ../rc_refcell_test
*/
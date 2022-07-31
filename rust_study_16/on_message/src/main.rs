use std::{sync::mpsc, thread, time::Duration};

/* 
    使用消息传递来跨线程传递数据
    + 有一种技术越来越流行, 而且他还能保证安全并发, 这就是 ———— 消息传递。
        - 在这种机制中, 线程(或Actor)通过彼此发送消息(数据)来进行通信
    + Go语言的名言: 不要用共享内存来通信, 要用通信来共享内存
    + Rust也提供了这样一种通信方式, 也就是 Channel(标准库提供的)

    * Channel
    + Channel 包含: 发送端、接收端(发布订阅模式)
    + 调用发送端的方法, 发送数据
    + 接收端会检查和接收到达的数据
    + 如果发送端、接收端中任意一端被丢弃了, 那么 Channel就 “关闭” 了

    * 创建Channel
    + 使用 mpsc::channel 函数来创建 Channel
        - mpsc 表示multiple producer, single consumer(多个生产者, 一个消费者), 其实就是可以有多个发送端, 但是只能有一个接收端
        - 返回一个tuple(元组), 第一个元素是发送端, 第二个元素是接收端
    + 发送端的send方法
        - 参数: 想要发送的数据
        - 返回 Result<T>:
            - 如果有问题（如接收端已被弃用）, 则返回一个错误
    + 接收端的方法
        - recv 方法: 阻止当前线程的执行, 直到Channel中有值被传进来
            - 一旦收到值, 就会返回一个Result<T, E>
            - 一旦所有发送端关闭, 就会收到一个错误
        - try_recv 方法, 不会阻塞当前线程执行
            - 立即返回一个 Result<T, E>
                - 有数据到达: 返回Ok<T>, 里面包含数据
                - 否则返回错误
            - 通常会循环调用检查 try_recv的结果
                - 一旦消息过来就进行处理, 如果还没有来, 那么将进行其他的一些指令
    例如下
*/
fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        // * `unwrap` 在接收到 `None` 时将返回 `panic`
        // * 新的线程必须拥有发送端的所有权, 才能往通道里发消息
        // ? 使用send方法发送消息, 返回的是一个 Result<T>, 如果此时发送端或接收端已经被丢弃了, 那么这里使用unwrap来触发一个panic
        tx.send(val).unwrap();
    });

    // ? 接收端有两个方法可以获取消息
    // ? 这里使用recv方法, 他会一直阻塞线程, 直到有消息被传入, 如果有消息传入, 结果还是被包裹在 Result<T>中, 有消息就返回 Ok<T>, 否则返回Err
    // ? 同样使用unwrap处理一个None, 来发生恐慌
    let received = rx.recv().unwrap();
    println!("Got: {}", received);


    // ? Channel和所有权转移
    // + 所有权在消息传递中非常重要: 能帮你编写安全、并发的代码

    // 例
    let (tx2, rx2) = mpsc::channel();
    thread::spawn(move || {
        let val = String::from("hi");
        tx2.send(val).unwrap();
        // * 这里val的所有权已经在 send 时被移交, 所以下面再次借用 val, 将会报错
        // println!("发送的值是: {}", val);
    });

    let received = rx2.recv().unwrap();
    println!("Got: {}", received);
    

    // 发送多个值, 看到接受者在等待
    let (tx3, rx3) = mpsc::channel();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for val in vals {
            tx3.send(val).unwrap();
            // * 暂停200ms看接收端效果
            // * 可以看到他是间隔200ms, 才会打印数据
            // * 所以接收端是每接收到一个数据, 才会执行一次循环体
            thread::sleep(Duration::from_millis(200));
        }
    });

    // let receiveds = rx3.recv().unwrap();
    // * 可以将 接收端 当迭代器使用, 这样就不用调用 recv 函数, 每收到一个值, 就打印出来
    // * 而当channel关闭后, 就会退出循环
    for received in rx3 {
        println!("got: {}", received);
    }

    // * 通过克隆创建多个发送者
    let (tx4, rx4) = mpsc::channel();
    // * 克隆一个发送端
    let tx4_1 = mpsc::Sender::clone(&tx4);

    // * 线程1使用克隆出来的tx4_1发送
    thread::spawn(move || {
        let vals = vec![
            String::from("1： hi"),
            String::from("1： from"),
            String::from("1： the"),
            String::from("1： thread"),
        ];
        for val in vals {
            tx4_1.send(val).unwrap();
            thread::sleep(Duration::from_millis(200));
        }
    });

    // * 线程2 使用原来的 tx4发送
    thread::spawn(move || {
        let vals = vec![
            String::from("4： hi"),
            String::from("4： from"),
            String::from("4： the"),
            String::from("4： thread"),
        ];
        for val in vals {
            tx4.send(val).unwrap();
            thread::sleep(Duration::from_millis(200));
        }
    });

    for received in rx4 {
        // * 可以看到两个线程会交替出现, 但是交替的顺序是不能保证的(一定是1和4, 但是1和4谁先出现无法保证)
        println!("gotrx4: {}", received);
    }

}

// mod blog

// 不同状态博文所共享的行为
trait State {
    // * 这里使用的 self: Box<Self>, 表示该方法只能在持有这个类型的 Box上被调用。
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, post: &'a Post) -> &'a str;
}
struct Draft {}

struct PendingReview {}

struct Published {}

impl State for Draft {
    // * Draft 的 request_review 方法需要返回一个新的，装箱的 PendingReview 结构体的实例，其用来代表博文处于等待审核状态
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        return Box::new(PendingReview {});
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        return self;
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        return "";
    }
}

impl State for PendingReview {
    // * 这个表示处于pendingReview的State, 继续调用 request_review, 则保持状态不变
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        // * 表示状态走向完成
        return Box::new(Published {})
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        return "";
    }
}

// * 完成状态后, 无论是调用审核还是完成, 状态都不会再发生变化
impl State for Published {
    fn approve(self: Box<Self>) -> Box<dyn State> {
        return self;
    }

    fn request_review(self: Box<Self>) -> Box<dyn State> {
        return self;
    }

    // * 完成的时候返回 post 中的 content字段
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        return &post.content;
    }
}

pub struct Post {
    state: Option<Box<dyn State>>,
    content: String
}

impl Post {
    pub fn new() -> Post {
        return Post {
            // * 保证无论何时创建一个新的 Post, 都会从草案开始
            state: Some(Box::new(Draft {})),
            content: String::new()
        }
    }

    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    // * 根据状态返回内容
    pub fn content(&self) -> &str {
        // * 因为目标是将所有像这样的规则保持在实现了 State 的结构体中，我们将调用 state 中的值的 content 方法并传递博文实例（也就是 self）作为参数。
        // * 接着返回 state 值的 content 方法的返回值。
        // * 这里调用 Option 的 as_ref 方法是因为需要 Option 中值的引用而不是获取其所有权。
        // * 因为 state 是一个 Option<&Box<dyn State>>，调用 as_ref 会返回一个 Option<&Box<dyn State>>。
        // * 如果不调用 as_ref，将会得到一个错误，因为不能将 state 移动出借用的 &self 函数参数。
        // ? as_ref() 将对Option的引用变为对Option所包含对象的不可变引用，并且返回一个新的Option
        // ? 对这个新的Option进行unwrap操作，可以获得原Option所包含的对象的不可变引用（这里就是 Box<dyn State>）
        return self.state.as_ref().unwrap().content(self);
    }

    // * Post增加一个获取 self 可变引用的公有方法 request_review。接着在 Post 的当前状态下调用内部的 request_review 方法
    // * 并且第二个 request_review 方法会消费当前的状态并返回一个新状态
    // ? 将阶段置为等待审核阶段
    pub fn request_review(&mut self) {
        // * 为了消费老状态，request_review 方法需要获取状态值的所有权。
        // * 调用 take 方法将 state 字段中的 Some 值取出并留下一个 None，因为 Rust 不允许结构体实例中存在值为空的字段。
        // * 这使得我们将 state 的值移出 Post 而不是借用它。接着我们将博文的 state 值设置为这个操作的结果。
        // * 我们需要将 state 临时设置为 None 来获取 state 值, 即老状态的所有权, 而不是使用 self.state = self.state.request_review(); 
        // * 这样的代码直接更新状态值。这确保了当 Post 被转换为新状态后不能再使用老 state 值。
        // ? 所以这里是把self.state编程None,  然后将值取出, 比如: let mut a = Some(Box::new(5));let d = a.take(); 然后a -> None, 但是d -> Some(5)
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review());
        }
    }

    // * 将 state 设置为审核通过时应处于的状态
    pub fn approve(&mut self) {
        // * 和 request_review 一样, 需要将 state 临时设置为 None, 来获取 state 的值
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve());
        }
    }
}
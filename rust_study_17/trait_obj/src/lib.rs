pub trait Draw {
    fn draw(&self);
}

// * 在标准库中, 克隆对象的trait就是一个不符合对象安全的例子, 签名如下, 会返回Self
pub trait Clone {
    fn clone(&self) -> Self;
}

pub struct Screen {
    // * dyn 用于表示Box指针中的元素都实现了 Draw 这个 trait
    pub components: Vec<Box<dyn Draw>>,
    // * 这里使用dyn Clone 就会报错, 报错信息: Clone 这个trait 不能变成对象, 因为它的返回类型是Self, 所以他不是对象安全的
    // pub components_clones: Vec<Box<dyn Clone>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Button {
    pub width: i32,
    pub height: i32,
    pub label: String
}

impl Draw for Button {
    fn draw(&self) {
        // 实现细节 ———— 绘制一个按钮
    }
}

// 泛型实现, 如下, 但是泛型实现有个缺点, 就是一次只能使用一个类型
// 那么我们使用Button这个泛型的时候, Vector中就不方便使用TextField了
// 但是 Box<dyn Draw>, 表示只要这个类型实现了Draw 这个 trait, 就都可以放在components里面
/* pub struct Screen<T: Draw> {
    pub components: Vec<T>,
}

impl<T> Screen<T>
where
    T: Draw,
{
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
} */
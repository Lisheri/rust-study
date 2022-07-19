fn main() {
    // println!("Hello, world!");
    // * 之前留了一个疑问, 说是手动写for循环更快
    // * 这里做一个测试
    // + 将一本小说的内容, 放在一个String里面, 搜索一个 the
    // + 结果显示, 迭代器其实更快

    // 迭代器在rust中是一种高层次的抽象, 但是他在编译了之后, 生成了和我们手写的for循环几乎一样的代码
    // 这套抽象, 在rust中叫做 ———— "零开销抽象" Zero-Cost Abstraction
    // + 意味着在使用抽象时不会引入额外的运行时开销

    // 音频解码器例子
    let buffer: &mut [i32];
    let coefficients: [i64; 22];
    let qlp_shift: i16;

    for i in 12..buffer.len() {
        let prediction = coefficients.iter()
            // rust在处理这个zip的时候, 并不会使用所谓的循环, 因为rust知道 buffer有12个成员, 这里使用了所谓的展开策略
            // 大概理解为下面这段代码复制了12次, 这样来消除循环控制语句带来的性能开销
            .zip(&buffer[i - 12..i]) // 将迭代器的12个元素配对
            .map(|(&c, &s)| c * s as i64) // 配对后的迭代元组项相乘返回一个新的迭代器
            .sum::<i64>() >> qlp_shift; // 新的迭代器求和
        let delta = buffer[i];
        buffer[i] = prediction as i32 + delta;
    }
}

// 可以大胆使用闭包和迭代器, 因为他们不会带来性能损失, 同时还能保持运行时的性能高效
// 这也是rust一直以来致力于实现的一个所谓的零开销抽象的环节
/*
    In general, C++ implementations obey the zero-overhead principle: What you don’t use, you don’t pay for. And further: What you do use, you couldn’t hand code any better.
    Bjarne Stroustrup "Foundations of C++"
    从整体来说，C++ 的实现遵循了零开销原则：你不需要的，无需为他们买单。更有甚者的是：你需要的时候，也不可能找到其他更好的代码了。

    本贾尼·斯特劳斯特卢普 "Foundations of C++"
*/

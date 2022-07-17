/*
  函数式语言的特性: 闭包(closures)
*/
use std::collections::HashMap;
use std::thread;
use std::time::Duration;

fn main() {
    // 1. 使用闭包创建抽象行为
    // * 闭包: 可以捕获其所在环境的匿名函数(和js不太一样)
    /*
      + 闭包
        - 是个匿名函数
        - 可以作为变量、作为参数、或者是作为函数返回值
        - 可以在一个地方创建闭包, 在另一个上下文中调用闭包来完成运算
        - 可从其定义的作用域内捕获值
    */
    // * 生成自定义运动计划的程序
    /*
      + 算法逻辑不是重点, 重点是算法中的计算过程需要几秒钟时间
      + 目标: 不让用户发生不必要的等待
        - 仅在必要时调用该算法
        - 只调用一次
    */
    println!("Hello, world!");
}

/* fn simulated_expensive_calculation(intensity: u32) -> u32 {
  println!("calculating slowly...");
  thread::sleep(Duration::from_secs(2));
  return intensity;
} */

/*
  + 闭包的类型推断
    - 闭包不要求标注参数和返回值类型(严格定义接口有助于所有人对参数和返回值的类型取得共识, 但是闭包, 并不用于暴露接口, 会被存储与变量中, 使用时, 也不需要命名, 也不会被暴露给代码库的用户)
    - 闭包通常很短小, 只是在狭小的上下文中工作, 编译器通常能够推断出类型
    - 可以手动添加类型标注
  + 闭包的类型推断
    - 注意: 闭包的定义最终只会为参数/返回值推断出唯一具体的类型, 如下:

    let example_closure = |x| x;
    上述闭包在不使用时, 会直接报错, 因为无法推断准确的类型, 但加上如下语句, 则可以推断出相对的类型, 也就不会报错了
    let s = example_closure(String::from("hello"));
    但是上述代码, 调用后, 如果添加如下语句, 如下语句会报错, 因为闭包, 只能推断出唯一的具体类型, 在上一句结束时, 类型已经绑定到闭包上了, 推断结果为String, 自然不能使用 i32类型
    let n = example_closure(5);
*/

fn generate_workout(intensity: u32, random_number: u32) {
    // ? 定义闭包, 这里属于是定义了一个匿名函数, 然后使用变量 expensive_closure 来接收定义的匿名函数
    // * 因为结构体可能会对value进行修改, 所以这里要加一个mut标志
    let mut expensive_closure = Cacher::new(|num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        return num;
    });

    // let calculation_result: u32 = expensive_closure(intensity);
    // * 可以创建一个 struct, 它持有闭包极其调用结果
    //  - 只会在需要结果时才执行该闭包
    //  - 可缓存结果
    // * 这种模式通常叫做记忆化(memoization) 或者是 延迟计算(lazy evaluation) (其实在js中很常见)
    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_closure.value(intensity));
        println!("Next, do {} situps!", expensive_closure.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_closure.value(intensity)
            );
        }
    }
}

// * 如何让 struct 持有闭包?
/*
  + struct 的定义需要知道所有字段的类型
    - 需要致命闭包的类型
  + 每个闭包实例都有自己唯一的匿名类型, 即使两个闭包签名完全一样
  + 所以需要使用: 泛型和Trait Bound
  + Fn Trait
    - 由标准库提供
    - 所有闭包都至少实现了以下 trait 之一
      ? Fn(这里只使用这个)
      ? FnMut
      ? FnOnce
*/

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    // value: Option<u32>,
    // * 用map存储已执行过的值, 修复value只能存储一个值的情况
    map: HashMap<u32, u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        let mut map = HashMap::new();
        return Cacher {
            calculation,
            // value: None,
            map,
        };
    }

    fn value(&mut self, arg: u32) -> u32 {
        // * 这里匹配一下self.value这个字段, 如果字段有结果, 直接返回, 说明执行过, 如果没有结果, 也就是None, 说明没有执行过, 那么执行闭包, 然后将结果赋给value, 并返回当前结果
        // self.map.entry(arg).or_insert((self.calculation)(arg));
        return match self.map.get(&arg) {
            Some(v) => *v,
            None => {
                let v = (self.calculation)(arg);
                self.map.insert(arg, v);
                return v;
            }
        };
        // match self.map {
        //     Some(v) => v,
        //     None => {
        //         let v = (self.calculation)(arg);
        //         self.value = Some(v);
        //         return v;
        //     }
        // }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn call_with_different_values() {
        let mut c = super::Cacher::new(|a| a);
        let v1 = c.value(1);
        let v2 = c.value(2);

        // * 这个测试必然失败, 因为在第一次调用value后, value值变成了1, 那么匹配到Some(1)后, 会直接返回1, 因此这里的v2, 值还是1
        assert_eq!(v2, 2);
    }
}

/*
  * 使用缓存器(Cacher) 实现的限制
  + 1. Cacher实例针对不同的参数 arg, value方法总会得到同样的值
    - 可以使用 HashMap 代替单个值:
      * key: arg 参数
      * value: 执行闭包的结果
  + 2. 目前该Cacher只能接收一个 u32类型的参数和一个u32类型的返回值
    - 可以引入多个泛型参数
*/

/*
  * 闭包可以捕获他们所在的环境
  + 闭包可以访问定义它的作用域内的变量, 普通函数则不能(其实就是访问了父级函数作用域的变量, 易产生副作用)
  + 会产生额外的内存开销(闭包不销毁, 内存不回收) 所以大多数情况下还是使用函数好

  * 函数从所在环境捕获值的方式
  + 与函数获得参数的三种方式一样
    - 1. 获得所有权: FnOnce, 这就是意味着闭包可以从定义它的作用域内(闭包所处环境中)来消耗捕获的变量, 为了实现这个功能, 闭包必须在定义时, 取得这些变量的所有权, 并且其移动到闭包内部
      - 而闭包无法多次获取并消耗同一个变量, 所以他只能被调用一次, 因此它叫做FnOnce
    - 2. 可变借用： FnMut, 可以从环境中可变的借用值, 并进行修改
    - 3. 不可变借用: Fn, 从环境中不可变借用值
  + 创建闭包时, 通过闭包对环境值的使用, Rust可以推断出具体使用那个 trait:
    - 所有的闭包都实现了 FnOne
    - 没有移动捕获变量的实现了FnMut
    - 无需可变访问捕获变量的闭包实现了Fn
  (原理较复杂, 这里不深入, 实际上所有实现了Fn的都实现了FnMut, 而所有实现了FnMut的, 都实现了FnOnce)
  + move关键字
    - 在参数列表前使用move关键字, 可以强制闭包取得它所使用的环境的所有权
    - 当将闭包传递给新线程以移动数据使其归新线程所有时, 此技术最为有用
  + 最佳实践
    - 当指定Fn trait bound之一时, 首先使用Fn, 基于闭包体里的情况, 如果需要FnOnce或FnMut, 编译器会告诉你
*/

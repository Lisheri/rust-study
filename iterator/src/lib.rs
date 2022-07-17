/* #[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iterator_demonstration() {
        // let v1 = vec![1, 2, 3];
        /* let mut v1_iter = v1.iter();

        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3)); */

        // * sum
        /* let v1_iter = v1.iter();
        let total: i32 = v1_iter.sum(); // * sum方法会通过next遍历所有元素, 将一个个的元素添加到总和内, 最终会耗尽迭代器
        assert_eq!(total, 6); */

        // * map
        let v1: Vec<i32> = vec![1, 2, 3];
        // * 这里会产生一个警告, 就是说迭代器是惰性的, 如果没有消耗他们, 那么他们就什么都不会做
        // * 意思是如果不调用消耗性适配器方法, 那么他们就什么都不会做, 就是说下面这个状态, 并不会对v1中的元素进行 +1 操作
        // v1.iter().map(|x| { return x + 1; });
        // ? collect方法
        // * 所以这里将使用collect方法, 这里会把结果值收集到某一个集合里面
        // ? Vec<_>里的 "_"表示让rust推断元素的类型, 实际上是一个i32
        let v2: Vec<_> = v1
            .iter()
            .map(|x| {
                return x + 1;
            })
            .collect();
        assert_eq!(v2, vec![2, 3, 4]);

        super::filer_by_size();
    }
} */

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

// * 返回了一个过滤后的迭代器
fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    // * 通过 into_iter 创建一个迭代器, 这个迭代器是获取了所有权的
    // * 相等鞋号则保留, 不等过滤
    shoes.into_iter().filter(|x| x.size == shoe_size).collect()
}

/* #[test]
pub fn filer_by_size() {
    let shoes = vec![
        Shoe {
            size: 10,
            style: String::from("sneaker"),
        },
        Shoe {
            size: 13,
            style: String::from("sandal"),
        },
        Shoe {
            size: 10,
            style: String::from("boot"),
        },
    ];
    let in_my_size = shoes_in_my_size(shoes, 10);
    assert_eq!(
        in_my_size,
        vec![
            Shoe {
                size: 10,
                style: String::from("sneaker")
            },
            Shoe {
                size: 10,
                style: String::from("boot")
            },
        ]
    )
} */

// * 此迭代器从1 遍历到 5
struct Counter {
    // 存储迭代状态
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    // * Option<Self::Item> 大约就和 Option<u32>差不多
    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            // * 每次迭代 +1, 并返回结果
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

// #[test]
// fn calling_next_directly() {
//   let mut counter = Counter::new();

//   assert_eq!(counter.next(), Some(1));
//   assert_eq!(counter.next(), Some(2));
//   assert_eq!(counter.next(), Some(3));
//   assert_eq!(counter.next(), Some(4));
//   assert_eq!(counter.next(), Some(5));
//   assert_eq!(counter.next(), None);
// }

#[test]
fn using_other_iterator_trait_methods() {
    // ? 在另启一个新的迭代器, 从2 到 5迭代, 然后依次求积, 在过滤掉不能整除3的最后求和
    // ? zip 表示拉链, 就是将两个迭代器的每个元素捏到一起作为一个元组元素, 产生一个新的迭代器, 每个成员就是一个元组, 这个元组就是(老元素, 拉进来的元素)
    // ? skip表示跳过第一项
    let sum: u32 = Counter::new()
        .zip(Counter::new().skip(1))
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();
    assert_eq!(18, sum);
}

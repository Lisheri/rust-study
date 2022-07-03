// * Rust标准库中包含一系列被称为集合(collection)的非常有用的数据结构。大部分其他数据类型都代表一个特定的值, 不过集合可以包含多个值。
// * 不同于内建的数组和元组类型, 这些集合指向的数据是存储在堆上的, 这意味着数据的数量不必在编译时就已知, 并且还可以随着程序的运行增长或缩小。
// * Rust中广泛使用的集合有三种:
// ? 1. vector 允许我们一个挨着一个地存储一系列数量可变的值(长度可变的数组)
// ? 2. 字符串(string)  是字符串的集合。 我们之前见过 String 类型
// ? 3. 哈希map (hash map) 允许我们将值与一个特定的键(key)相关联。这是一个叫做 map 的更通用的数据结构的特定实现。


// * 引入HashMap
pub mod use_vector {
  use std::collections::HashMap;
  // * 创建一个新的vector
  pub fn createVector() {
    // * 创建一个空的 vector 来存储 i32 类型的值
    // ? 这里需要插入一个类型注解, 因为没有向这个vector中插入值, rust并不知道我们想要存储什么类型的元素
    // * Vec是一个由标准库提供的类型, 它可以存放任何类型, 而当Vec存放某个特定类型时, 那个类型位于尖括号(泛型)中。
    let mut v: Vec<i32> = Vec::new();

    // * 在更实际的代码中，一旦插入值 Rust 就可以推断出想要存放的类型，所以你很少会需要这些类型注解。
    // * 更常见的做法是使用初始值来创建一个 Vec，而且为了方便 Rust 提供了 vec! 宏
    let v1 = vec![1, 2, 3];

    // * 向vector中新增元素, 当然需要增加成员, 也需要使用mut关键字声明为可变
    v.push(5);
    v.push(6);
    v.push(7);
    println!("当前 vec为: {:?}", v);

    // * 读取vector元素
    let v2 = vec![1,2,3,4,5];
    // ? 索引读取, 需要给一个引用, 否则会被回收
    let third: &i32 = &v2[2];
    println!("第三个成员是: {}", third);

    // ? get方法读取, get方法得到的是一个 Options<&T>
    match v2.get(2) {
      Some(third) => println!("The third element is {}", third),
      None => println!("没有第三个成员")
    }

    /* 
      * 需要注意的是, 首先使用索引值 2 来获取第三个元素, 索引是从 0 开始的。其次, 这两个不同的获取第三个元素的方式分别是: 使用 & 和 [] 返回一个引用; 或者使用 get 方法以索引作为参数来返回一个 Option<&T>

      * rust 有两个引用元素的方法的原因是程序可以选择如何处理当索引值在 vector 中没有对应值的情况。
    */

    // 这里看一个情况, 如果有一个五元素的vector接着尝试访问索引为100的元素时, 程序会如何处理
    let v3 = vec![1, 2, 3, 4, 5];
    
    // * 对于使用 [] 直接访问 index, 引用一个不存在的元素时, Rust 会造成 panic。这个方法更适合当程序认为尝试访问超过 vector 结尾的元素是一个严重错误的情况, 这时应该使程序崩溃。
    // let does_not_exit = &v3[100]; 
    // * 当 get 方法被传递了一个数组外的索引时, 他不会panic, 而是返回 None。偶尔出现超过 vector 范围的访问属于正常情况的时候, 可以考虑使用它。
    // * 代码中可以有一个处理 Some(&element) 或 None 的逻辑。毕竟索引的来源可能是用户输入的一个数字, 如果他们不慎输入了一个过大的数字, 那么程序就会得到 None值, 可以告诉用户当前 vector元素的数量并再请求他们输入一个有效的值, 这比因为输入错误而导致程序崩溃要有好很多
    let does_not_exit = v3.get(100);

    // * 一旦程序获取了一个有效的引用，借用检查器将会执行所有权和借用规则（第四章讲到）来确保 vector 内容的这个引用和任何其他引用保持有效。
    // * 这里就有一个问题, 当获取了一个vector的第一个元素的不可变引用并尝试在vector末尾增加一个元素时, 是行不通的, 如下
    let mut v4 = vec![1,2,3,4,5];
    // let first = &v4[0];
    // * 这里会报错
    // * 只是看起来能够运行, 因为我们印象中, 获取第一个元素, 并不会影响末尾元素增加
    // * 不能这么做的原因在于: vector的工作方式, 在vector末尾增加元素时, 在没有足够空间将所有元素依次相邻存放的情况下, 可能会要求分配新的内存并将老的元素拷贝到新的空间中。
    // * 此时第一个元素的引用, 就指向了被释放的内存。借用规则阻止程序陷入这样的情况。
    v4.push(6);
    // println!("第一个元素是: {}", first);

    // * 遍历vector 中的元素
    // 如果想要依次访问 vector 中的每一个元素, 我们可以遍历其所有的元素而无需通过索引一次一个的访问
    
    // ? 1. 遍历方式1： for循环
    for i in &v4 {
      println!("{}", i);
    }

    // ? 2. 遍历并改变
    let mut v5 = vec![100, 20, 50];
    for i in &mut v5 {
      // ? 为了修改可变引用指向的值, 在修改之前, 必须使用解引用运算符(*), 获取 i 中的值
      *i = *i + 50;
      println!("{}", i);
    }


    // * 同时, 离开作用域后, v 就回收了, 内部的成员也同时被回收
  }

  pub fn useEnumSaveValue() {
    // * 在开始的时候, 提到 vector 只能存储相同类型的值, 这是很不方便的; 绝对会有需要存储一系列不同类型的值的用例。不过, 枚举的成员都被定义为相同的枚举类型, 所以当需要在 vector 中存储不同类型值时, 我们可以定义并使用一个枚举。
    enum SpreadsheetCell {
      Int(i32),
      Float(f64),
      Text(String)
    }

    // * 这样这个vector中只有一个类型, 那就是SpreadsheetCell这个枚举
    let row = vec![
      SpreadsheetCell::Int(3),
      SpreadsheetCell::Float(10.2),
      SpreadsheetCell::Text(String::from("fuck"))
    ];

    /* 
      Rust 在编译时就必须准确的知道 vector 中类型的原因在于它需要知道储存每个元素到底需要多少内存。
      第二个好处是可以准确的知道这个 vector 中允许什么类型。
      如果Rust 允许 vector 存放任意类型, 那么当对 vector 元素执行操作时一个或多个类型的值就有可能会造成错误。
      使用枚举+match意味着Rust能在编译时就保证总是会处理所有可能的情况
    */

    // * 如果在编写程序时不能确切无遗地知道运行时会储存进 vector 的所有类型，枚举技术就行不通了。相反，你可以使用 trait 对象。
  }
  
  pub fn create_string() {
    // * 字符串是新晋Rustacean 们通常会被困住的领域, 这是由于三方面理由的结合: 
    // * 1. Rust 倾向于确保暴露出可能的错误
    // * 2. 字符串是比很多程序员所想象的要更为复杂的数据结构
    // * 3. rust的字符串使用的是 UTF-8 编码。
    // * 字符串就是作为字节的集合外加一些方法实现的，当这些字节被解释为文本时，这些方法提供了实用的功能。

    /* 
      * 字符串的核心:
      ? 1. Byte的集合
      ? 2. 提供了一些方法, 能将 byte解析为文本
    */

    // * Rust的核心语言中只有一种字符串类型: str, 字符串切片(slice), 它通常以被借用的形式出现(&str)
    // * 前面说到的 字符串slice: 他们是一些存储在别处的 UTF-8 编码字符串数据的引用。比如字符串字面值被存储在程序的二进制输出中, 也是一种字符串切片

    /* 
      String 类型:
        * 来自标准库, 而不是核心语言
        * 可增长, 可修改, 可拥有
        * 同样采用UTF-8编码格式

      ? 通常说的字符串指的是:
        * String 和 &str
          - 标准库里用的多
          - UTF-8编码
      
      * 在这里主要说String

      TODO 在Rust标准库中, 还包含了很多其他的字符串类型: OsString、 OsStr、 CString、 CStr

      * - String vs Str后缀: 通常以String结尾的字符串类型, 是指拥有所有权的, 而以Str结尾的类型, 通常是指可借用的
      * - 可存储不同编码的文本, 或者说在内存中以不同的形式展现

      TODO 某些Library crate 针对存储字符串可提供更多的选项(后续再说)
    */

    // * 创建一个新的祝福词(String)
    // * 很多 Vec<T>的操作都可用于String
    // ? 比如 String::new()函数, 可用于创建一个新的字符串
    let mut s = String::new();
    // ? 使用初始值来创建String 
    // - to_string() 方法, 可用于实现了 Display trait 的类型, 包括字符串字面值, 如下:
    // - String::from() 函数, 从字面值创建String
    // TODO 上面两种方法根据实际情况选择使用即可

    // * 这个data就是一个字符串字面值
    let data = "initial contents";
    // * 使用data上的一个to_string 方法, 将其转化为String这么要一个类型
    let s1 = data.to_string();
    // * 或者可以直接使用一个字符串字面值.to_string 获得一个String
    let s2 = "initial contents".to_string();

    // * to_string是第一种方法, 还有一种是String::from 函数
    let s3 = String::from("initial contents");

    // * 由于字符串是基于 UTF-8 编码的, 所以任何合法的数据, 都可以是一个字符串, 无论是日语、朝鲜语、德语、俄语亦或者是法语等, 都可以

    // ! 更新String
    // String是可以增减的, 内容也可以修改, 他的操作就像我们操作vector一样, 此外, 还可以对String进行拼接

    // ? 1. push_str() 方法: 把一个字符串切片, 附加到 String上
    let mut s4 = String::from("str");
    let s5 = String::from(" bar");
    s4.push_str(&s5);
    println!("s4 的值是:{}", s4);

    // ? 2. push方法, 将单个字符附加到String
    let mut s6 = String::from("lo");
    // * 注意, push方法只能添加单个字符(单引号, char类型), 但凡多一个都报错
    s6.push('l');
    println!("{}", s6);

    // ? +: 连接字符串
    // * - 使用了类似这个签名的方法 fn add(self, s: &str) -> String{...}, 这里是类似, 因为在标准库中, add方法是个泛型方法, 这里给搞成了具体的类型
    // * - 不过他确实是将一个String 和 一个字符串引用相连接
    // * - 当然, 这里第二个参数应该是一个字符串切片, 而不是字符串引用类型, 至于为什么能编译通过
    // * - 在调用add方法的时候, rust使用了 解引用强制转换(deref coercion), 把String的引用, 转换为了 字符串切片, 而第二个参数有引用标记, 在add函数内, 是一个借用, 所以他就被保留了, 但是add函数执行时, s7 的所有权被移动到了函数中, 所以当函数执行完毕后, s7就被回收了
    let s7 = String::from("hello, ");
    let s8 = String::from("world!");
    // * 注意: + 前面的变量是String类型, 但是 + 后面的变量, 要求是字符串切片类型(&str)
    // * 但实际上 + 后面的参数是 String类型的一个引用
    let s9 = s7 + &s8;
    // * 在此处如果打印s7, 其实会报错的, 因为s7, 在字符串拼接操作结束后, 就已经被回收了
    println!("{}, {}", s9, s8);

    // ? format!: 连接多个字符串(他带 ! 是个宏, 和println!类似, 使用宏能够更灵活的拼接)
    let s10 = format!("{}-{}", s8, s9);
    println!("{}", s10);

    // Rust的字符串不支持索引语法访问
    /* 
      ! String类型的内部表示:
      * String 是对 Vec<u8>的包装
        - len() 方法, 返回String所占的字节数
          * 实际上len方法统计字节数, 采用的是 unicode标量值
          * 每一个 unicode标量值, 都会对应两个字节
          * 但是下面的Hola返回的长度却是4, 也就是说, 一个字符串字节值的索引并不总是对应一个有效的 Unicode标量值
          * 比如 let hello = "Здравствуйте";
          * let answer = &hello[0];
          * 上面的3, 对应的unicode标量是 208,151, 如果索引可用, 那么answer 对应的就是 208, 但是这个208并没有任何意义, 但是他确实是索引0上的字节
          * 所以为了防止这种无异议的操作, 或者说防止出现这种难以发现的bug, rust语言禁止了索引值访问字符串
    */
    let len = String::from("Hola").len();
    println!("{}", len); // 4, 说明采用UTF-8编码后, 一个英文字符占用一个字节

    /* 
      ! 字节、标量值、字形簇
      Bytes,  Scalar Values, Grapheme Clusters
      * Rust 有三种看待字符串的方式:
        * - 字节
        * - 标量值
        * - 字形簇(这个字形簇才最接近所谓的"字母")
    */

    let w = "नमस्ते"; // 梵文书写的印度语单词
    /* for b in w.bytes() {
      * bytes方法返回字符串所有的字节
      * 这里一共有18个字节, 这些字节, 就是计算机存储这个字符串的方式
      println!("字符串字节: {}", b);
    } */

    // * 接下来使用unicode 标量值来看这个字符串
    // * chars 就是获取组成字符串的 unicode 标量值的方法
    // * 这样打印出来, 刚好只有6个, 但是都很奇怪, 并不是一个正常的数字或者字符, 并且他单独存在没有任何意义, 需要和两个字符组合, 才能得到对应的字符
    /*  for c in w.chars() {
      println!("unicode标量值: {}", c);
    } */

    // * 以字形簇的方式获取, 得到的就是通常的印度语字符, 但是从String里获得字形簇, 相对复杂一些, 因此标准库并没有提供此功能

    // ! 最后一个 Rust 不允许使用索引获取 String 字符的原因是: 索引操作预期总是需要常数时间 (O(1))。但是对于 String 不可能保证这样的性能，因为 Rust 必须从开头到索引位置遍历来确定有多少有效的字符。

    /* 
      ! 切割String(字符串slice)
      可以使用 [] 和一个范围, 来创建字符串的切片
    */

    let hello = "Здравствуйте";
    let s11 = &hello[0..4]; // 由于一个字母占两个字节, 所以这里, 0-4就是前两个字节: Зд
    // * 但是用索引0..3, 会panic, 因为索引3, 不是一个char边界
    // * 也就是说在切割的时候, 必须沿着char边界来切割, 否则就会panic（这个char边界就是说如果一个语言单个字符占两个字节, 那么切割的时候, 必须是 2, 4, 6, 否则就会出现半个字符这样的情况, 也就panic了, 因为不是字符边界）
    println!("{}", s11);

    // ! 遍历String
    // * 对于 标量值, 可以使用chars() 方法
    // * 对于字节值, 可以使用bytes() 方法
    // * 对于字形簇, 需要第三方库实现

    // ! 强调: String不简单
    // * Rust 选择将正确处理String数据作为所有Rust程序的默认行为
    //  - 程序员必须在处理 UTF-8 数据之前投入更多的精力
    // * 好处就是可以防止在开发后期处理涉及非ASCII字符的报错
  }

  pub fn create_hash_map() {
    /* 
      HashMap<K, V>
      * 以键值对的形式存储, 一个键(key) 对应一个值(Value)
      * Hash 函数: 决定如何在内存中存放 K 和 V
      * 适用场景: 通过 K (任何类型) 来寻找数据, 而不是通过索引
    */

    // 创建HashMap
    // 空 HashMap: new() 函数
    // 添加: insert()方法
    // let mut scores: HashMap<String, i32> = HashMap::new(); // * 如果是直接创建空HashMap, 则必须给类型, 否则Rust无法自动推断, 当然, 有了insert方法则不需要在给类型了

    // scores.insert(String::from("Blue", 10));

    /* 
      * HashMap 用的比较少, 因此不在 Prelude中
      * 标准库对其支持较少, 没有内置的宏来创建HashMap
      * 数据存储在 heap(堆)上
      * 他是同构的。在一个HashMap中: 所有的 K 必须是同一种类型, 所有的 V 也必须是同一种类型
    */

    // collect创建HashMap
    // 在元素类型为 Tuple 的 Vector上使用 collect 方法, 可以组件一个 HashMap:
    //  - 要求Tuple有两个值: 一个作为K, 一个作为V
    //  - collect方法可以把数据整合成很多种集合类型, 包括HashMap
    //  * · 返回值需要显示指明类型
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let intial_scores = vec![10, 50];
    // * 通过 iter() 方法返回 vector 的遍历器, 在通过zip方法, 就可以创建一个元组的数组
    // * 这个zip有拉链的意思, 所以很形象的就是像个拉链一样, 一拉就是一个元组的数组了
    // * 然后通过collect就可以创建一个HashMap
    // * 前面的类型不能去掉, 因为collect可以返回很多种不同的集合数据, 不一定是HashMap, 不指明的话, rust就不知道是什么类型的集合了, 但是K和V能推导, 因此就用下划线即可
    let scores: HashMap<_, _> = teams.iter().zip(intial_scores.iter()).collect();

    // ! 所有权
    /* 
      * 对于实现了 Copy trait 的类型(如i32), 值会被复制到 HashMap 中
      * 对于拥有所有权的值 (例如 String), 值会被移动, 所有权会转移给 HashMap
    */

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    // * 这个insert执行时, name 和 value 的所有权就移动给了insert函数, 当insert执行完后, 他们就会回收了 map.insert(field_name, field_value);
    // * 这是因为String类型的变量, 拥有所有权, 进入HashMap之后, 他们的所有权, 就移动到了 HashMap中
    // * 如果是引用入参, 插入进HashMap, 则值本身不会移动 map.insert(&field_name, &field_value);
    map.insert(&field_name, &field_value);

    // ! 访问
    // * get方法 get(K) -> Options<&V>;
    let team_name1 = String::from("Favorite color");
    let color = map.get(&team_name1);
    
    match color {
      Some(s) => println!("{}", s),
      None => println!("没有匹配到")
    }

    // ! 遍历
    let mut scores2 = HashMap::new();
    scores2.insert(String::from("Blue"), 10);
    // * insert 方法同名覆盖
    scores2.insert(String::from("Yellow"), 50);

    // * 这个小括号相当于元组
    for (k, v) in &scores2 {
      println!("key: {}, value: {}", k, v);
    }

    // ! 更新
    /* 
      * HashMap 大小可变
      * 每个 K 同时只能对应一个 V
      * 更新 HashMap中的数据:
      *   - K 已经存在, 对应一个 V
      *    · 替代现有V
      *    · 保留现有V, 忽略新的V
      *    · 合并现有V和新的V
      *  - K 不存在, 直接添加
    */

    // ? 插入的K相同, 但是V不同, 则直接替换
    // 比如: scores2.insert(String::from("Blue"), 10); scores2.insert(String::from("Blue"), 50); 之后 Blue 的值就成了50

    // ? 只在K不对应任何值的情况下, 才被插入V
    // * 需要使用 entry 方法: 检查指定的K是否对应一个V。
    // * entry(K: &K) -> enum Entry; Entry枚举代表值是否存在
    let mut score3 = HashMap::new();
    score3.insert(String::from("Blue"), 10);
    // * or_insert(V) 会在entry执行结果的枚举为 VacantEntry(K)时候, 执行 insert(K, V)
    // * entry 在存在时候, 返回的结果为 Entry(OccupiedEntry { key: K, value: V })
    score3.entry(String::from("Yellow")).or_insert(50);
    score3.entry(String::from("Blue")).or_insert(100);

    println!("{:?}", score3);
    println!("{:?}", score3.entry(String::from("Yellow")).or_insert(50));

    // * Entry 的 or_insert() 方法:
    // 返回结果为: 如果 K存在, 则返回到对应的V的一个可变引用; 如果K不存在, 将方法参数作为K的新值插进去, 返回到这个值的可变引用
    // * 也就是说 or_insert() 返回的都是一个可变引用

    // ! 基于现有K更新
    let text = "hello world wonderful world";
    let mut map_text = HashMap::new();
    // * String.split_whitespace表示将字符串按 空格字符 分割
    for word in text.split_whitespace() {
      let count = map_text.entry(word).or_insert(0);
      // * 由于or_insert 返回的是一个可变引用, 因此需要解引用
      // * 还是一样的 在加法操作时, 需要使用 * 解引用
      *count += 1;
    }

    println!("{:#?}", map_text);

    // ! Hash函数
    // * 默认情况下, HashMap使用加密功能强大的Hash函数, 可以抵抗拒绝服务(Dos)攻击
    // - 不是可用的最快的Hash算法
    // - 但具有更好的安全性
    // * 可以指定不同的 hasher 来切换到另一个函数
  }
}



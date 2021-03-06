// ! library crate
use std::error::Error;
// * 处理和文件相关的事务
use std::fs;
// * 使用环境变量
use std::env;

// * 返回一个Result, 如果正确, 就返回空元祖, 否则返回一个实现了 Error trait的类型
// ? dyn 表示动态
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // * 读取可能会发生错误, 而read_to_String这个方法返回的是 Result枚举, 所以可以使用expect接收
    // ? 这里使用?操作符, 就是会产生一个错误, 返回给调用者, 也就是Result.Err
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }
    return Ok(());
}

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    // * 用于解析配置, 返回一个struct
    // * 改用Result枚举, 暴露错误信息, 错误信息使用 static生命周期即可
    // pub fn new(args: &[String]) -> Result<Config, &'static str> {
    // * 这里修改入参类型, 使用标准库下的env下的Args迭代器, 因为迭代器在迭代过程中会修改自身状态
    // * 此时的 new 函数就获得了args的所有权
    pub fn new(mut args: str::env::Args) -> Result<Config, &'static str> {
        if args.len() < 3 {
            // * 针对这种使用的问题, 其实我们可以返回一个 result枚举, 而不是使用panic, panic一般是程序本身的错误才使用
            // panic!("参数不足, 请检查参数, 此处至少需要两个参数!")
            return Err("参数不足, 请检查参数, 此处至少需要两个参数!");
        }
        // args现在是一个迭代器, 但是第一个元素没什么用, 因此需要调用一下next, 过滤掉
        args.next();
        // * 因为这个切片没有实现 Copy这个 trait, 不能复制, 所以这里采用简单但是比较低效的克隆方法
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Did not get a query string")
        }; // * 也就是第二个元素的引用
        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Did not get a file name")
        }; // * 文件名
        /*
            * 其实这里可以不用克隆
            * 这里之所以要克隆, 是因为这个new函数并不是要args的所有权, 传进来的只是一个string的切片
            * 为了返回一个Config, Config里面需要有下面哪些字段的所有权
            * 所以上面针对query、filename, 才必须将args的第二和第三个参数克隆一份
            * 但其实这里可以不用这样搞, 可以在new函数里面直接使用迭代器获得他的所有权
            * 还可以使用迭代器附带的功能进行长度检查和索引
        */
        // ? var里面放的就是环境变量的名称, 它只要出现, 就表示区分大小写, 否则表示不区分大小写
        // ? var函数结果是一个Result, 如果环境变量被设置, 那么结果就包裹在Ok中, 否则返回Err变体
        // ? 这里只需要通过is_err, 检查是否发生错误, 如果他出现, 那就一定不会发生错误
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        // println!("Search for {}", query);
        // println!("In file {}", filename);
        // * 要解引用
        return Ok(Config { query, filename, case_sensitive });
    }
}

// * 这里要声明一个生命周期, 因为返回值的vector是从contents中获取的字符串切片, 所以这个生命周期给 contents即可
// * 只有当切片引用的数据是有效的, 切片本身才是有效的
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    /*
        let mut results = Vec::new();
        // * lines()会返回一个迭代器
        for line in contents.lines() {
            if line.contains(query) {
                results.push(line);
            }
        }
        return results;
    */
    // * 其实这里使用filter就可以了
    // * contents.lines()返回的是一个迭代器, 过滤完成后再调用collect返回一个集合
    return contents.lines().filter(|line| {
        return line.contains(query);
    }).collect();
    // ? 有人会说手动写循环效率更高, 而是用迭代器不停的转换, 会导致效率变低, 真的是这样吗?
    // ? 这个问题见13章最后一小节
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    /*
        let mut results = Vec::new();
        // * lines()会返回一个迭代器
        // * 这个 to_lowercase 会创建一个新的字符串, 因此query此时拥有所有权
        let query = query.to_lowercase();
        for line in contents.lines() {
            if line.to_lowercase().contains(&query) {
                results.push(line);
            }
        }
        return results;
    */
    return contents.lines().filter(|line| {
        return line.to_lowercase().contains(query);
    }).collect();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents))
    }

    // * 区分大小写
    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents))
    }

    // * 不区分大小写
    #[test]
    fn case_insensitive() {
        let query = "rUst";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, contents))
    }
}

/*
  !使用 TDD(测试驱动开发) 来开发库功能
  先编写测试, 在编写能够通过测试的代码, 也有助于在开发过程中保持较高的开发覆盖率, 而我们将使用TDD实现具体的搜索功能
  + 搜索: 在文件内容中搜索指定的字符串, 并生成一个包含所有匹配行的列表
  + TDD(Test-Driven Development)
    - 编写一个会失败的测试, 运行该测试, 确保它是按照预期的原因失败
    - 编写或修改刚好足够的代码, 让新测试通过
    - 重构刚刚添加或修改的代码, 确保测试会始终通过
    - 返回步骤1, 继续
*/

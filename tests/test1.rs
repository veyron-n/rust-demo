/**
 * 格式化输出
 */
#[cfg(test)]
mod demo1 {
    // 推导 `Structure` 的 `fmt::Debug` 实现
    // `Structure` 是一个包含单个 `i32` 的结构体
    #[derive(Debug)]
    struct Structure(i32);

    // 将 `Structure` 放到结构体 `Deep` 中, 然后使 `Deep` 也能够打印
    #[derive(Debug)]
    struct Deep(Structure);

    #[allow(dead_code)]
    #[derive(Debug)]
    struct Person<'a> {
        name: &'a str,
        age: u8
    }
   
    /**
     * 调试 (debug) 指令 cargo test demo1::test1 --  --nocapture
     */
    #[test]
    fn test1() {
        println!("{:?} months in a year.", 12);
        println!("{1:?} {0:?} is the {actor:?} name.",
                "Slater",
                "Christian",
                actor="actor's");
        
        // `Structure` 也可以打印!
        println!("Now {:?} will print!", Structure(3));

        // 使用 `derive` 的一个问题是不能控制输出的形式
        // 假如我只想展示一个 `7` 怎么办?
        println!("Now {:?} will print!", Deep(Structure(7)));
        // 所以使用fmt::Debug确实使这些内容可以打印, 但是牺牲了一些美感, Rust也通过 {:#?} 提供了"美化打印"的功能
        
        let name = "Peter";
        let age = 23;
        let peter = Person{ name, age };
        println!("{:#?}", peter);
    }

    /**
     * 显示 (display) 指令 cargo test demo1::test2 --  --nocapture
     */
    #[test]
    fn test2() {
        /**
         *  fmt::Debug 通常看起来不太简洁, 因此自定义输出的外观经常更可取的, 这需要通过手动实现
         *  fmt::Display 来做到 fmt::Display 采用 {} 标记, 实现方式看起来像这样:
         */
        // (使用 `use`) 导入 `fmt` 模块使 `fmt::Display` 可用
        use std::fmt;

        // 定义一个结构体, 咱们会为它实现 `fmt::Display` 以下是个简单的元组结构体
        // `Structure`, 包含一个 `i32` 元素
        struct Structure(i32);

        //为了使用 {} 标记, 必须手动为类型实现 `fmt::Display` trait
        impl fmt::Display for Structure {
            // 这个 trait 要求 `fmt` 使用与下面的函数完全一致的函数签名
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                // 仅将 self 的第一个元素写入到给定的输出流 `f` 返回 `fmt::Result`
                // 此结果表明操作成功或失败, 注意 `write!` 的用法和 `println! 很相似
                write!(f, "{}", self.0)
            }
        }
        /**
         * fmt::Display 的效果可能比 fmt::Debug 简洁, 但对于std 库来说, 这就有一个问题, 模棱两可的
         * 类型如何显示呢? 举个例子, 假设标准库对所有的Vec<T> 都实现了一种输出样式, 那么它应该使哪一种
         * 样式? 下面两种的一种吗?
         * Vec<path>: /:/etc:/home/username:/bin (使用 : 分割)
         * Vec<number> : 1,2,3 (使用 , 分割)
         * 我们没有这样做, 因为没有一种合适的样式使用与所有类型, 标准库也并不擅自规定一种样式, 对于
         * Vec<T> 或其他任意泛型容器(generic container) , fmt::Display都没有实现, 因此在这些泛型的情况下要用fmt::Debug
         * 这并不是一个问题, 因为对于任何非泛型的容器类型, fmt::Display 都能够实现
         */

        // 带有两个数字的结构体, 推导出 `Debug`, 以便于 `Display` 的输出进行比较
        #[derive(Debug)]
        struct MinMax(i64, i64);

        // 实现 `MinMax` 的 `Display`
        impl fmt::Display for MinMax {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                // 使用 `self.number` 来表示各个数据
                write!(f, "({}, {})", self.0, self.1)
            }
        }
        // 为了比较, 定义一个具有字段的结构体
        #[derive(Debug)]
        struct Point2D {
            x: f64,
            y: f64,
        }
        // 类似的对 `Point2D` 实现 `Display`
        impl fmt::Display for Point2D {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                // 自定义格式, 使得仅显示 `x` 和 `y` 的值
                write!(f, "x: {}, y: {}", self.x, self.y)
            }
        }

        let minmax = MinMax(0, 14);
        println!("Compare structures:");
        println!("Display: {}", minmax);
        println!("Debug: {:?}", minmax);

        let big_range = MinMax(-300, 300);
        let small_range = MinMax(-3,3);

        println!("The big range is {big} and the small is {small}",
                    small = small_range,
                    big = big_range);

        let point = Point2D { x: 3.3, y: 7.2 };

        println!("Compare point:");
        println!("Display: {}", point);
        println!("Debug: {:?}", point);

        // 报错, `Debug` 和 `Display` 都被实现了, 但 `{:b}` 需要`fmt::Binary`
        // 得到实现, 这句不能运行
        // eprintln!("What does Point2D look like in binary: {:b}?", point);

        #[derive(Debug)]
        struct Complex {
            real: f32,
            imag: f32,
        }
        impl fmt::Display for Complex {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                // 使用 `self.number` 来表示各个数据
                write!(f, "real: {}, imag: {}i", self.real, self.imag)
            }
        }
        let complex = Complex{ real: 3.3, imag: 7.2 };
        println!("Display: {}", complex);
        println!("Debug: Complex {:?}", complex)
    }

    /**
     * 测试实例: List 指令 cargo test demo1::test3 --  --nocapture
     */
    #[test]
    fn test3() {
        /*
         * 对一个结构体实现 fmt::Display, 其中的元素需要一个接一个地处理到, 这可能会很麻烦, 问题在于每个write!
         * 都要生成一个fmt::Result, 正确的实现需要处理所有的Result, Rust专门为解决这个问题提供了 ? 操作符
         * 在write!上使用 ? 会像是这样:
         * 对 `write!` 进行尝试 (try), 观察是否出错, 若发生错误, 返回相应的错误, 否则(没有出错) 继续执行后面的语句
         * write!(f, "{}", value)?;
         * 另外, 你也可以使用 try! 宏, 它和 ? 是一样的, 这种写法比较啰嗦, 故不再推荐, 但在老一些的Rust代码中仍会看到
         * try!(write!(f, "{}", value));
         */
        // 有了 ? 对一个 Vec 实现 fmt::Display 就很简单了:
        use std::fmt; // 导入fmt模块
        // 定义一个包含单个 `Vec` 的结构体 `List`
        struct List(Vec<i32>);
        impl fmt::Display for List {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                // 使用元组的下标获取值, 并创建一个 `vec` 的引用
                let vec = &self.0;
                write!(f, "[")?;
                // 使用 `v` 对 `vec` 进行迭代, 并用 `count` 记录迭代次数
                for (count, v) in vec.iter().enumerate() {
                    // 对每个元素 (第一个元素除外) 加上逗号
                    // 使用 `?` 或 `try!` 来返回错误
                    if count != 0 { write!(f, ", ")?; }
                    write!(f, "{}", v)?;
                }

                // 加上配对中括号, 并返回一个 fmt::Result 值
                write!(f, "]")
            }
        }
        let v = List(vec![1,2,3]);
        println!("{}", v)

        /*
         * 更改程序使vector里面的每一个元素的下标也能够打印出来, 新的结果如下:
         * [0: 1, 1: 2, 2: 3]
         */

    }
}
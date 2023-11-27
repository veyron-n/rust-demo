/**
 * 原生类型
 */
#[cfg(test)]
mod primitive_type {
    /**
     * 字面量和运算符 测试命令: cargo test primitive_type::test1 --  --nocapture
     * 整数 1, 浮点数 1.2, 字符 'a', 字符串 "abc", 布尔值 true 和单元类型() 可以用数字, 文字或者符号之类的 "字面量(literal)来表示"
     * 另外, 通过加前缀 0x, 0o, 0b, 数字也可以用十六进制, 八进制或者二进制记法表示
     * 为了改善可读性, 可以在数值字面量中插入下划线, 比如: 1_000 等于 1000, 0.000_001等于 0.000001
     * 我们需要把字面量的类型告诉编译器, 如前面讲过, 我们使用 u32 后缀来表示字面量是一个32位无符号整数, i32后缀表明字面量是一个32位有符号整数
     * Rust提供了一系列的运算符(operator), 它们的优先级和类C语言类似 ps:类C语言包括C/C++, Java, PHP等语言
     */
    #[test]
    fn test1() {
        // 整数相加
        println!("1 + 2 = {}", 1u32 + 2);

        // 整数相减
        println!("1 - 2 = {}", 1i32 - 2);
        // 试一试 ^ 尝试将 `1i32` 改为 `1u32`, 体会为什么类型声明这么重要
        // println!("1 - 2 = {}", 1u32 - 2); 报错attempt to compute `1_u32 - 2_u32`, which would overflow

        // 短路求值的布尔逻辑
        println!("true AND false is {}", true && false);
        println!("true OR false is {}", true || false);
        println!("NOT true is {}", !true);

        // 位运算
        println!("0011 AND 0101 IS {:04b}", 0b0011u32 & 0b0101); // 0011 AND 0101 IS 0001
        println!("0011 OR 0101 IS {:04b}", 0b0011u32 | 0b0101);  // 0011 OR 0101 IS 0111
        println!("0011 XOR 0101 IS {:04b}", 0b0011u32 ^ 0b0101); // 0011 XOR 0101 IS 0110
        println!("1 << 2 is {}", 1u32 << 2);                     // 1 << 2 is 4
        println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);           // 0x80 >> 2 is 0x20

        // 使用下划线改善数字的可读性!
        println!("One million is written as {}", 1_000_000u32);
    }
    /**
     * 元组 测试命令: cargo test primitive_type::test2 --  --nocapture
     * 元组是一个可以包含各种类型值的组合, 元组使用括号()来构造(construct), 而每个元组自身又是一个类型标记位(T1,T2,...)的值
     * 其中T1,T2是每个元素的类型, 函数可以使用元组来返回多个值, 因为元组可以拥有任意多个值
     */
    #[test]
    fn test2() {
        // 元组可以充当函数的参数和返回值
        fn reverse(pair: (i32, bool)) -> (bool, i32) {
            // 可以使用 `let` 把一个元组的成员绑定到一些变量
            let (integer, boolean) = pair;
            (boolean, integer)
        }
        // 在 "练习" 中要用到下面这个结构体
        #[derive(Debug)]
        struct Matrix(f32, f32, f32, f32);
        // 包含各种不同类型的元组
        let long_tuple = 
        (
            1u8, 2u16, 3u32, 4u64,
            -1i8, -2i16, -3i32, -4i64,
            0.1f32, 0.2f64, 'a', true
        );
        // 通过元组的下标来访问具体的值
        println!("long tuple first value: {}", long_tuple.0);
        println!("long tuple second value: {}", long_tuple.1);

        // 元组也可以充当元组的元素
        let tuple_of_tuples = ((1u8, 2u16, 2u32),(4u64, -1i8), -2i16);

        // 元组可以打印
        println!("tuple of tuples: {:?}", tuple_of_tuples);

        // 但很长的元组无法打印
        // let too_long_tuple = (1,2,3,4,5,6,7,8,9,10,11,12,13);
        // println!("too long tuple: {:?}", too_long_tuple);
        let pair = (1, true);
        println!("pair is {:?}", pair);

        println!("the reversed pair is {:?}", reverse(pair));
        // 创建单元素元组需要一个额外的逗号, 这是为了和被括号包含的字面量作区分
        println!("one element tuple: {:?}", (5u32,));
        println!("just an integer: {:?}", (5u32));

        // 元组可以被解构(deconstruct), 从而将值绑定给变量
        let tuple = (1, "hello", 4.5, true);

        let (a,b,c,d) = tuple;
        println!("{:?}, {:?}, {:?}, {:?}", a,b,c,d);
        
        let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
        println!("{:?}", matrix);

        /*
         * 练习: 1. 在上面例子中给Matrix结构体加上fmt::Display trait, 这样当你从Debug格式化{:?}切换到Display格式化{}时， 会得到如下输出：
         * ( 1.1 1.2 )
         * ( 2.1 2.2 ) 
         */
        use std::fmt;
        impl fmt::Display for Matrix {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "({} {})\n({} {})", self.0, self.1, self.2, self.3)
            }
        }
        // 创建 Matrix 实例
        let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
        println!("{}", matrix);
        /*
         * 练习: 2. 以reverse函数作为样板, 写一个 transpose 函数， 它可以接收一个Matrix 作为参数， 并返回一个右上-左下对角线上的两元素交换后的Matrix, 举个例子：
         * Matrix:
         * ( 1.1 1.2 )
         * ( 2.1 2.2 )
         * Transpose:
         * ( 1.1 2.1 )
         * ( 1.2 2.2 )
         */
        fn transpose(matrix: Matrix) -> Matrix {
            Matrix(matrix.0, matrix.2, matrix.1, matrix.3)
        }
        // 打印原始矩阵
        println!("Matrix:\n{}", matrix);

        // 使用 transpose 函数获取交换后的矩阵并打印
        let transposed_matrix = transpose(matrix);
        println!("Transpose:\n{}", transposed_matrix);
    }
}
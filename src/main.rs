// mod factorial;
// mod issues;
// mod threads;

fn main() {
    // println!("Hello, world!");
    greet_world();
    println!("===========================");
    test();
    println!("===========================");
    penguin();
    println!("===========================");
    add_one();
    println!("===========================");
    adds();
    println!("===========================");
    // add_one_to_number();
    mutable();
    println!("===========================");
    basic_type();
    println!("===========================");
    complex_num();
    println!("===========================");
    char();
    println!("===========================");
    expression_fuc();
    println!("===========================");
    renference_and_dereference();
    println!("===========================");
    // println!("{}", factorial::factorial(5));
    // println!("{}", issues::issues());
    // println!("{}", threads::threads());
    compound_types();
    println!("===========================");
    struct_operations();
    println!("===========================");
    array_operations();
    println!("===========================");
    flow_control();
    println!("===========================");
    match_control();
    println!("===========================");
}

fn greet_world() {
    let southern_germany = "Grüß Gott! 😊";
    let chinese = "世界，你好 😊";
    let english = "World, Hello 😊";
    let regions = [southern_germany, chinese, english];

    for regions in regions.iter() {
        println!("{}", regions)
        // println!("{}", &regions)
    }
}

// Rust 原生支持 UTF-8 编码的字符串
// println 后面的 !, 在 Rust 中，这是 宏 操作符，你目前可以认为宏是一种特殊类型函数
// println 没有使用 %s,%d 来做输出占位符，而是使用 {},因为 Rust 会自动识别输出数据的类型
// Rust 的集合类型不能直接进行循环，需要变成迭代器(这里是通过 .iter() 方法)，才能用于迭代循环
// 在 2021 edition 及以后, 支持直接写 for region in regions，原因是因为 for 隐式地将 regions 转换成迭代器

fn penguin() {
    let penguin_data = "\
    common name,length（cm)
    little penguin,33
    Yellow-eyed penguin,65
    Fiord-land penguin,60
    invalid,data
    ";
    let records = penguin_data.lines();
    for (i, record) in records.enumerate() {
        if i == 0 || record.trim().len() == 0 {
            continue;
        }

        // 声明一个fields变量，类型是Vec
        // Vec是vector的缩写，是一个可伸缩的集合类型，可以认为是一个动态数组
        // <_>表示Vec中的元素类型由编译器自行推断，在很多场景下，都会帮我们省却不少功夫
        let fields: Vec<_> = record.split(',').map(|field| field.trim()).collect();
        if cfg!(debug_assertions) {
            // 输出到标准错误输出
            eprintln!("debug: {:?} -> {:?}", record, fields);
        }

        let name = fields[0];
        // 1. 尝试把fields[1]的值转换为f32类型的浮点数，如果成功，则把f32值赋给length变量
        // 2. if let是一个匹配表达式，用来从=右边的结果中，匹配出length的值:
        // 当=右边的表达式执行成功，则会返回一个Ok(f32)的类型，若失败，则会返回一个Err(e)类型，if let的作用就是仅匹配Ok也就是成功的情
        // 况,如果是错误，就直接忽略，同时if let还会做一次解构匹配，通过Ok(length)去匹配右边的Ok(f32)，最终把相应的f32值赋给length
        // 3. 当然你也可以忽视成功的情况，用if let Err(e) = fields[1].parse::<f32>() {...}匹配出错误，然后打印出来，但是没啥卵用
        if let Ok(length) = fields[1].parse::<f32>() {
            // 输出到标准输出
            println!("{}, {}cm", name, length);
        }
    }
}

fn add_one() {
    let x = 5;
    let y = 10;
    let sum = x + y;
    println!("The sum of {} and {} is {}", x, y, sum);
}

// 在 Rust 中，可以使用 if let 语句来匹配出错误，如果匹配成功，则把错误赋给变量 e，然后打印出来
// Rust 程序入口函数必须是 fn main()，这样才能被编译器识别为主函数，该函数目前无返回值，也无参数
// 用let来声明变量，变量的类型由编译器自动推断 let a = 10，也可以主动指定类型 let b: i32 = 20，如果变量的类型是一个结构体，则可以使用 { } 来解构
// 可以在数值中带上类型:30i32表示数值是30，类型是i32
// let mut c = 30i32 表示c是一个可变的整数，类型是i32, mut是mutable的缩写表示可变的
// let d = 30_i32 还能在数值和类型中间添加一个下划线，让可读性更好，但是不能被编译器识别，所以不能在数值中使用下划线，但是可以在类型中使用下划线
// 可以使用函数返回值来接收函数的返回值，如果函数返回值是一个结构体，则可以使用 { } 来解构
// 可以使用函数返回值来作为函数的参数，如果函数返回值是一个结构体，则可以使用 { } 来解构
// println! 是一个宏，它的作用是输出一个字符串，它的第一个参数是一个字符串，它的第二个参数是一个表达式，表达式的值会被输出到字符串中， 它看起来像是函数但是它返回的是宏定义的代码块
// 宏定义的代码块可以包含多行代码，它们会被编译器忽略，它们只是为了提高编译器的性能，它们不会被编译器解析
// { } 是一个表达式或占位符，它的值是一个表达式的值，它的类型是表达式的类型
// 字符串使用双引号 "" 而不是单引号 ''，Rust 中单引号是留给单个字符类型（char）使用的
// Rust 使用 {} 来作为格式化输出占位符，其它语言可能使用的是 %s，%d，%p 等，由于 println! 会自动推导出具体的类型, 因此无需手动指定

fn adds() {
    let a = 10;
    let b: i32 = 20;
    // 通过添加到宏生成的代码来抑制警告：
    #[allow(unused_mut)]
    let mut c = 30i32;
    let d = 30_i32;
    let e = add(add(a, b), add(c, d));
    println!("(a+b)+(c+d) = {}", e);
}

fn add(i: i32, j: i32) -> i32 {
    // 返回相加值，这里可以忽略return，不要添加；号
    i + j
}

fn mutable() {
    // 可变变量，值可以在函数中改变，但是类型不能改变
    let mut a = 10;
    println!("a = {}", a);
    a = 20;
    println!("a = {}", a);

    // 变量遮蔽(shadowing)
    let b = String::from("hello");
    println!("b = {}", b);
    let (a, mut b): (bool, bool) = (true, false);
    // a = true,不可变; b = false，可变
    println!("a = {:?}, b = {:?}", a, b);

    b = true;
    assert_eq!(a, b);

    // Rust 常量的命名约定是全部字母都使用大写，并使用下划线分隔单词，另外对数字字面量可插入下划线以提高可读性
    const MAX_POINTS: u32 = 100_000;
    println!("MAX_POINTS = {}", MAX_POINTS);

    let x = 5;
    // 在函数的作用域内对之前的x进行遮蔽
    // 在被遮蔽后，无法再访问到之前的同名变量
    let x = x + 1;
    {
        // 在当前的花括号作用域内，对之前的x进行遮蔽
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }
    // println!("x = {}", x);
    println!("The value of x is: {}", x);

    // 字符串类型
    let spaces = "   ";
    // usize数值类型
    let spaces = spaces.len();
    println!("spaces = {}", spaces);
    // let mut spaces = "   ";
    // spaces = spaces.len();
    // 前后spaces的类型是不同的，会报错

    // 变量的生命周期
    // 前后两个变量的生命周期不一致，会报错， rust对类型要求严格
    // quote! 宏，用于生成字符串
}

fn basic_type() {
    // 基本类型和复合类型
    // 数值类型: 有符号整数integer (i8, i16, i32, i64, isize)、 无符号整数unsigned (u8, u16, u32, u64, usize) 、浮点数 (f32, f64)、以及有理数、复数
    // 字符串：字符串字面量和字符串切片 &str
    // 布尔类型： true和false
    // 字符类型: 表示单个Unicode字符，存储为4个字节
    // 单元类型: 即 () ，其唯一的值也是 ()

    // 数组：数组是一种有序的集合，其中的元素可以是不同类型的
    // 元组：元组是一种有序的集合，其中的元素可以是不同类型的
    // 枚举：枚举是一种构造的类型，其中的元素可以是不同类型的
    // 注意：枚举和元组的区别在于，枚举是构造的类型，元组是值类型
    // 元组的元素可以是不同类型的，枚举的元素必须是相同类型的

    // 基本数据类型：bool，char，i8，u8，i16，u16，i32，u32，i64，u64，i128，u128，isize，usize，f32，f64
    // 基本数据类型的默认值是0，bool默认值是false
    let a = 10;
    let b = true;
    let c = 'c';
    let d = "hello";
    let e = [1, 2, 3];
    let f = vec![1, 2, 3];
    let g = d.len() as i32;
    let h = d.len();
    println!(
        "a = {}, b = {}, c = {}, d = {}, e = {:?}, f = {:?}, g = {:?}, h = {:?}",
        a, b, c, d, e, f, g, h
    );

    let guess: u32 = "42".parse().expect("Not a number!");
    // let guess = "42".parse::<u32>().expect("Not a number!");
    println!("guess = {}", guess);

    // f32 类型是单精度浮点型，f64 为双精度浮点型
    let m = 2.2;
    let n: f32 = 3.3;
    println!("m = {}, n = {}", m, n);

    let sum = m + m;
    let difference = 96.2 - 4.3;
    let product = 2 * 3;
    let quotient = 56.7 / 32.2;
    let remainder = 43 % 5;

    println!(
        "sum = {}, difference = {:.2}, product = {}, quotient = {}, remainder = {}",
        sum, difference, product, quotient, remainder
    );

    // assert!(0.1 + 0.2 == 0.3);
    // assert!((0.1 + 0.2 - 0.3) < 0.1);
    println!("{}", 0.1 + 0.2 == 0.3); // false
    println!("{}", (0.1 + 0.2 - 0.3) < 0.1); // true
    println!("{}", (0.1 + 0.2 - 0.3) < 0.2); // false
    println!("{}", (0.1 + 0.2 - 0.3)); // 0.00000000000000005551115123125783
    println!("{}", 0.1 + 0.2); // 0.30000000000000004
    println!("{}", (0.1 + 0.2 - 0.3) < 0.00001); // true
    println!("{}", (0.1 + 0.2 - 0.3) < 0.00002); // false
    println!("{}", (0.1_f64 + 0.2 - 0.3).abs() < 0.00001); //true

    // ======================
    let abc: (f32, f32, f32) = (0.1, 0.2, 0.3);
    let xyz: (f64, f64, f64) = (0.1, 0.2, 0.3);

    println!("{:?}", abc);
    println!("{:?}", xyz);
    println!("{:?}", abc.0);
    println!("abc (f32)");
    println!("   0.1 + 0.2: {:x}", (abc.0 + abc.1).to_bits());
    println!("         0.3: {:x}", (abc.2).to_bits());

    println!("xyz (f64)");
    println!("   0.1 + 0.2: {:x}", (xyz.0 + xyz.1).to_bits());
    println!("         0.3: {:x}", (xyz.2).to_bits());

    assert!(abc.0 + abc.1 == abc.2);
    assert!(xyz.0 + xyz.1 != xyz.2);

    println!("{}", -42.0_f64.sqrt());
    println!("{}", -42.0_f32.sqrt());
    println!("{}", (-42.0_f32).sqrt());
    let _x = (-42.0_f32).sqrt();
    // assert_eq!(_x, _x) // NaN不能用来比较
    // 出于防御性编程的考虑，可以使用 is_nan() 等方法，可以用来判断一个数值是否是 NaN ：
    if _x.is_nan() {
        println!("未定义的数学行为{}", "NaN");
    }
    // assert!(!_x.is_nan()); // 可以用来判断一个数值是否是 NaN ：
    // assert!(!_x.is_infinite()); // 无穷大
    // assert!(!_x.is_finite()); // 有限的，只有 NaN 或者 Infinity 才是无穷大的
    // assert!(!_x.is_normal()); // 正常数
    // assert!(_x.is_sign_positive()); // 正数
    // assert!(_x.is_sign_negative()); // 负数

    // Range(序列)
    // 序列只允许用于数字或字符类型，原因是：它们可以连续的, 同时编译器在编译期可以检查该序列是否为空，字符和数字值是 Rust 中仅有的可以用于判断是否为空的类型
    println!("{:?}", 1..5); // 1..5 生成从 1 到 4 的连续数字序列
    println!("{:?}", 1..=5); // 1..=5 生成从 1 到 5 的连续数字序列
    for i in 1..=5 {
        println!("{}", i);
    }
    for i in 'a'..'f' {
        println!("{}", i);
    }
    // 有理数和复数
    // 任意大小的整数和任意精度的浮点数
    // 固定精度的十进制小数，常用于货币相关的场景
    // 实数和复数
    // 实数和复数是 Rust 中的两个基本类型，它们都是浮点数，但是它们的精度不同，实数是有符号的，而复数是无符号的。
    // 实数和复数的精度取决于它们的类型，实数的精度是 32 位，而复数的精度是 64 位。

    // 类型转换必须是显式的. Rust 永远也不会偷偷把你的 16bit 整数转换成 32bit 整数，因为这样做会导致精度丢失。
}

use num::Complex;

fn complex_num() {
    let a = Complex { re: 2.1, im: -1.2 };
    let b = Complex::new(11.1, 22.2);
    let result = a + b;
    println!("{} + {}i = {}", result.re, result.im, result);
}

fn char() {
    // 字符、布尔、单元类型
    // 字符是一种类型，它是一个 Unicode 字符，它的值是一个 32 位的整数。
    // 字符可以是英文字母，中文汉字
    // 字符可以是数字，但是不能是数字字符
    // 字符可以是标点符号，但是不能是标点符号字符
    // 字符可以是空格，但是不能是空格字符
    let c = 'c';
    let d = '\u{20BB7}';
    let e = '\u{00A9}';
    let f = '\u{1F680}';
    let g = '中';
    let h = '😍';
    let i = 'ℤ';
    // let j = &g;

    println!(
        "{} {} {} {} {} {} {} {} {} {}",
        c, d, e, f, g, h, i, c as u32, d as u32, e as u32
    );
    // println!("{}", j);
    println!(
        "字符'中'占用了 {} 字节的内存大小",
        std::mem::size_of_val(&g)
    );

    // 布尔值(bool)
    // 布尔值是 Rust 中的一个基本类型，它只有两个值：true 和 false。
    // 布尔值可以用来表示逻辑值，也可以用来表示状态值。
    // 布尔值可以用来表示空值，也可以用来表示非空值。
    // 布尔值可以用来表示错误值，也可以用来表示非错误值。
    // 布尔值可以用来表示可变性，也可以用来表示非可变性。
    // 布尔值可以用来表示可空性，也可以用来表示非可空性。
    // 布尔值可以用来表示可达性，也可以用来表示非可达性。
    let t = true;
    let f: bool = false; // 使用类型标注，显式指定类型
    if t && f {
        println!("这是段毫无意义的代码")
    }

    // 单元类型就是 () ，唯一的值也是 () ，它是一个空元组。
    // 可以作为map的value
    // 可以作为一个值用来占位，但是完全不占用任何内存
}

// 函数表达式是一种特殊的函数，它的类型是它被调用时所返回的类型。
// 函数表达式可以用来创建一个函数，也可以用来调用一个函数。
// main函数是一个函数表达式，它的类型是()->()，也就是说它返回一个函数，这个函数没有参数，返回值也是()。
// 表达式不能包含分号，一旦在表达式后加上分号，它就会变成一条语句，再也不会返回一个值
fn expression_fuc() {
    (|x: i32, y: i32| x + y)(1, 2);
    #[allow(unused_variables)]
    let a = 8;
    let b: Vec<f64> = Vec::new();
    let (a, c) = ("hi", false);
    println!("{} {:?} {}", a, b, c);
    let expr = add_with_extra(10, 20);
    println!("{}", expr);

    let x = plus_or_substract(10);
    println!("{}", x);
    let x = plus_or_minus(10, 20);
    println!("{}", x);
    report(10);
    // dead_end();
    ownership();
}

// 函数名和变量名使用蛇形命名法(snake case)，例如 fn add_two() -> {}
// 函数的位置可以随便放，Rust 不关心我们在哪里定义了函数，只要有定义即可
// 每个函数参数都需要标注类型
// Rust 的函数体是由一系列语句组成，最后由一个表达式来返回值，例如：
fn add_with_extra(x: i32, y: i32) -> i32 {
    // 语句会执行一些操作但是不会返回一个值，而表达式会在求值后返回一个值
    let x = x + 1; // 语句
    let y = y + 5; // 语句
    x + y // 表达式
}

fn plus_or_substract(x: i32) -> i32 {
    if x > 5 {
        return x - 5;
    }
    x + 5
}
fn plus_or_minus(x: i32, y: i32) -> i32 {
    if x > y {
        x - y
    } else {
        x + y
    }
}
// 单元类型 ()，是一个零长度的元组。它没啥作用，但是可以用来表达一个函数没有返回值：
// 函数没有返回值，那么隐式返回一个 ()
// 通过 ; 结尾的表达式返回一个 ()

use std::fmt::Debug;
// use std::fmt::Display;

// let x = (let y = 5);
fn report<T: Debug>(item: T) {
    println!("{:?}", item);
}
#[allow(dead_code)]
// 显式的返回了 ()
fn clear(text: &mut String) -> () {
    // text.clear();
    // text
    // 将文本处的字符串替换为空字符串
    *text = String::from("");
}
// 永不返回的函数!
// 当用 `!` 作函数返回值的时候，表示该函数永不返回，特别的，这种语法往往用做会导致程序崩溃的函数：
#[allow(dead_code)]
fn dead_end() -> ! {
    panic!("你已经到了穷途末路，崩溃吧！");
}
// 下面的函数创建了一个无限循环，该循环永不跳出，因此函数也永不返回：
#[allow(dead_code)]
fn forever() -> ! {
    loop {
        //...
    }
}

// Rust 中的函数可以有可变参数，可变参数是一个数组，数组的类型是 [T]，其中 T 是可变的，
// 可变参数在函数调用时会被转换成一个数组，数组的长度是可变参数的个数，数组的元素是可变参数的值。
// 可变参数的个数可以是 0 个，也可以是任意多个。
// 可变参数的类型可以是任意类型，但是 Rust 中的所有类型都是可变的
// Rust提供了一个特殊的可变参数，这个可变参数是一个 & 或者 &mut 的引用，
// 它可以让函数接受一个可变参数的引用，而不是可变参数的值。
// 可变参数的引用可以被传递给函数，但是不能被修改，因此函数不能修改参数的值。
// Rust 提供动态类型，这意味着函数可以接受任意类型的参数，而不需要显式地声明参数的类型。
// Rust 提供了一个特殊的参数，这个参数是一个 &self 的引用，它可以让函数接受一个自身的引用
// Rust 提供动态字符串类型：String，它是一个字符串的可变引用，它可以被传递给函数，该类型被分配到堆上，因此可以动态伸缩，也就能存储在编译时大小未知的文本
// 可变参数的引用可以被传递给函数，但是不能被修改，因此函数不能修改参数的值。
// 在 Rust 中，函数可以接受一个可变参数的引用，而不是可变参数的值
// Rust 中每一个值都 有且只有 一个所有者(变量)，所有者是一个指针，指针指向值的内存地址
// 当所有者(变量)离开作用域范围时，这个值将被丢弃(free)，因此所有者(变量)不能再被使用
// :: 是一种调用操作符
//  String 类型是一个复杂类型，由存储在栈中的堆指针、字符串长度、字符串容量共同组成，其中堆指针是最重要的，它指向了真实存储字符串内容的堆内存，至于长度和容量，则是由编译器自动计算的, 容量是堆内存分配空间的大小，长度是目前已经使用的大小, 当长度和容量相等时，字符串是一个空字符串，当长度大于容量时，编译器会自动扩容，扩容后的容量是原来容量的两倍, 当长度小于容量时，编译器会自动缩容，缩容后的容量是原来容量的一半, 总之 String 类型指向了一个堆上的空间

//// Rust 中每一个值都 有且只有 一个所有者(变量)
//// 当所有者(变量)离开作用域范围时，这个值将被丢弃(free)
fn ownership() {
    let s = String::from("hello");
    // equals to `let s = "hello".to_string();`
    // s.push_str(" world");
    // s所有权转移后失效
    let s3 = s; // s3 is a copy of s so s is no longer valid since it was moved
    let mut s2 = "hello".to_string();
    s2.push(char::from(65));
    // let mut s1 = s;
    // s1.push_str(" world"); // push_str() 在字符串后追加字面值
    // println!("{}", s1);
    // println!("{}", s);
    println!("{}", s2);
    println!("{}", s3);
    // assert!(s3 == s);

    // Rust 永远也不会自动创建数据的 “深拷贝” 因此，任何自动的复制都不是深拷贝，可以被认为对运行时性能影响较小
    // 如果我们确实需要深度复制 String 中堆上的数据，而不仅仅是栈上的数据，可以使用一个叫做 clone 的方法。
    // 在 Rust 中，所有的数据类型都是值类型，值类型的数据在被赋值或传递时，会被复制，而引用类型的数据在被赋值或传递时，不会被复制，而是被引用
    // 对于执行较为频繁的代码(热点路径)，使用 clone 会极大的降低程序性能，需要小心使用！

    let s4 = s3.clone();
    print!("{}", s4);

    // 浅拷贝只发生在栈上，因此性能很高，在日常编程中，浅拷贝无处不在。
    // 通用的规则：任何基本类型的组合可以是 Copy 的，不需要分配内存或某种形式资源的类型是 Copy 的
    // 对于引用类型，如果它的所有者(变量)是 Copy 的，那么它也是 Copy 的
    // 所有整数类型、所有浮点数类型、布尔类型bool、字符类型char、所有指针类型、所有引用类型、元组（当且仅当其包含的类型也都是 Copy 的时候）和所有结构体类型都是 Copy 的
    // 将值传递给函数，一样会发生 移动 或者 复制
    // 如果你想要在函数中修改参数的值，你必须使用引用，而不是值
    let x = 5;
    takes_ownership(s3); // s3 被移动到函数内部，因此 s3 不再有效
    makes_copy(x); // x 被复制到函数内部，但 x 依然有效

    // 同样的，函数返回值也有所有权
    let s5 = gives_ownership(); // gives_ownership 函数返回值被移动到 s5 中
    let s6 = String::from("hello"); // s6 被复制到函数内部，但 s6 依然有效
    let s7 = takes_and_gives_back(s6); // s6 被移动到函数内部，它将返回值赋值给 s7
    println!("{} {}", s5, s7);

    // 所有权很强大，避免了内存的不安全性，但是也带来了一个新麻烦:总是把一个值传来传去来使用它。 传入一个函数，很可能还要从该函数传出去，结果就是语言表达变得非常啰嗦，幸运的是，Rust 提供了新功能解决这个问题。
} // s5,s6, s7 作用域结束后移除，并且清理s5, s7中的内存, s6被移入函数中

fn gives_ownership() -> String {
    let some_string = String::from("hello"); // some_string 被移动到函数内部
    some_string // some_string 返回给调用函数并且被移动到函数外部
}
// 函数传递给另一个函数时，函数内部的变量会被移动到函数外部，而不是被复制
// 在函数内部，变量的生命周期仅限于函数内部，函数结束后，变量的生命周期就结束了
// takes_and_gives_back 将传入字符串移动到函数内部并返回
fn takes_and_gives_back(a_string: String) -> String {
    a_string // a_string 被移动到函数外部并返回
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
} // 作用域结束，some_string 被丢弃并调用 `drop` 方法。占用的内存被释放

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
} // 作用域结束，some_integer 依然有效

// 引用与解引用
// 引用是一个指针，它指向一个变量的内存地址，而不是变量本身。
// 引用的作用是让函数能够访问变量的内容，而不能改变变量的内容。
// 引用的类型是 &T，其中 T 是变量的类型。
// 引用的作用域是在函数内部，drop 函数不会被调用, 因此引用的变量不会被释放
// 引用没有所有权，所以它不能被移动，但是它可以被复制
// 引用的作用域结束后，引用所指向的变量的生命周期也结束了。
// 可变引用同时具有所有权和可变性，所以它们可以被移动，但是它们不能被复制
// 可变引用的类型是 &mut T，其中 T 是变量的类型。
// 可变引用同时只能存在一个，因此它们不能被复制
// 同一作用域，特定数据只能有一个可变引用
// 这种限制的好处就是使 Rust 在编译期就避免数据竞争，数据竞争可由以下行为造成：
// 两个或更多的指针同时访问同一数据
// 至少有一个指针被用来写入数据
// 没有同步数据访问的机制
// 数据竞争会导致未定义行为，这种行为很可能超出我们的预期，难以在运行时追踪，并且难以诊断和修复。而 Rust 避免了这种情况的发生，因为它甚至不会编译存在数据竞争的代码！
use std::ops::Deref;
fn renference_and_dereference() {
    let x = 5;
    let y = &x; // y 是 x 的引用
    println!("x = {}, y = {},", x, y);
    assert_eq!(5, x);
    assert_eq!(5, *y); // 取引用的值

    assert_eq!(5, *y.deref());
    // assert_eq!(5, y.deref());
    // assert_eq!(5, y); // error:无法比较整数类型和引用类型

    // TODO cannot borrow `x` as mutable, as it is not declared as mutable
    // let z = &mut x; // z 是 x 的引用
    // println!("x = {}, z = {}", x, z);

    // 不可变引用 `&s` 无法修改引用所指向的变量的值
    let s = String::from("hello");
    let len = calculate_length(&s);
    println!("The length of '{}' is {}.", s, len);

    // 可变引用 `&mut s` 可以修改引用所指向的变量的值
    let mut s = String::from("hello");
    let _s = change(&mut s);
    println!("{}", s);

    {
        let r1 = &mut s;
        println!("{}", r1);
    } // r1 在这里离开了作用域，所以我们完全可以创建一个新的引用

    let r2 = &mut s;
    println!("{}", r2);

    // 可变引用与不可变引用不能同时存在
    // 可变引用必须被初始化，不可变引用不能被初始化
    // 其实这个也很好理解，正在借用不可变引用的用户，肯定不希望他借用的东西，被另外一个人莫名其妙改变了。多个不可变借用被允许是因为没有人会去试图修改数据，每个人都只读这一份数据而不做修改，因此不用担心数据被污染。
    let mut s = String::from("hello");

    // let r1 = &s; // 没问题
    // let r2 = &s; // 没问题
    // let r3 = &mut s; // 大问题, 无法借用可变 `s` 因为它已经被借用了不可变引用
    // println!("{}, {}, and {}", r1, r2, r3);

    // 引用的作用域 s 从创建开始，一直持续到它最后一次使用的地方，这个跟变量的作用域有所不同，变量的作用域从创建持续到某一个花括号 }
    let r1 = &s;
    let r2 = &s;
    println!("{} and {}", r1, r2);
    // 新编译器中，r1,r2作用域在这里结束

    let r3 = &mut s;
    println!("{}", r3);

    dangling_reference();
}
// 悬垂引用（Dangling References）
// 悬垂引用也叫做悬垂指针，意思为指针指向某个值后，这个值被释放掉了，而指针仍然存在，其指向的内存可能不存在任何值或已被其它变量重新使用。在 Rust 中编译器可以确保引用永远也不会变成悬垂状态：当你拥有一些数据的引用，编译器可以确保数据不会在其引用之前被释放，要想释放数据，必须先停止其引用的使用。
fn dangling_reference() {
    // dangle 不能返回悬垂引用&str
    let reference_to_nothing = dangle();
    println!("{}", reference_to_nothing);
}
// fn dangle() -> &String {
//     let s = String::from("hello");
//     // &s
// } // 这里 s 离开作用域并被丢弃。其内存被释放, 因此&s引用指向一个无效的值

// String的所有权被转移给外面的调用者。
fn dangle() -> String {
    let s = String::from("hello");
    s // 返回值 s 被转移给了函数调用者，s 离开作用域并被丢弃,所以不能返回&s, 因为&s指向的内存已经被释放,否则会成为悬垂引用
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) -> &String {
    some_string.push_str(", world");
    some_string
}
// fn change(some_string: &mut String) {
//     some_string.push_str(", world");
// }

// fn mix_up(a: &str, b: &str) -> String {
//     let mut result = String::new();
//     result.push_str(b);
//     result.push_str(a);
//     result
// }

// 复合类型
// 复合类型是一种类型，它可以是其他类型的子集。
// 在 Rust 中，复合类型是通过元组（tuple）来实现的，元组是一种复合类型。
// 元组可以包含不同类型的值，并且每个值都有一个名字。
// 元组的类型是由其中的每个值的类型组成的。

fn compound_types() {
    let tuple = (1, 2, 3);
    let (x, y, z) = tuple;
    println!("{}, {}, {}", x, y, z);

    let tuple2 = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10);
    let (a, b, c, d, e, f, g, h, i, j) = tuple2;
    println!(
        "{}, {}, {}, {}, {}, {}, {}, {}, {}, {}",
        a, b, c, d, e, f, g, h, i, j
    );
    file_operations();

    // structs();
    // enums();
    // 字符串String 与 切片 &str
    let n = 120;
    greater_than_100(n);

    // 切片(slice)并不是 Rust 独有的概念,它允许你引用集合中部分连续的元素序列，而不是引用整个集合。
    // 对于字符串而言，切片就是对 String 类型中某一部分的引用，它看起来像这样
    // 切片是描述一组属于同一类型、长度不确定的、在内存中连续存放的数据结构，用 [T] 来表述。因为长度不确定，所以切片是个 DST（Dynamically Sized Type）
    // 切片的类型是 &[T]，其中 T 是切片所指向的数据类型。
    // 切片因为长度不确定，属于unsized类型，不能直接访问，所以一般通过&[T]，&mut[T]，Box<[T]>这三种引用方式使用切片。
    // 切片的类型标识符 &[T] 和 &mut [T] 可以用于指向任何类型的数组，而 Box<[T]> 只能指向数组类型。
    // &str 类型 与 &String 类型
    let s = String::from("Hello World");
    let s1 = "Jello".to_string();
    // let s: &str = "World";
    // let r = &s;
    // let all = &s[..];
    let hello = &s[0..5];
    // equals `let hello = &s[..5];`
    let world = &s[6..11];
    // equals `let world = &s[6..];` and `let world = &s[6..s.len()];`
    greet(s1);
    println!("{} {}", hello, world);

    let s = "中国人".to_string();
    // let s = "HELLO";
    let a = &s[0..3]; // 切片的索引必须落在字符之间的边界位置, 每个汉字占用3个字节，&s[0..2] 就会报错
    println!("{} {}", a, s.len());
    greet(s);

    let mut s = String::from("hello world");
    // #[allow(unused_variables)]
    let word = first_word(&s);
    println!("this first word is {}", word); // println 必须在clear之前
    s.clear(); //  clear(&mut self) 方法清空字符串,此处可变引用尝试调用clear方法

    // 字符串的切片类型是 &str，而不是 &String。 数组的切片类型是 &[T]，而不是 &Vec<T>。
    // 因为切片是对集合的部分引用，因此不仅仅字符串有切片，其它集合类型也有，例如数组：
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3]; // 该数组切片的类型是 `&[i32]`
    println!("{:?}", slice);
    assert_eq!(slice, &[2, 3]);

    // 字符串字面量是切片
    // 该切片指向了程序可执行文件中的某个点，这也是为什么字符串字面量是不可变的，因为 `&str` 是一个不可变引用
    // 字符串字面量是不可变的，因为它是一个不可变引用
    let s = "hello world"; // s 是一个字符串字面量，它的类型是 &str
    println!("{}", s);

    // 字符串是由字符组成的连续集合
    // Rust 中的字符是 Unicode 类型，因此每个字符占据 4 个字节内存空间，但是在字符串中不一样，字符串是 UTF8 编码，也就是字符所占的字节数是变化的(1 - 4)，这样有助于大幅降低字符串所占用的内存空间
    // 当 Rust 用户提到字符串时，往往指的就是 `String` 类型和 `&str` 字符串切片类型，这两个类型都是 UTF8 编码
    // 除了 `String` 类型的字符串，Rust 的标准库还提供了其他类型的字符串，例如 `OsString`， `OsStr`， `CsString` 和` CsStr` 等，注意到这些名字都以 `String` 或者 `Str` 结尾了吗？它们分别对应的是具有所有权和被借用的变量。
    let s = "hello world";
    let c = s.chars().nth(0); // 返回字符串中的第一个字符
    println!("{} {:?}", s.len(), c); // 字符串长度

    change_string();
    // 元组是由多种类型组合到一起形成的，因此它是复合类型，元组的长度是固定的，元组中元素的顺序也是固定的。
    // 元组的长度是固定的，因此不能添加或删除元组中的元素
    // 函数返回值可以是一个元组(tuple)，元组可以包含0到32个元素，元组的元素类型可以不相同
    // 元组可以用在函数返回多个值的场合
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup; // 使用模式匹配来解构元组
    println!("{} {} {} {}", x, y, z, tup.2); // 使用.索引来访问元组中的元素
}

// 由于 `String` 是可变字符串，因此我们可以对它进行创建、增删操作
fn change_string() {
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s);
    // 字符串字面量是不可变的，因此我们不能对它进行修改
    // let s = "hello";
    // s.push_str(", world!");
    // println!("{}", s);

    // 创建一个可变空字符串
    let mut s = String::new();
    // 将&str类型的“hello,world”转换成String类型
    // 从现有的&str切片创建String类型
    // let s = "hello,world".to_string();
    // String与&str都是UTF8编码，因此支持中文
    // let mut s = String::from("你好,世界");
    // 将&str类型的"hello,world"添加到s中
    s.push_str("hello,world");
    // 将字符串s转换成&str类型
    // let s = s.as_str();
    // 将字符'!'添加到s中
    s.push('!');
    assert_eq!(s, "hello,world!");
    println!("{}", s);

    let s1 = String::from("hello,");
    // let s2 = s1.clone();
    // let s2 = s1;
    let s2 = String::from("world!");
    println!("{}", s1.len());
    // 在下句中，s1的所有权被移，后面不能再使用s1, s2依然可以使用
    let s3 = s1 + &s2;
    // 使用 `+` 来对字符串进行相加操作,这里之所以使用 `s1 + &s2` 的形式，是因为 `+` 使用了 `add` 方法, `fn add(self, s: &str) -> String`
    println!("{} {}", s2, s3);

    // 如果你想要以 Unicode 字符的方式遍历字符串，最好的办法是使用 chars 方法
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }
    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }
}
//
fn first_word(s: &String) -> &str {
    // s[..s.find(' ').unwrap_or(s.len())].trim()
    &s[..1]
}

// fn first_word(s &String) -> String {
//     let bytes = s.as_bytes();
//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return s[0..i].to_string();
//         }
//     }
//     s.to_string()
// }

// fn first_word(s: &String) -> usize {
//     let bytes = s.as_bytes();
//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return i;
//         }
//     }
//     s.len()
// }
fn greet(name: String) {
    println!("Hello, {}!", name);
}
fn greater_than_100(n: i32) -> bool {
    n > 100
}

// file operations
fn file_operations() {
    #![allow(unused_variables)]
    type File = String;
    fn open(f: &mut File) -> bool {
        true
    }
    fn close(f: &mut File) -> bool {
        true
    }
    #[allow(dead_code)]
    fn read(f: &mut File, save_to: &mut Vec<u8>) -> ! {
        // panic!("not implemented");
        // 这里的 ! 是一个标识符，表示这个函数不会返回任何值，包括 ()，而是直接终止运行。
        // read 是一个发散函数，因此它的返回值类型是 !。
        // 这意味着它的返回值类型是不确定的，因为它可能会在运行时发生错误。
        // unimplemented!() 告诉编译器该函数尚未实现，并且编译器会终止程序运行。
        // 类似的标记还有 todo!() 和 unreachable!()。
        unimplemented!()
    }

    let mut f1 = File::from("file1.txt");
    open(&mut f1);
    // read(&mut f1, &mut Vec::new());
    // read(&mut f1, &mut Vec![0; 10]);
    // read(&mut f1, &mut vec![]);
    close(&mut f1);
    // let file = File::from("hello.txt");
    // fn open(f: &mut File) -> Result<(), &str> {
    //     // ...
    //     Ok(())
    // }
    // fn open(filename: File) -> Result<File, String> {
    //     Ok(filename)
    // }
}
// fn file_struct() {
//     #[derive(Debug)]
//     struct File {
//         name: String,
//         size: u64,
//     }
//     impl File {
//         fn new(name: String, size: u64) -> File {
//             File { name, size }
//         }
//     }
//     let file = File::new(String::from("hello.txt"), 100);
//     println!("{:?}", file);
// }

// 最典型的就是结构体 struct 和枚举 enum
// 结构体是一种类型，它允许你定义一组相关的数据，并且这些数据可以是不同类型的。
fn struct_operations() {
    #[derive(Debug)]
    #[allow(dead_code)]
    struct Point {
        x: i32,
        y: i32,
    }
    impl Point {
        fn new(x: i32, y: i32) -> Point {
            Point { x, y }
        }
    }
    let p = Point::new(1, 2);
    println!("{:?}", p);

    // #[derive(Debug)]
    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }
    let mut user = User {
        active: true,
        username: String::from("someone"),
        email: String::from("someone@some.com"),
        sign_in_count: 1,
    };
    // 必须要将结构体实例声明为可变的，才能修改其中的字段，Rust 不支持将某个结构体某个字段标记为可变。
    user.email = String::from("someone@exaple.com");

    // 构建函数，返回结构体
    fn _build_user(email: String, username: String) -> User {
        User {
            email,
            username,
            active: true,
            sign_in_count: 1,
        }
    }

    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    // let user2 = User {
    //     active: user1.active,
    //     username: user1.username,
    //     email: String::from("another@example.com"),
    //     sign_in_count: user1.sign_in_count,
    // };
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    println!("{}", user1.active);
    println!("{:?}", user1.email);
    println!("{:?}", user1.sign_in_count);
    println!("{:?}", user2.username);
    // 下面这行会报错
    // println!("{:?}", user1);
    // println!("{:?}", user1.username);

    #[derive(Debug)]
    struct File {
        name: String,
        data: Vec<u8>,
    }

    // impl File {
    //     fn new(name: String, data: Vec<u8>) -> File {
    //         File { name, data }
    //     }
    // }
    let file = File {
        name: String::from("hello.txt"),
        // data: vec![0, 1, 2, 3, 4, 5],
        data: Vec::new(),
    };
    let file_name = &file.name;
    let file_length = &file.data.len();
    println!("{:?}", file);
    println!("{} is {} bytes long", file_name, file_length);

    // 元组结构体（Tuple Structs）
    // 结构体必须要有名称，但是结构体的字段可以没有名称，这种结构体长得很像元组，因此被称为元组结构体
    // 元组结构体在你希望有一个整体名称，但是又不关心里面字段的名称时将非常有用
    // add `#[derive(Debug)]` to `Color` or manually `impl Debug for Color`
    #[derive(Debug)]
    struct Color(i32, i32, i32);
    #[derive(Debug)]
    // 添加下划线_, 就可以避免warning
    struct Point2 {
        _x: i32,
        _y: i32,
        _z: i32,
    }
    let black = Color(0, 0, 0);
    let origin = Point2 {
        _x: 0,
        _y: 0,
        _z: 0,
    };
    println!("{:?}", black);
    println!("{:?}", origin);

    // 元结构体（Unit-like Structs）
    // 元结构体是一种特殊的结构体，它没有任何字段和属性

    struct AlwaysEqual;
    let _subject = AlwaysEqual;
    // 我们不关心 AlwaysEqual 的字段数据，只关心它的行为，因此将它声明为元结构体，然后再为它实现摸个特征

    // 枚举结构体
    // 枚举（enumeration）允许通过列举可能的成员来定义一个枚举类型，枚举成员的名字是成员的名字，而不是索引
    // 通过::操作符来访问枚举成员的名字，而不是索引
    #[allow(dead_code)]
    #[derive(Debug)]
    enum Color2 {
        Red,
        Green,
        Blue,
    }
    let r = Color2::Red;
    let g = Color2::Green;
    println!("{:?} {:?}", r, g);

    #[allow(dead_code)]
    #[derive(Debug)]
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(Color2),
    }
    let msg = Message::ChangeColor(Color2::Blue);
    println!("{:?}", msg);

    // - `Quit` 没有任何关联数据
    // - `Move` 包含一个匿名结构体
    // - `Write` 包含一个 `String` 字符串
    // - `ChangeColor` 包含三个 `i32`

    // 当然，我们也可以用结构体的方式来定义这些消息：
    #[allow(dead_code)]
    struct QuitMessage; // 元结构体
    #[allow(dead_code)]
    struct MoveMessage {
        x: i32,
        y: i32,
    }
    #[allow(dead_code)]
    struct WriteMessage(String); // 元组结构体
    #[allow(dead_code)]
    struct ChangeColorMessage(i32, i32, i32); // 元组结构体

    // enum PokerSuit {
    //     Spades,
    //     Hearts,
    //     Diamonds,
    //     Clubs,
    // }
    #[derive(Debug)]
    #[allow(dead_code)]
    enum PokerRank {
        Two,
        Three,
        Four,
        Five,
        Six,
        Seven,
        Eight,
        Nine,
        Ten,
        Jack,
        Queen,
        King,
        Ace,
    }
    #[allow(unused_variables)]
    // struct PokerCard {
    //     suit: PokerSuit,
    //     value: u8,
    // }
    // #[derive(Debug)]
    // struct Poker {
    //     suit: PokerSuit,
    //     rank: PokerRank,
    // }
    // let card = PokerCard {
    //     suit: PokerSuit::Spades,
    //     value: 1,
    // };
    // enum PokerCard {
    //     Spades(Poker),
    //     Hearts(Poker),
    //     Diamonds(Poker),
    //     Clubs(Poker),
    // }
    // let c1 = PokerCard::Spades(Poker {
    //     suit: PokerSuit::Spades,
    //     rank: PokerRank::Two,
    // });

    // let card = Poker {
    //     suit: PokerSuit::Spades,
    //     rank: PokerRank::Two,
    // };
    #[derive(Debug)]
    enum PokerCard {
        Spades(PokerRank),
        Hearts(PokerRank),
        Diamonds(PokerRank),
        Clubs(PokerRank),
    }
    let c1 = PokerCard::Spades(PokerRank::Two);
    let c2 = PokerCard::Hearts(PokerRank::Three);
    let c3 = PokerCard::Diamonds(PokerRank::Four);
    let c4 = PokerCard::Clubs(PokerRank::Five);
    println!("{:?} {:?} {:?} {:?}", c1, c2, c3, c4);

    // 同一个枚举类型下的不同成员还能持有不同的数据类型，例如让某些花色打印 1-13 的字样，另外的花色打印上 A-K 的字样：
    #[allow(dead_code)]
    #[derive(Debug)]
    enum _PokerCard {
        Clubs(u8),
        Spades(u8),
        Diamonds(char),
        Hearts(char),
    }
    let c1 = _PokerCard::Spades(5);
    let c2 = _PokerCard::Diamonds('A');
    println!("{:?} {:?}", c1, c2);

    // Option枚举
    let some_number = Some(5);
    let some_string = Some("a string");
    // 如果使用 None 而不是 Some，需要告诉 Rust Option<T> 是什么类型的，因为编译器只通过 None 值无法推断出 Some 成员保存的值的类型
    let absent_number: Option<i32> = None;

    println!("{:?} {:?} {:?}", some_number, some_string, absent_number);

    // let x: i8 = 5;
    // let y: Option<i8> = Some(5);

    // let sum = x + y;

    // match 表达式就是这么一个处理枚举的控制流结构：它会根据枚举的成员运行不同的代码，这些代码可以使用匹配到的值中的数据。
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("{:?} {:?}", six, none);
}

// 数组操作
// 在Rust中，数组是一个结构体，它包含一个数组的元素，以及一个数组的长度。
// 最常用的数组操作是访问数组中的元素，这些操作可以使用下标来访问数组中的元素。
// 下标是一个 i32 类型，因此数组中的元素也是 i32 类型。
// 最常用的数组有两种，第一种是速度很快但长度固定的array, 第二种是可动态增长的但是有性能损耗的Vector，可以使用Vec<T>来表示，（array-数组，Vector-动态数组）
// 这两种数组的关系跟 &str 和 String 的关系相似，前者是长度固定的字符串切片，后者是可动态增长的字符串。其实，在 Rust 中无论是 String 还是 Vector，它们都是 Rust 的高级类型：集合类型， 因此array存储在栈上，而Vector存储在堆上。
// : [T; N] 声明一个数组类型，其中 T 是数组中元素的类型，N 是数组的长度。数组类型也从侧面说明了数组的元素类型要统一，长度要固定。
// 声明一个数组，并且初始化：语法初始化一个某个值重复出现N次的数组：let array = [value; N]; value是数组中的元素的初始化值，N是数组的长度。
// 当你不确定是使用数组还是动态数组时，那就应该使用动态数组Vector.
// 数组的具体定义很简单：将多个类型相同的元素依次组合在一起，就是一个数组
// array: 长度固定，元素必须是相同类型的，依次线性排列
// 因为数组是连续存放元素的，因此可以通过索引的方式来访问存放其中的元素，索引下标从 0 开始，而不是从 1 开始。
fn array_operations() {
    let a = [1, 2, 3, 4, 5];
    let mut b = [1, 2, 3, 4, 5, 6];
    let c = ["hello", "world"];
    println!("{:?} {:?}", b, c);
    b = [1; 6]; // 可变变量 b 的长度固定为6，不能改变，但元素可变
    let c = [0; 5]; // 创建一个长度为 5 的数组，其中的所有元素都是 0
    let d = [3; 5]; // 创建一个长度为 5 的数组，其中的所有元素都是 3
    println!("{:?} {:?} {:?} {:?}", a, b, c, d);

    // _io_array();
    // _io_array_mut();
    // _io_array_slice();
    // _io_array_iter();
    // _io_array_iter_mut();
    // _io_array_index();
    // _io_array_index_mut();
    // _io_array_index_mut_error();

    // 数组切片
    // 数组切片的类型声明是 &[T] 或者 &mut [T]，其中 T 是数组的元素类型。
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3]; // 创建一个数组切片，其中包含 a 的第二个元素，第三个元素。
    assert_eq!(slice, &[2, 3]);
    println!("切片：{:?}", slice);

    array_slice_operations();
}

fn _io_array_mut() {
    let mut a = [1, 2, 3, 4, 5];
    println!("{:?}", a);
    // 可变数组的元素可以被修改
    a[0] = 10;
    println!("{:?}", a);
}

fn array_slice_operations() {
    // 编译器自动推导出one的类型
    let one = [1, 2, 3];
    // 显式类型标注
    let two: [u8; 3] = [3, 4, 5];
    let blank1 = [6; 3];
    let blank2: [u8; 3] = [7; 3];

    // arrays是一个二维数组，其中每一个元素都是一个数组，元素类型是[u8; 3]
    let arrays: [[u8; 3]; 4] = [one, two, blank1, blank2];

    println!("{:?}", arrays);

    // 借用arrays的元素用作循环中
    for a in &arrays {
        print!("{:?}: ", a);
        // 将a变成一个迭代器，用于循环，可以不显式调用.iter(),会自动调用
        // 你也可以直接用for n in a {}来进行循环
        for n in a.iter() {
            print!("\t{} + 10 = {}", n, n + 10);
        }

        let mut sum = 0;
        // 0..a.len,是一个 Rust 的语法糖，其实就等于一个数组，元素是从0,1,2一直增加到到a.len-1
        for i in 0..a.len() {
            sum += a[i];
        }
        println!("\t({:?} = {})", a, sum);
    }
}

use std::io;
// 越界访问数组
// 下面是一个接收用户的控制台输入，然后将其作为索引访问数组元素的例子：
fn _io_array() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();
    println!("{:?}", index);
    // 读取控制台的输出
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!(
        "The value of the element at index {} is: {}",
        index, element
    );
}

// 流程控制
fn flow_control() {
    // 分支控制 if...else...和 match...及if let...else if let...
    // 循环控制 while...loop...for...
    // 在 Rust 语言中有三种循环方式：for、while 和 loop，其中 for 循环是 Rust 循环王冠上的明珠。
    // 函数控制 return...break...continue...
    for i in 0..5 {
        print!("{}", i);
    }
    println!("");
    for i in 1..=5 {
        print!("{}", i);
    }
    println!("");
    for i in (0..5).rev() {
        print!("{}", i);
    }
    println!("");
    // vec![1, 2, 3, 4, 5].into_iter().for_each(|x| print!("{}", x));
    for item in &vec![1, 2, 3, 4, 5] {
        print!("{}", item);
    }
    println!("");
    // 如果不使用引用的话，所有权会被转移（move）到 for 语句块中，后面就无法再使用这个集合了
    //如果想在循环中，修改该元素，可以使用 mut 关键字：
    let mut collection = vec![1, 2, 3, 4, 5];
    for item in &mut collection {
        *item += 10;
    }
    println!("{:?}", collection);

    // `for item in collection` equals to `for item in collection.into_iter()`
    // `for item in &collection` equals to `for item in collection.iter()`
    // `for item in &mut collection` equals to `for item in collection.iter_mut()`
    // `for item in collection.iter().skip(2).take(2).rev()` equals to `for item in collection.iter().skip(2).take(2).rev()`

    // 循环控制的另一个重要特性是，可以在循环中使用 break 和 continue。
    // break 可以跳出当前循环，而 continue 可以跳过本次循环的剩余执行部分。
    // 如果你想跳出多层循环，可以使用 break 2，表示跳出两层循环。
    // 如果想在循环中获取循环变量，可以使用 loop { } 和 break。
    // 如果你想在循环中使用 break，你必须使用 loop { }，因为 break 只能在 loop { } 中使用。
    // 如果你想在循环中获取元素的索引
    // 可以使用 for (i, item) in collection.iter().enumerate()，这样就可以获取索引和元素了。

    for i in 0..5 {
        if i % 2 == 0 {
            // 0, 2, 4 能被2整除跳出循环
            continue;
        }
        print!("{}", i);
    }
    println!("");
    for i in 1..5 {
        // 若从0开始，因0能被2整除，所以不会打印并跳出整个循环
        // 若从1开始，因1不能被2整除，所以打印1，2可以被2整除，所以不打印并跳出整个循环
        if i % 2 == 0 {
            break;
        }
        print!("{}", i);
    }
    println!("");
    // 如果我们想用 for 循环控制某个过程执行 10 次，但是又不想单独声明一个变量来控制这个流程
    // 可以用 _ 来替代 i 用于 for 循环中，在 Rust 中 _ 的含义是忽略该值或者类型的意思，如果不使用 _，那么编译器会给你一个 变量未使用的 的警告
    for _ in 0..5 {
        print!("{}", "*");
    }
    println!("");

    // while
    let mut i = 0;
    while i <= 5 {
        print!("{}", i);
        i += 1;
    }
    println!("");
    let mut i = 0;
    loop {
        i += 1;
        if i > 5 {
            break;
        }
        print!("{}", i);
    }
    println!("");

    // reak 可以单独使用，也可以带一个返回值，有些类似 return
    // loop 是一个表达式，因此可以返回一个值
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {}", result);
}

fn match_control() {
    #[allow(dead_code)]
    enum Direction {
        North,
        East,
        South,
        West,
    }

    // match 的匹配必须要穷举出所有可能，因此这里用 _ 来代表未列出的所有可能性
    // match 的每一个分支都必须是一个表达式，且所有分支的表达式最终返回值的类型必须相同
    // X | Y，是逻辑运算符 或，代表该分支可以匹配 X 也可以匹配 Y，只要满足一个即可, 单分支多模式的匹配可以用 | 来连接
    // 其实 match 跟其他语言中的 switch 非常像，_ 类似于 switch 中的 default
    // match 后紧跟着的是一个表达式，跟 if 很像，但是 if 后的表达式必须是一个布尔值，而 match 后的表达式返回值可以是任意类型
    let direction = Direction::North;
    match direction {
        // Direction::North => println!("north"),
        Direction::East => println!("east"),
        // Direction::South => println!("south"),
        // Direction::West => println!("west"),
        Direction::North | Direction::South => {
            println!("South or North");
        }
        _ => println!("West"),
    }

    #[allow(dead_code)]
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter,
    }
    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => {
                println!("Lucky penny!");
                1
            }
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        }
    }
    value_in_cents(Coin::Penny);

    // enum IpAddr {
    //     V4(u8, u8, u8, u8),
    //     V6(String),
    // }
    // let home = IpAddr::V4(127, 0, 0, 1);
    // let loopback = IpAddr::V6(String::from("::1"));
    #[allow(dead_code)]
    enum IpAddr {
        Ipv4,
        Ipv6,
    }
    let ip1 = IpAddr::Ipv6;
    let ip_str = match ip1 {
        // IpAddr::Ipv4 => "IPv4",
        // IpAddr::Ipv6 => "IPv6",
        IpAddr::Ipv4 => "127.0.0.1",
        _ => "::1",
    };
    println!("ip_str is {}", ip_str);

    // 美国的某个州（因为在 1999 年到 2008 年间，美国在 25 美分(Quarter)硬币的背后为 50 个州印刷了不同的标记，其它硬币都没有这样的设计）,获取到25美分硬币上刻印的州名称
    #[derive(Debug)]
    #[allow(dead_code)]
    enum UsState {
        Alabama,
        Alaska,
        // ...
        // 注意，这里的编译器会把这些州的名字自动转换为大写
        // 所以这里的名字必须是大写的
        California,
        // ...
    }
    #[allow(dead_code)]
    #[derive(Debug)]
    enum Coin2 {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }
    let coin = Coin2::Quarter(UsState::Alaska);
    println!("coin is {:?}", coin);
    let mut count = 0;
    match coin {
        Coin2::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => count += 1,
    }
    println!("count is {}", count);

    fn _value_in_cents(coin: Coin2) -> u8 {
        match coin {
            Coin2::Penny => 1,
            Coin2::Nickel => 5,
            Coin2::Dime => 10,
            Coin2::Quarter(state) => {
                println!("State quarter from {:?}!", state);
                25
            }
        }
    }

    enum Action {
        Say(String),
        MoveTo(i32, i32),
        ChangeColorRGB(u16, u16, u16),
    }
    // let actions = vec![
    //     Action::MoveTo(1, 2),
    //     Action::Say("hello".to_string()),
    //     Action::ChangeColorRGB(255, 0, 0),
    // ];
    let actions = [
        Action::Say("Hello Rust".to_string()),
        Action::MoveTo(1, 2),
        Action::ChangeColorRGB(255, 0, 0),
    ];
    // actions 隐式调用了 iter() 方法，返回的是一个迭代器，这里的迭代器是一个 &Action
    // 穷尽匹配，穷举所有的 Action，并且每一个 Action 都调用了一次 visit_action() 方法
    // _ 通配符，代表剩余需要匹配的部分，放置于最后一个通配符的位置，可以匹配任意数量的遗漏元素
    // _ => (), 这个表达式是一个空表达式，它的作用是把剩余的元素放到一个空的表达式中，这个表达式可以是任意的，（）表示返回单元类型与所有分支返回值得类型相同，所以匹配到_后，什么也不会发生
    for action in actions {
        match action {
            Action::MoveTo(x, y) => println!("Point from (0, 0) move to ({}, {})", x, y),
            Action::Say(s) => println!("Say {}", s),
            Action::ChangeColorRGB(r, g, b) => println!("Change color to ({}, {}, {})", r, g, b),
        }
    }
    // 在某些场景下，我们其实只关心某一个值是否存在，此时 match 就显得过于啰嗦,可以使用 if let 匹配,如果没有匹配到，则会执行 else 分支
    let x = Some(5);
    if let Some(x_val) = x {
        println!("x is {}", x_val);
    } else {
        println!("x is None");
    }

    // Rust 标准库中提供了一个非常实用的宏：matches!，它可以将一个表达式跟模式进行匹配，然后返回匹配的结果 true or false。
    #[derive(Debug)]
    enum MyEnum {
        A,
        B,
        C,
    }
    let v = vec![MyEnum::A, MyEnum::B, MyEnum::C, MyEnum::A];
    let mut count = 0;
    for v in v.iter() {
        if matches!(v, MyEnum::B) {
            count += 1;
        }
    }
    println!("count is {}", count);
    let duplicate_count = v.iter().filter(|&v| matches!(v, MyEnum::A)).count();
    println!("duplicate_count is {}", duplicate_count);

    let s = v
        .iter()
        .filter(|&v| matches!(v, MyEnum::A))
        .collect::<Vec<_>>();
    println!("s is {:?}", s);

    // 如果想对 v 进行过滤，只保留类型是 MyEnum::Foo 的元素

    // 枚举类型不能用 == 进行判断, 无法将 x 直接跟一个枚举成员进行比较
    // println!("{}", MyEnum::A == MyEnum::A);
    // v.iter().filter(|x| x == MyEnum::A);

    // let foo = Foo {
    //     a: 1,
    //     b: 2,
    // };
    let foo = 'f';
    assert!(matches!(foo, 'f'));
    assert!(matches!(foo, 'f' | 'o'));
    assert!(matches!(foo, 'a' ..= 'z' | 'A' ..= 'Z'));
    let bar = Some(4);
    assert!(matches!(bar, Some(x) if x > 3));

    // 变量覆盖
    // 无论是是 match 还是 if let，他们都可以在模式匹配时覆盖掉老的值，绑定新的值:
    let x = Some(5);
    let y = match x {
        Some(mut val) => {
            val = 6;
            Some(val)
        }
        None => None,
    };
    println!("x is {:?}, y is {:?}", x, y);

    let age = Some(30);
    println!("Before is {:?}", age);
    if let Some(mut age2) = age {
        age2 = 32;
        println!("Current is {:?}", age2);
    }
    println!("After is {:?}", age);

    match age {
        Some(age) => println!("age is {}", age),
        // None => println!("age is None"),
        _ => println!("age is other"),
    }
    println!("{:?}", age);

    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    println!("stack is {:?}", stack);
    while let Some(top) = stack.pop() {
        println!("top is {}", top);
    }

    // 这里使用 enumerate 方法产生一个迭代器，该迭代器每次迭代会返回一个 (索引，值) 形式的元组，然后用 (index,value) 来匹配。
    let v = vec!['a', 'b', 'c'];
    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }

    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {:?}", y),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {:?}", x, y);

    // 通过序列 ..= 匹配值的范围, 序列不仅可以用循环中，还能用于匹配模式，序列只允许用于数字或字符类型，原因是：它们可以连续，字符和数字值是 Rust 中仅有的可以用于判断是否为空的类型
    let x = 5;
    match x {
        1..=5 => println!("one through five"),
        // 'a'..='z' => println!("a through z"),
        _ => println!("something else"),
    }

    // 解构结构体
    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
    }
    let p = Point { x: 0, y: 7 };
    let Point { x: a, y: b } = p;
    // let Point { x, y } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);

    let p = Point { x: 0, y: 7 };

    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        // Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }

    let s = Some(String::from("Hello!"));
    // s 是一个拥有所有权的动态字符串，因为 s 的值会被转移给 _s,所以会报错
    // if let Some(_s) = s {
    //     println!("found a string");
    // }
    if let Some(_) = s {
        println!("found a string");
    }
    println!("{:?}", s);

    let s = (1..=10).map(|x| x * x).collect::<Vec<_>>();
    // let s1 = (..s.len()).map(|x| s[x]).collect::<Vec<_>>();
    println!("{:?}", s);

    // 用 .. 忽略剩余值
    // 对于有多个部分的值，可以使用 .. 语法来只使用部分值而忽略其它值，这样也不用再为每一个被忽略的值都单独列出下划线。.. 模式会忽略模式中剩余的任何没有显式匹配的值部分。
    #[allow(dead_code)]
    struct Point2 {
        x: i32,
        y: i32,
        z: i32,
    }
    let origin = Point2 { x: 0, y: 0, z: 0 };
    match origin {
        Point2 { x, .. } => println!("x is {}", x),
    }

    // 可以使用 @ 来指定一个变量的名字，这样就可以在模式中使用这个变量了
    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, .., last) => println!("Some numbers: {}, {}", first, last),
    }

    // 匹配守卫提供的额外条件
    let num = Some(4);
    match num {
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("{}", x),
        // some_number @ Some(x) => println!("{:?}", some_number),
        None => (),
        // _ => (),
    }

    let x = 4;
    let y = false;

    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }

    enum Message {
        Hello { id: i32 },
    }

    let msg = Message::Hello { id: 5 };

    match msg {
        Message::Hello {
            id: id_variable @ 3..=7,
        } => {
            println!("Found an id in range: {}", id_variable)
        }
        Message::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        Message::Hello { id } => {
            println!("Found some other id: {}", id)
        }
    }

    // 绑定新变量 `p`，同时对 `Point` 进行解构
    let p @ Point { x: px, y: py } = Point { x: 10, y: 23 };
    println!("x: {}, y: {}", px, py);
    println!("{:?}", p);

    let point = Point { x: 10, y: 5 };
    if let p @ Point { x: 10, y } = point {
        println!("x is 10 and y is {} in {:?}", y, p);
    } else {
        println!("x was not 10 :(");
    }

    match 1 {
        num @ (1 | 2) => {
            println!("{}", num);
        }
        _ => {}
    }
}

fn test() {
    #[allow(unused_assignments)]
    let mut aa = 10;
    aa = 2021;
    #[allow(unused_assignments)]
    let mut bb = aa;
    bb = 2022;
    println!("aa = {} bb = {}", aa, bb);
}

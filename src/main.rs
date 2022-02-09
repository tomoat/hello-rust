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
    // 字符串
}

fn test() {
    #[allow(unused_assignments)]
    let mut aa = 10;
    aa = 2021;
    println!("aa = {}", aa);
}

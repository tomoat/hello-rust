//// # 阶乘
// 下面是三个不同版本的阶乘函数，分别以递归、循环和反复运算器的方法写成：

// 這個函數的if-else語句中展示了Rust中可選的隱式返回值，可用於寫出更像函數式程式設計風格的代碼
// 與C++和其他類似的語言不同，Rust中的if-else結構不是語句而是運算式，有返回值
fn recursive_factorial(n: u32) -> u32 {
    if n <= 1 {
        1
    } else {
        n * recursive_factorial(n - 1)
    }
}

fn iterative_factorial(n: u32) -> u32 {
    // 變數用`let`定義，`mut`關鍵字使得變數可以變化
    let mut i = 1u32;
    let mut result = 1u32;
    while i <= n {
        result *= i;
        i += 1;
    }
    result // 顯式返回值，與上一個函數不同
}

fn iterator_factorial(n: u32) -> u32 {
    // 反覆運算器有多種用於變換的函數
    // |accum, x| 定義了一個匿名函數
    // 內聯展開等優化方法會消去區間和fold，使本函數的運行效率和上一個函數相近
    (1..n + 1).fold(1, |accum, x| accum * x)
}

fn main() {
    println!("Recursive result: {}", recursive_factorial(10));
    println!("Iterative result: {}", iterative_factorial(10));
    println!("Iterator result: {}", iterator_factorial(10));
}

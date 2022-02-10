// 并行
// 一个简单的Rust并发示例：

use std::thread;

// 這個函數將創建十個同時併發運行的執行緒
// 若要驗證這一點，可多次運行這個程式，觀察各執行緒輸出順序的隨機性
fn main() {
    // 這個字串是不可變的，因此可以安全地同時被多個執行緒訪問
    let greeting = "Hello";

    let mut threads = Vec::new();
    // `for`迴圈可用於任何實現了`iterator`特性的類型
    for num in 0..10 {
        threads.push(thread::spawn(move || {
            // `println!`是一個可以靜態檢查格式字串類型的巨集
            // Rust的巨集是基於結構的（如同Scheme）而不是基於文本的（如同C）
            println!("{} from thread number {}", greeting, num);
        }));
    }

    // 收集所有執行緒，保證它們在程式退出前全部結束
    for thread in threads {
        thread.join().unwrap();
    }
}

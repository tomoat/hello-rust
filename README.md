## 创建可执行程序的三种方式

### 1、main.rs

### 2、创建 src/bin 目录，在 bin 目录下创建包含 main 函数的的源码文件，可以创建多个文件，`cargo run --bin`

### 3、在 Cargo.toml 文件中配置，指定可执行程序的名称及它的源码路径, `cargo build --release`

```
[[bin]]
name = "hi"
path = "src/hi.rs"

[[bin]]
name = "hello"
path = "src/bin/hello.rs"
```

启动本地文档：rustup doc

io::stdin().read_line(&mut guess)
    .expect("Failed to read line");

let r#fn = "this variable is named 'fn' even though that's a keyword";

// 调用名为 'match' 的函数
r#match();


```rust 
pub fn notify(item: impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

pub fn notify<T: Summary>(item: T) {
    println!("Breaking news! {}", item.summarize());
}
```

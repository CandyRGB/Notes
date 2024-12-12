## Drop Trait
实现 Drop Trait，可以让我们自定义当值将要离开作用域时发生的动作，例如文件、网络资源释放等。
Drop trait 只要求实现`drop`方法，它的参数是对`self`的可变引用。

### 使用 Drop Trait 运行清理代码
下面我们想要打开游戏背包中的武器窗口，然后需要依次关闭：
```rust
struct Form {
    data: String,
}

impl Drop for Form {
    fn drop(&mut self) {
        println!("{}已关闭", self.data);
    }
}

fn main() {
    let form_bag = Form { data: String::from("背包") };
    let form_weapon = Form { data: String::from("武器") };
    println!("{}已打开\n{}已打开", form_bag.data, form_weapon.data);
}
```
执行该程序，得到：
```
背包已打开
武器已打开
武器已关闭
背包已关闭
```
可以看出先创建的后销毁了，类似于压栈出栈的顺序。

### 通过`std::mem::drop`提早丢弃值
我们目前无法显式调用`drop`方法，但是可以调用`std::mem::drop`函数，提前销毁值：
```rust
fn main() {
    // --snip--
    println!("{}已打开\n{}已打开", form_bag.data, form_weapon.data);
    drop(form_bag);
}
```
执行得到：
```
背包已打开
武器已打开
背包已关闭
武器已关闭
```
可以看出通过调用`std::mem::drop`函数，让`form_bag`提前销毁了。
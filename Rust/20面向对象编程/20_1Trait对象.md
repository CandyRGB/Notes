## Trait 对象
- 存储实现了特定 Trait 的任何类型的值。
- 在编译时不检查具体的类型，而是在运行时检查。
- 实现了相同 Trait 的不同类型可以存储在同一个容器中，并在运行时调用它们的方法。

### 定义 Trait 对象
Trait 对象通常使用`dyn`关键字后跟 Trait 名称来定义。例如，如果你有一个 Draw Trait，你可以定义一个 Draw Trait对象：`dyn Draw`。这就代表它可以是任何实现了 Draw trait 的类型。

### 使用 Trait 对象
Trait对象可以存储在变量中，但它们通常存储在智能指针中，如`Box`、`Rc`或`Arc`，因为它们需要在堆上分配内存以支持不同大小的类型。例如：
```rust
let draw_object: Box<dyn Draw> = Box::new(Button);
```
这里，`draw_object`是一个`Box`智能指针，它存储了一个实现了 Draw Trait 的对象。这个对象的具体类型可以是任何实现了 Draw Trait 的类型，如`Button`或`TextBox`。

### 动态分发
动态分发是指方法调用的确切代码在编译时不是确定的，而是在**运行时**通过查找对象的虚函数表来确定，会在运行时带来额外的开销。
当通过 Trait 对象调用方法时，Rust 使用动态分发来确定调用哪个具体实现。

### Trait 对象必须保证对象安全
如何保证某个对象安全：
- 方法的返回类型不是`Self`
    比如 Clone Trait 中 clone 方法的签名是这样的：
    ```rust
    pub trait Clone {
        fn clone(&self) -> Self;
    }
    ```
    它会返回`Self`，这就是种不安全的 Trait 对象。
- 方法中不包含任何泛型类型参数


### 示例
我们打算实现一个 GUI 工具：
- 它会遍历组件的列表，依次调用各个组件的 draw 方法进行绘制。
首先，定义一个 Draw trait：
```rust
pub trait Draw {
    fn draw(&self);
}
```
为`Button`和`TextBox`这两个组件实现这个 trait：
```rust
// 按钮
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    // 这里不提供实现，仅打印一段文字
    fn draw(&self) {
        println!("Drawing a button");
    }
}

// 文本框
pub struct TextBox {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for TextBox {
    // 这里不提供实现，仅打印一段文字
    fn draw(&self) {
        println!("Drawing a text box");
    }
}
```
让`Screen`持有 Draw Trait 对象，在`Screen`实现一个可以绘制所有组件的方法。
```rust
// 屏幕
pub struct Screen {
    components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn new(components: Vec<Box<dyn Draw>>) -> Screen {
        Screen {
            components,
        }
    }

    pub fn draw_all(&self) {
        for component in self.components.iter() {
            component.draw();
        }   
    }
}
```

最后我们在一个主线程使用它们：
```rust
use theards::{Screen, TextBox, Button};

fn main() {
    let screen = Screen::new(vec![
        Box::new(TextBox {
            width: 10,
            height: 5,
            label: String::from("yes"),
        }),
        Box::new(Button {
            width: 10,
            height: 5,
            label: String::from("ok"),
        })
    ]);
    screen.draw_all();
}
```
运行后，我们得到了正确的结果：
```
Drawing a text box
Drawing a button
```
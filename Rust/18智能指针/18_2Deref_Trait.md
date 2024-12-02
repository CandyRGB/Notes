## Deref Trait
- 实现 Deref Trait 使我们可以自定义解引用运算符`*`的行为。
- 通过实现 Deref，智能指针可像常规引用一样来处理。

### 普通指针解引用
```rust
fn main() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);  // 这里必须对y解引用才能取到值5
}
```

### 把`Box<T>`当作引用
这里将上例中的`y`替换成`Box<T>`类型的智能指针，同样也可以解引用。
```rust
fn main() {
    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}
```

### 定义自己的智能指针
我们仿照`Box<T>`定义一个自己的`MyBox<T>`，同时定义构造方法。
```rust
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
```
然后我们为`MyBox<T>`实现 Deref Trait：
```rust
use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    type Target = T;    // 定义 Deref Trait 的关联类型

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
```
这样我们就可以对`MyBox<T>`类型的实例解引用：
```rust
fn main() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);  // *y == *(y.deref())
}
```

### 函数和方法的隐式解引用转化（ Deref Coercion ）
假设类型`T`实现了 Deref Trait，Deref Coercion 可以把 T 的引用转化为 经过`deref`操作后生成的引用。
当这种特定类型的引用作为实参传递给和形参类型不同的函数或方法时将自动进行。这时会有一系列的`deref`方法被调用，把我们提供的类型转换成了参数所需的类型。
```rust
fn hello(name: &str) {
    println!("Hello, {name}!");
}
fn main() {
    let m = MyBox::new(String::from("Rust"));   // m: MyBox<String>
    hello(&m);  // &m: &MyBox<String> .deref() -> &string .deref() -> &str
}
```
如果没有隐式解引用，那么将是下面的情形：
```rust
hello(&(*m)[..]);
```
这样的代码对于我们来说比较难以阅读，不如上面隐式解引用的清晰。

### 解引用与可变性
DerefMut Trait 用于重载 **可变** 引用的`*`运算符。
Rust 在发现类型和 trait 实现满足三种情况时会进行 Deref 强制转换：
- 当 `T: Deref<Target=U>` 时从 `&T` 到 `&U`。
- 当 `T: DerefMut<Target=U>` 时从 `&mut T` 到 `&mut U`。
- 当 `T: Deref<Target=U>` 时从 `&mut T` 到 `&U`。

可变引用可转化为不可变引用和可变引用，而不可变引用仅能转化为不可变引用。
## `RefCell<T>`与内部可变性

### 内部可变性（interior mutability）
**内部可变性**（Interior mutability）是 Rust 的设计模式之一，它允许你有不可变引用时也可以对数据进行修改。

### `RefCell<T>`
#### `Box<T>` vs `RefCell<T>`
|`Box<T>`|`RefCell<T>`|
|:-|:-|
|**编译阶段**强制代码遵守借用规则|**运行时**才会检查借用规则|

它们的同一个数据都是只能有一个所有者。

#### 使用场景
- 只能用于**单线程**场景。
- 实现某些特定的内存安全场景。（不可变环境下修改自身数据）

### 内部可变性的用例：mock 对象
**测试替身**（test double）是一种在软件测试中用来替代实际组件的临时对象，目的是为了让测试能够顺利进行，而不需要依赖于实际的组件。**mock 对象** 是特定类型的测试替身，它们记录测试过程中发生了什么以便可以断言操作是正确的。
下面我们写一个根据背包容量占用来发送消息的程序：
```rust
pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct Bag<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> Bag<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &'a T, max: usize) -> Bag<'a, T> {
        Bag {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percent = self.value as f64 / self.max as f64;

        if percent >= 1.0 {
            self.messenger.send("你的背包已满，请清理道具!");
        } else if percent >= 0.9 {
            self.messenger
                .send("你的背包只有一点点空间了!");
        } else if percent >= 0.75 {
            self.messenger
                .send("你已经使用了大部分的背包空间了!");
        }
    }
}
```
我们想要测试`set_value`，但是它并不返回任何的值，无法直接测试。考虑到其中使用`send`方法，而`send`是接口`Messenger`的方法，因此我们可以尝试为结构体`Messenger`创建一个`Mock`，让`send`的执行可以被记录下来：
```rust
#[cfg(test)]
mod tests {
    use super::*;

    struct MockMessenger {
        sent_messages: Vec<String>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: vec![],
            }
        }
    }

    impl Messenger for MockMessenger {
        // 此处有错误：想要改变不可变引用内部的值
        fn send(&self, message: &str) {
            self.sent_messages.push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = Bag::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        assert_eq!(mock_messenger.sent_messages.len(), 1);
    }
}
```
通过使用`RefCell<T>`，我们就可以对不可变引用的内部进行可变引用：
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            self.sent_messages.borrow_mut().push(String::from(message));    // borrow_mut：内部可变引用
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = Bag::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1); // borrow：内部不可变引用
    }
}
```

### `RefCell<T>`在运行时记录借用
#### `borrow` & `borrow_mut`
- `borrow`方法返回`Ref<T>`类型的智能指针
- `borrow_mut`方法返回`RefMut<T>`类型的智能指针。

这两个类型都实现了 Deref，所以可以当作常规引用对待。

#### `RefCell<T>`记录借用信息
`RefCell<T>` 记录当前有多少个活动的 `Ref<T>` 和 `RefMut<T>` 智能指针：
- 调用 `borrow`，`RefCell<T>` 将活动的不可变借用计数加一。
- 当 `Ref<T>` 值离开作用域时，不可变借用计数减一。
- `RefCell<T>` 只允许有多个不可变借用或一个可变借用。

### `RefCell<T>`和`Rc<T>`结合使用
`RefCell<T>`：一个所有者的可变数据。
`Rc<T>`：多个所有者的不可变引用数据。
将它们结合，可以得到：多个所有者的内部可变数据。这种方式很常用，比如创建可修改结点值的交叉链表：
```rust
#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let value = Rc::new(RefCell::new(5));

    // 让多个所有者拥有value
    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));
    
    // 改变value的值
    *value.borrow_mut() += 10;

    println!("a after = {a:?}");
    println!("b after = {b:?}");
    println!("c after = {c:?}");
}
```

### 其它可实现内部可见性的类型
- `Cell<T>`：通过**复制**来访问数据
- `Mutex<T>`：用于实现**跨线程**情形下内部可变性模式
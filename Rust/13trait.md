# <center>trait</center>
trait类似于其他语言中的常被称为**接口**（interface）的功能。

## 定义trait
使用`trait`关键字来声明一个 trait，其中的方法可以不用给出具体实现，仅需给出方法签名：
```rust
// lib.rs

pub trait Summary {
    // 摘要
    fn summarize(&self) -> String;
}
```
该trait中的方法用来实现对数据的摘要。

## 在类型上实现trait
我们要分别对新闻文章和推文做摘要，这是共同行为，因此使用`Summary`这个`trait`来重载各自的方法：
```rust
// lib.rs

// 新闻文章
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

// 推文
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
```
`trait`必须和类型一起引入作用域以便使用额外的trait方法：
```rust
// main.rs
use aggregator::{Summary, Tweet};   //aggregator:package名称

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
}
```

## 实现trait的约束
- 为某个类型实现某个trait的前提：
  *这个类型* **或** *这个 trait* 是在 **本地crate** 里定义的。
- **不能**为 *外部类型* 实现 *外部 trait*。

## 默认实现
有时为 trait 中的某些或全部方法提供默认的行为，而不是在每个类型的每个实现中都定义自己的行为是很有用的。这样当为某个特定类型实现 trait 时，可以选择保留或重载每个方法的默认行为。
```rust
pub trait Summary {
    // 默认摘要实现
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}
```
默认实现允许调用相同 trait 中的其他方法，哪怕这些方法没有默认实现。
下例没有默认实现`summarize_author()`，但在`summarize()`可以调用它。
```rust
pub trait Summary {
    // 作者
    fn summarize_author(&self) -> String;

    // 默认摘要实现
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}
```
当然，我们必须在具体类型方法上为`summarize_author()`具体实现，才能使用`summarize()`。
```rust
impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}
```

## trait作为参数
```rust
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
```
上例中，凡是实现了`Summary`的类型都可以作为参数传入。这种语法适用于传入参数少的情况。

### trait bound语法
我们把上例改为trait bound语法：
```rust
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}
```
### 通过`+`指定多个 trait bound
如果`notify`需要显示`item`的格式化形式，同时也要使用`summarize`方法，那么`item`就需要同时实现两个不同的 trait：`Display`和`Summary`。这可以通过`+`语法实现：
```rust
pub fn notify(item: &(impl Summary + Display)) {···
```
`+`语法也适用于泛型的 trait bound：
```rust
pub fn notify<T: Summary + Display>(item: &T) {···
```

### 通过`where`简化 trait bound
然而，使用过多的 trait bound 也有缺点。每个泛型有其自己的 trait bound，所以有多个泛型参数的函数在名称和参数列表之间会有很长的 trait bound 信息，这使得函数签名难以阅读。
```rust
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> String {···
```
使用`where`可让签名变得清晰易读：
```rust
fn some_function<T, U>(t: &T, u: &U) -> String
where
    T: Display + Clone,
    U: Clone + Debug,
{···
```

## trait作为返回类型
```rust
fn returns_summarizable() -> impl Summary {
    Tweet {···
}
```
上例必定返回一个类型`Tweet`，而当返回类型不确定时，代码就无法通过编译：
```rust
// 无法通过编译
fn returns_summarizable(switch: bool) -> impl Summary {
    if switch {
        NewsArticle {···
    } else {
        Tweet {···
    }
}
```

## 使用 trait bound 有条件地实现方法
通过使用带有 trait bound 的泛型参数的`impl`块，可以有条件地只为那些实现了特定 trait 的类型实现方法。
```rust
use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
```
也可以对任何实现了特定 trait 的类型有条件地实现 trait，这称作*覆盖实现*。例如，标准库为任何实现了`Display` trait 的类型实现了`ToString` trait：
```rust
impl<T: Display> ToString for T {
    // --snip--
}
```
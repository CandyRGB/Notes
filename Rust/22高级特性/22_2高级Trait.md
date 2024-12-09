## 高级 Trait

### 关联类型
之前在[创建自定义迭代器](../17函数式语言特性：闭包与迭代器/17_2迭代器.md#创建自定义迭代器)中我们已经使用过关联类型。
关联类型（associated types）让我们可以在 trait 里面增加一个待定义的类型（类型占位符），将类型占位符与 trait 相关联，这样 trait 的方法签名中就可以使用这些占位符类型。trait 的实现者在实现这个 trait 的时候，会指定一个具体类型，来替换掉这个占位符。
听描述有些像泛型，那如果用**泛型**定义呢？
```rust
pub trait Iterator<T> {
    fn next(&mut self) -> Option<T>;
}
```
在 trait 中使用两者的**区别**：
**关联类型**：不能多次实现这个 trait，我们只能选择一次`Item`会是什么类型，也必须选择具体的类型。
**泛型**：对于一个泛型 struct，可以选择多种类型来对泛型 trait 实现。比如指定`String`类型实现，还能对`i32`类型进行实现。

### 默认泛型类型参数
当使用泛型类型参数时，可以为泛型指定一个默认的具体类型。如果默认类型就足够的话，这样在为具体类型实现 trait 时，就有可能无需标注泛型。
`Add` trait 中使用了默认泛型类型，`<Rhs=Self>`意为默认的类型为自己的类型，也就是`add`方法传入参数的类型默认是与调用`add`方法的类型相同的。
```rust
trait Add<Rhs=Self> {
    type Output;

    fn add(self, rhs: Rhs) -> Self::Output;
}
```
一般情况下，需要被相加的两个元素总是类型相同的，不过也有不同的时候，这时就需要给出具体类型：
```rust
use std::ops::Add;

#[derive(Debug)]
struct Millimeters(u32);
struct Meters(u32);

impl Add<&Meters> for &Millimeters {
    type Output = Millimeters;

    fn add(self, other: &Meters) -> Self::Output {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

fn main() {
    let mm = Millimeters(314);  // 314mm
    let m = Meters(1);  // 1m
    let sum = &mm + &m;
    println!("{}mm + {}m = {}mm", mm.0, m.0, sum.0);    // 314mm + 1m = 1314mm
}
```
上例中，需要相加的元素为两个不同的类型：“米”和“毫米”，并且使用借用以不获得结构体元组的所有权。

### 调用相同名称的方法
下例中，`Human`拥有三个同名的方法，一个是它自己的，另外两个是不同`trait`的。
```rust
trait Wing {
    fn fly(&self);
}

trait Plane {
    fn fly(&self);
}

struct Human;

impl Wing for Human {
    fn fly(&self) {
        println!("像鸟一样飞行！");
    }
}

impl Plane for Human {
    fn fly(&self) {
        println!("坐在天空上");
    }
}

impl Human {
    fn fly(&self) {
        println!("靠自己很难了");
    }
}
```
那么如何分别调用这三个方法呢？
首先，我们尝试平常调用方法的形式：
```rust
fn main() {
    let person = Human;
    person.fly();   // 靠自己很难了
}
```
根据打印结果，得知这样会直接调用本身的方法。
为了调 trait 上的方法，我们就需要指定然后把本身作为参数传入：
```rust
fn main() {
    let person = Human;
    Wing::fly(&person); // 像鸟一样飞行！
    Plane::fly(&person);// 坐在天空上
}
```
但是，有时候并不会把`&self`作为参数：
```rust
trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

fn main() {
    println!("A baby dog is called a {}", Animal::baby_name()); // 错误：rust 不知道是为哪个类型实现的这个 trait
}
```
为了能够正常调用 trait 上的方法，需要使用**完全限定语法**：
```rust
fn main() {
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
}
```
在方法前使用`<Type as Trait>`，rust 就知道是为哪个 type 实现的这个 trait。

### 父 Trait
Trait 中的方法也可能使用到其他 Trait 的方法。
下面我们希望可以打印出这样显示点位的输出：
```
**********
*        *
* (1, 3) *
*        *
**********
```
```rust
use std::fmt;

trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();  // 这个方法需要使用 Display Trait
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}
```
写法上类似于为泛型添加 trait bound，这里泛型替换成了 trait，所以就是为 trait 添加 trait bound。也就是实现`OutlinePrint` trait 的类型也必须实现`Display` trait。
下面我们为`Point`实现`OutlinePrint`的同时也需要实现`Display`，否则就会报错。
```rust
struct Point {
    x: i32,
    y: i32,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl OutlinePrint for Point {}
```

### 在外部类型上实现外部 trait
**newtype 模式**（newtype pattern）可以在外部类型上实现外部 trait。
实际上就是把外部类型封装到了内部创建的类型里，然后给内部类型实现实现外部 trait。
```rust
use std::fmt;

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn main() {
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
}
```
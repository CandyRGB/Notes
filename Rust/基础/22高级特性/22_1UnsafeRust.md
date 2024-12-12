## Unsafe Rust

### 裸指针 `*`
1. 创建裸指针
    ```rust
    let mut num = 5;

    let r1 = &num as *const i32;    // 不可变裸指针
    let r2 = &mut num as *mut i32;  // 可变裸指针
    ```
2. 解引用裸指针
   解引用时，需要放在`unsafe`块中，对裸指针使用解引用运算符`*`。
    ```rust
    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }
    ```

### Unsafe fn
#### 创建并使用 unsafe 函数
相比普通函数来说，unsafe 函数的创建与使用都要加上`unsafe`关键字。
```rust
unsafe fn dangerous() {}

unsafe {
    dangerous();
}
```

#### 使用`extern`调用其它语言的函数
```rust
// 调用 C 标准库中的 abs 函数
extern "C" {
    fn abs(input: i32) -> i32;
}

fn main() {
    // 在 unsafe 块中使用它
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}
```
extern 块中声明的函数在 Rust 代码中总是不安全的，因为其他语言不会强制执行 Rust 的规则，并且 Rust 无法对它们进行检查。

#### 其它语言调用 Rust 函数
想在 C 语言中使用 Rust 函数，如何将 Rust 函数暴露给C++代码呢？
1. 编写 Rust 函数并使用 `extern "C"`标记：
    ```rust
    #[no_mangle] // 告诉编译器不要修改函数名
    // 让 C 语言可以调用 call_from_c
    pub extern "C" fn call_from_c() {
        println!("Just called a Rust function from C!");
    }
    ```
2. 编译 Rust 代码为动态库：
    ```rust
    cargo build --release
    ```
3. 在 C 中声明外部 Rust 函数：
   在 C 中使用`extern "C"`来声明外部 Rust 函数。
    ```C
    extern "C" {
        void call_from_c();
    }

    int main() {
        call_from_c();
        return 0;
    }
    ```
4. 链接 Rust 动态库运行程序。

### 访问或修改可变静态变量
[静态变量](../00小知识.md#静态static变量)可以是可变的。访问和修改可变静态变量都是**不安全**的。
声明、访问和修改可变静态变量：
```rust
static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

fn main() {
    add_to_count(3);

    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}
```
就像常规变量一样，使用 mut 关键来指定静态变量可变性。
拥有多个线程访问可变静态变量则可能导致数据竞争，因此它不安全。

### Unsafe Trait
```rust
unsafe trait Foo {
    // 里面有不安全方法
}

unsafe impl Foo for i32 {
    // 为 unsafe trait 的实现也需标记为 unsafe
}

fn main() {}
```

### 访问 union 的字段
- union 和 struct 类似，但是在一个实例中同时只能使用一个声明的字段。
- union 主要用于和 C 代码中的联合体交互。
- 访问 union 的字段需要放在 unsafe 块中，因为 Rust 无法保证当前存储在union中的数据类型。
```rust
#[repr(C)]  // 使内存布局与 C 中的 union 一致
union MyUnion {
    f1: u32,
    f2: f32,
}

let mut u = MyUnion { f1: 1 };  // 实例化
unsafe { u.f1 = 5; }            // 修改
let value = unsafe { u.f1 };    // 赋给其它变量
```
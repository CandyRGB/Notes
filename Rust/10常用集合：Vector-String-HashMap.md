# <center>常用的集合<center>

## Vector
类似于数组，但是动态可变，实现类型为`Vec<T>`。它的数据分配在内存中的堆上

### 创建方式
1. 使用`Vec::new()`创建一个空的vector。
2. 使用`vec![]`宏。和数组一样，元素相同时，使用`[value; number]`创建。
   ```rust
   let v = vec![1, 2, 3, 4, 5]; // v: Vec<i32>
   let v1 = vec![2; 5];         // [2, 2, 2, 2, 2] 
   ```

### 对Vector操作
1. 增：`vector.push()`
2. 删：`vector.pop()`
3. 查：
   1. 索引
      ```rust
      println!("{}", v[0]);    // 1
      ```
   2. get方法
      ```rust
      match v.get(0) {
          Some(i) => println!("{}", i),
          None => (),
      }
      ```

### 遍历Vector
```rust
for i in &v {
    println!("{}", i);
}
```

### Enum+Vector
枚举变体可以附加值，当vector中的元素是枚举类型时，就可以实现使用vector存放不同的数据类型。
```rust
enum CellType {
    Int(i32),
    Float(f64),
    Text(String),
}

let row = vec![
    CellType::Int(63),
    CellType::Float(20.02),
    CellType::Text(String::from("fdn")),
];
```

## String
- utf-8编码
  UTF-8的特点是对不同范围的字符使用不同长度的编码，长度从1个字节到4个字节。String中的字符就是utf-8编码。

### 更新String
```rust
let mut s = String::from("ansi");
s.push_str("bi");    //ansibi
s.push('i');         //ansibii
let s1 = String::from(" rust");
let s2 = s + &s1;    //ansibi rust
let s3 = format!("{}-{}", s1, s2);  //" rust-ansibi rust"
```
使用+运算符后，s将无法使用，它的所有权交给了s2，类似于`s2 = s.add(&s1)`。

### 切割String
```rust
let hello = "你好";
let s = &hello[..3];    //你
```
- 切割必须谨慎使用，不能跨过字符边界

## HashMap
哈希表，在python中叫做字典，有一个键值对，通过键可以快速地查找到想要的数据。哈希表在标准库中，使用时需引入。
```rust
use std::collections::HashMap;
```
### 创建与添加
```rust
let mut h = HashMap::new(); //创建
h.insert(9, 17);            //添加
h.insert(5, 23);

// collect方法创建哈希表
let color = vec![String::from("Black"), String::from("White")];
let v = vec![0, 255];
let mut h2: HashMap<_, _> = color.iter().zip(v.iter()).collect();
```

### 访问HashMap中的值
使用`get(K)`返回`Option<&V>`。
```rust
let v1 = h2.get(&color[0]);
match v1 {
    Some(v) => println!("{}", v),           //0
    None => println!("value not exist"),
};
```

### 遍历哈希表
```rust
for (k, v) in &h2 {
    println!("{}: {}", k, v);
}
```
如果你运行上面代码，可能你注意到了，每次输出的顺序不相同，这是哈希表无序的特性决定的。

### 更新哈希表
1. 替换现有的V
   ```rust
   let binding = String::from("Grey");
   h2.insert(&binding, &64);
   h2.insert(&binding, &128);

   println!("{:?}", h2.get(&binding));    //Some(128)
   ```
2. 哈希表中没有要插入的K才插入该键值对
   ```rust
   h2.entry(&binding).or_insert(&0);    //无法加入
   ```
3. 基于现有V更新V
   ```rust
   let s = "hello world hello cargo";
   let mut map = HashMap::new();

   // 统计各个单词出现的次数
   for word in s.split_whitespace() {
       let count = map.entry(word).or_insert(0);
       *count += 1;
   }
   println!("{:#?}", map);
   ```
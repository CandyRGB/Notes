# <center>Cargo</center>
## 创建项目
打开终端，输入`cd "目录路径"`转到想要创建项目的文件夹下，再输入`cargo new 项目名称`即可创建rush项目。
rush项目命名方式为下划线命名法，下面新建项目：
```c
cargo new hello_cargo
cd h*       //通配符查询以h开头的文件并转入
code .      //使用vscode打开该文件夹
```
生成的项目中有以下文件和文件夹：
+ src
  - main.rs(主函数源代码)
    ```rust
    fn main() {
    println!("Hello, world!");
    }
    ```
+ target
+ Cargo.toml(Cargo的配置格式)
  ```toml
  [package]
  name = "my_hello_world"
  version = "0.1.0"
  edition = "2021"

  [dependencies]
  ```
## 运行项目
1. `carge build`仅生成可执行文件
2. `carge run`编译并运行
3. `carge check`检查代码并编译，是最快的选项，可在编写代码时使用它检查以提高效率
4. `carge build --release`正式发布使用
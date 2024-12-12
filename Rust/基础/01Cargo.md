# <center>Cargo</center>
## 创建项目
打开终端，输入`cd "目录路径"`转到想要创建项目的文件夹下，再输入`cargo new 项目名称`即可创建rush项目。
rush项目命名方式为下划线命名法，下面新建并打开项目：
```shell
cargo new hello_cargo
cd h*
code .
```
生成的项目目录结构以及文件中的内容如下：
```
hello_cargo
├── src
│   └── main.rs(主函数源代码)
│       fn main() {
│           println!("Hello, world!");  //打印宏（带换行符）
│       }
├── target
└── Cargo.toml(Cargo的配置格式)
    [package]
    name = "hello_world"
    version = "0.1.0"
    edition = "2021"
 
    [dependencies]
```
## 运行项目
1. `carge build`仅生成可执行文件
2. `carge run`编译并运行
3. `carge check`检查代码并编译，是最快的选项，可在编写代码时使用它检查以提高效率
4. `carge build --release`正式发布使用  

运行后，会在终端生成：
`Hello, world!`
> <font face = "楷体">纸上得来终觉浅，绝知此事要躬行。</font>

### grep简单功能介绍
grep 是 “**G**lobally search a **R**egular **E**xpression and **P**rint.” 的首字母缩写。grep 最简单的使用场景是在特定文件中搜索指定字符串。为此，grep 获取一个文件路径和一个字符串作为参数，接着读取文件并找到其中包含字符串参数的行，然后打印出这些行。

### 知识应用实践
grep 项目将会结合之前所学的一些内容：
- [代码组织](../09代码组织.md)
- [vector](../10常用集合：Vector-String-HashMap.md#vector) 和[字符串](../10常用集合：Vector-String-HashMap.md#string)
- [错误处理](../11错误处理.md)
- 合理的使用 [trait](../13trait.md) 和[生命周期](../14生命周期.md)
- [测试](../15编写自动化测试.md)

### 创建项目
依旧是熟练的三部走：
```shell
cargo new minigrep
cd mi*
code .
```
我们创建了minigrep项目，并在vscode中打开了它。
我们希望这个程序应该这样运行：
```shell
cargo run SearchstringExample filename.txt
```
运行这个程序需要接收两个参数：要查找的内容和指定的文件。
但我们现在程序`cargo run`忽略任何传递给它的参数，为此我们需要能够接收命令行参数。
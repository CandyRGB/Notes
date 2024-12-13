# 构建 TCP Server

## workspace
创建项目：
```shell
cargo new cs
cd cs
cargo new tcpserver
cargo new tcpclient
code .
```
更改 cs 目录下的 cargo\.toml：
```toml
[workspace]

members = ["tcpserver", "tcpclient"]
```

## 监听 TCP 连接
文件路径：cs/tcpserver/src/main\.rs
```rust
use std::net::TcpListener;

fn main() {
    // 127.0.0.1 本地回环地址：将数据包发送回本地主机，不进行网络传输
    // 3000 端口号
    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();
    println!("Running on port 3000...");    // 提示服务器开始运行

    for stream in listener.incoming() {
        let _stream = stream.unwrap();

        println!("Connection established!");    // 提示连接成功
    }
}
```
我们先使用`bind`新建一个`TcpListener`类型的实例`listener`。
然后使用`incoming`方法，产生`TcpStream`（TCP 流）的迭代器。流（stream）代表一个客户端和服务端之间打开的连接。连接（connection）代表客户端连接服务端、服务端生成响应以及服务端关闭连接的全部请求/响应过程。
我们的服务器到这里就可以连接了。在终端执行`cargo run -p tcpserver`，让服务器运行：
```
Running on port 3000...
```
这里不使用浏览器连接，我们要构建的是 C/S （客户端/服务器）模型，因此之后编写客户端代码，使其可以连接服务器。

## 请求建立 TCP 连接
文件路径：cs/tcpclient/src/main\.rs
```rust
use std::net::TcpStream;

fn main() {
    let _stream = TcpStream::connect("localhost:3000").unwrap();
}
```
在客户端，使用`connect`向`localhost:3000`发送一条连接。再打开一个终端，进入`cs`目录，执行`cargo run -p tcpclient`，会发现服务器端的命令行打印：
`Connection established!`
提示我们 C/S 连接成功。

## 通过 stream 发送/接收消息
文件路径：cs/tcpserver/src/main\.rs
```rust
use std::io::{Read, Write};
use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();
    println!("Running on port 3000...");     // 提示服务器开始运行

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        println!("Connection established!"); // 提示连接成功

        let mut buf = [0; 1024];
        stream.read(&mut buf).unwrap(); // 把 stream 中的 bytes 转移到 buf  
        stream.write(&mut buf).unwrap();// 把 buf 中的 bytes 转移到 stream
    }
}
```
文件路径：cs/tcpclient/src/main\.rs
```rust
use std::io::{Read, Write};
use std::net::TcpStream;
use std::str;

fn main() {
    let mut stream = TcpStream::connect("localhost:3000").unwrap();
    stream.write(b"Hello").unwrap();    // 向 stream 中写 bytes("Hello")

    let mut buf = [0; 5];
    stream.read(&mut buf).unwrap();     // 把 stream 中的 bytes 转移到 buf

    println!("Response from server:{:?}", str::from_utf8(&buf).unwrap());
}
```
以上程序发送/接收消息的流程：
1. client写：client 向 stream 中写 bytes("Hello")
2. server读：server 把 stream 中的 bytes 转移到 它的 buf
3. server写：server 把 它的 buf 中的 bytes 转移到 stream
4. client读：client 把 stream 中的 bytes 转移到 它的 buf

buf 中的元素是 byte(`u8`)，`str::from_utf8()`可以把 bytes 转化为`str`。
先运行服务器，再运行客户端：
```shell
cargo run -p tcpserver
Running on port 3000...
Connection established!
```
```shell
cargo run -p tcpclient
Response from server:"Hello"
```
最终客户端接收到了它发向服务器的`"Hello"`。
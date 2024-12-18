## 构建 server, router, handler 模块
准备工作：
在 hs/http/lib\.rs 下，把`httprequest`和`httpresponse`作为 lib crate 的模块，也就是 HTTP library：
```rust
pub mod httprequest;
pub mod httpresponse;
```
在 hs/httpserver/Cargo\.toml 中添加依赖项：
```toml
[dependencies]
http = {path = "../http"}
```
在 hs/httpserver/src 下新建 handler\.rs, router\.rs & server\.rs 分别作为handler, router & server，并在 main\.rs声明它们：
```rust
mod server;
mod router;
mod handler;
```

### server
server 监听进来的 TCP 字节流 并调用 HTTP library 转化为 HTTP 请求 结构体传给 router。因此，它的`run`方法和[00构建TCPServer](../00构建TCPServer.md)里面的代码几乎相同：
文件路径：hs/httpserver/src/server\.rs
```rust
use super::router::Router;
use http::httprequest::HttpRequest;
use std::io::prelude::*;
use std::net::TcpListener;
use std::str;

pub struct Server<'a> {
    socket_addr: &'a str,   // 地址:端口号
}

impl<'a> Server<'a> {
    pub fn new(socket_addr: &'a str) -> Self {
        Server { socket_addr }
    }
    pub fn run(&self) {
        let connection_listener = TcpListener::bind(self.socket_addr).unwrap();
        println!("Running on {}", self.socket_addr);
        
        for stream in connection_listener.incoming() {
            let mut stream = stream.unwrap();
            println!("Connection established");

            let mut read_buf = [0; 200];
            stream.read(&mut read_buf).unwrap();

            // 将 TCP字节流 转化为 HttpRequest，并将它们一并传给 route 方法
            let req: HttpRequest = String::from_utf8(read_buf.to_vec()).unwrap().into();
            Router::route(req, &mut stream);
        }
    }
}
```
在`main`中创建一个服务器：
文件路径：hs/httpserver/src/main\.rs
```rust
use server::Server;

fn main() {
    let server = Server::new("127.0.0.1:3000");
    server.run();
}
```
现在还不能运行，我们并未实现`Router::route`。

### router
router 根据 HTTP 请求 的内容，决定调用哪个 `handler`。另外它还调用了`send_response`方法，把`handle`返回的`HttpResponse`转换为`String`并写入到`stream`中。
文件路径：hs/httpserver/src/router\.rs
```rust
use super::handler::{PageNotFoundHandler, StaticPageHandler, WebServiceHandler, Handler};
use http::{
    httprequest::{self, HttpRequest},
    httpresponse::HttpResponse,
};
use std::io::prelude::*;

pub struct Router;

impl Router {
    pub fn route(req: HttpRequest, stream: &mut impl Write) -> () {
        let mut _res = HttpResponse::default();
        // 根据不同的 Request，使用不同的 handle 以返回对应的页面
        match req.method {
            httprequest::Method::Get => {
                let route: Vec<&str> = req.path.split("/").collect();
                match route[1] {
                    // 一级路径是 api
                    "api" => _res = WebServiceHandler::handle(&req),
                    // 一级路径是其它
                    _ => _res = StaticPageHandler::handle(&req),
                }
            }
            _ => _res = PageNotFoundHandler::handle(&req),
        }
        let _ = _res.send_response(stream);
    }
}
```
这里我们希望有各种`Handler`都使用`handle`这个方法，因此依旧不能运行。

### handler
handler 处理 HTTP 请求，构建 HTTP 响应。

#### Handler trait
响应需要返回一个页面，这个项目的页面文件应该存放在 hs/httpserver 中：
文件路径：hs/httpserver/src/handler\.rs
```rust
use http::{httprequest::HttpRequest, httpresponse::HttpResponse};
use std::env;
use std::fs;

// 通过文件名找到该文件
pub trait Handler {
    fn handle(req: &HttpRequest) -> HttpResponse;
    fn load_file(file_name: &str) -> Option<String> {
        let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
        let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
        let full_path = format!("{}/{}", public_path, file_name);

        let contents = fs::read_to_string(full_path);
        contents.ok()
    }
}
```
所使用的环境变量：
1. `CARGO_MANIFEST_DIR`：
   - 指向当前 Rust 项目的 Cargo\.toml 文件所在的目录。
   - 在这段代码中，`env!("CARGO_MANIFEST_DIR")` 宏被用来获取这个环境变量的值。这个值被用来构建项目的公共文件（如静态文件）的**默认路径**。
2. `PUBLIC_PATH`：
   - 用于指定项目中公共文件（如静态文件）的**路径**。如果这个环境变量被设置了，它将被用来覆盖默认的公共文件路径。
   - `env::var("PUBLIC_PATH")` 函数尝试获取 `PUBLIC_PATH` 环境变量的值。如果这个环境变量没有被设置，`unwrap_or` 方法将使用`default_path`作为默认值。

`load_file`方法的逻辑为：
1. 使用`CARGO_MANIFEST_DIR`环境变量来确定项目的根目录：hs/httpserver。
2. 项目根目录下的 public 子目录作为默认的公共文件路径。
3. 尝试获取`PUBLIC_PATH`环境变量的值，如果这个环境变量被设置了，它将被用来作为公共文件的路径；如果没有设置，就使用默认路径：hs/httpserver/public。
4. 使用构建的路径来加载指定的文件，并尝试将其内容读取为一个`String`类型。

#### PageNotFoundHandler
为`PageNotFoundHandler`实现的`handle`用来返回`404`响应。
文件路径：hs/httpserver/src/handler\.rs
```rust
pub struct PageNotFoundHandler;
impl Handler for PageNotFoundHandler {
    fn handle(_req: &HttpRequest) -> HttpResponse {
        HttpResponse::new("404", None, Self::load_file("404.html"))
    }
}
```

#### StaticPageHandler
为`StaticPageHandler`实现的`handle`用来返回静态页面响应。
文件路径：hs/httpserver/src/handler\.rs
```rust
pub struct StaticPageHandler;
impl Handler for StaticPageHandler {
    fn handle(req: &HttpRequest) -> HttpResponse {
        let path = &req.path;
        let route: Vec<&str> = path.split("/").collect();
        match route[1] {
            // URL: localhost:3000/
            "" => HttpResponse::new("200", None, Self::load_file("index.html")),
            // URL: localhost:3000/health
            "health" => HttpResponse::new("200", None, Self::load_file("health.html")),
            // URL: localhost:3000/?
            path => {
                if let Some(contents) = Self::load_file(path) {
                    let mut map: HashMap<&str, &str> = HashMap::new();
                    if path.ends_with(".css") {
                        map.insert("Content-Type", "text/css");         // 比如: URL: localhost:3000/styles.css
                    } else if path.ends_with(".js") {
                        map.insert("Content-Type", "text/javascript");
                    } else {
                        map.insert("Content-Type", "text/html");
                    }
                    HttpResponse::new("200", Some(map), Some(contents))
                } else {
                    PageNotFoundHandler::handle(req)
                }
            },
        }
    }
}
```

#### WebServiceHandler
`WebServiceHandler`有一个类似于`load_file`的方法：`load_json`，逻辑基本相同，但需要处理 JSON 文件。这就需要在配置文件中添加依赖：
文件路径：hs/httpserver/cargo\.toml
```toml
[dependencies]
serde = {version = "1.0.131", features = ["derive"]}
serde_json = "1.0.72"
```
文件路径：hs/httpserver/src/handler\.rs
```rust
use serde::{Deserialize, Serialize};    // 序列化和与反序列化 JSON 文件
use std::collections::HashMap;

pub struct WebServiceHandler;
impl Handler for WebServiceHandler {
    fn handle(req: &HttpRequest) -> HttpResponse {
        let path = &req.path;
        let route: Vec<&str> = path.split("/").collect();
        if route.len() < 3 {
            return PageNotFoundHandler::handle(req)
        }
        match route[2] {
            // URL: localhost:3000/api/shipping/orders -> orders.json
            "shipping" if route.len() > 3 && route[3] == "orders" => {
                let body = Some(serde_json::to_string(&Self::load_json()).unwrap());
                let mut headers: HashMap<&str, &str> = HashMap::new();
                headers.insert("Content-Type", "application/json");
                HttpResponse::new("200", Some(headers), body)
            }
            _ => PageNotFoundHandler::handle(req)
        }
    }
}
impl WebServiceHandler {
    // 从一个 JSON 文件中加载订单状态数据，并将其解析为 OrderStatus
    fn load_json() -> Vec<OrderStatus> {
        let default_path = format!("{}/data", env!("CARGO_MANIFEST_DIR"));  // hs/httpserver/data
        let data_path = env::var("DATA_PATH").unwrap_or(default_path);
        let full_path = format!("{}/{}", data_path, "orders.json"); // default: hs/httpserver/data/orders.json
        let json_contents = fs::read_to_string(full_path);
        let orders: Vec<OrderStatus> =
            serde_json::from_str(json_contents.unwrap().as_str()).unwrap();
            
        orders
    }
}
```
JSON 文件用于保存订单状态数据，对应的结构体为：
文件路径：hs/httpserver/src/handler\.rs
```rust
#[derive(Serialize, Deserialize)]
struct OrderStatus {
    order_id: i32,
    order_date: String,
    order_status: String,
}
```

### 运行
1. 运行服务器
   ```shell
   cargo run -p httpserver
   Running on 127.0.0.1:3000
   ```
2. 在浏览器输入不同的 URL 测试程序是否正确运行：
    ```
    - localhost:3000/                       返回 index.html
    - localhost:3000/health                 返回 health.html
    - localhost:3000/api/shipping/orders    返回 orders.json
    - localhost:3000/styles.css             返回 styles.css
    - localhost:3000/foo                    返回 404.html
    ```
# axum 框架复现 HttpServer
使用框架会让工作量大大减少。

## axum 框架
axum 是一款 Web 应用程序框架。需要与 Tokio h Hyper 配合使用。
Tokio 是一个事件驱动的非阻塞 I/O 平台，用于异步编写 使用 Rust 的应用程序。
hyper 是一个较低级别的 HTTP 库，用于为库和应用程序构建块。

### Hello, Axum!
创建项目：
```shell
cargo new hello_axum
cd hello_axum
code .
```
在 cargo\.toml 中添加依赖项：
```toml
[dependencies]
axum = "0.7"
tokio = { version = "1.0", features = ["full"] }
```
在主程序中编写代码：
```rust
use axum::{
    routing::get,
    Router,
};

#[tokio::main]
async fn main() {
    // build our application with a single route
    let router = Router::new().route("/", get(|| async { "Hello, Axum!" }));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, router).await.unwrap();
}
```
代码解释：
- `#[tokio::main]`: 属性宏，用于标记 main 函数是异步的，并且使用 tokio 作为异步运行时。
- `async fn`: 定义异步函数。
- `Router::new().route()`: 创建一个新的 Router 实例，并添加路由规则。
  本程序中，当接收到对根路径`/`的`GET`请求时，返回`Hello, World!`字符串。
  `route()`方法后还可以`.route()`继续添加多条路由规则。
- `axum::serve`: 启动 Server。
- `await`: 等待异步操作完成。

`cargo run`运行，到浏览器访问`localhost:3000`，在页面中会显示`Hello, Axum!`。

## 复现 HttpServer
### 创建项目
```shell
cargo new hs_axum
cd hs_axum
code .
```

### 添加依赖
在 cargo\.toml 中添加依赖项：
```toml
[dependencies]
axum = "0.7"
tokio = { version = "1.0", features = ["full"] }
```

### 构建 server, router
在主程序中编写代码，根据之前项目路由规则添加路由：
```rust
use axum::{
    routing::get,
    Router,
};

#[tokio::main]
async fn main() {
    let api_router = Router::new()
        .route("/shipping/orders", get(orders_handler))
        .fallback(fallback_handler);
    let route = Router::new()
        .nest("/api", api_router)
        .route("/", get(index_handler))
        .route("/health", get(health_handler))
        .fallback(fallback_handler);

    let addr = "127.0.0.1:3000";
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!("Running on {}", addr);

    axum::serve(listener, route).await.unwrap();
}
```
代码解释：
- `nest`: 对指定路径添加路由规则
  本程序中，`api_router`中定义的所有路由都会自动加上`/api`前缀。

### 构建 handler
这里我们简单构建 handler，不同的 handler 会把对应文件的内容放到响应的 Body 中返回。
```rust
async fn orders_handler() -> Response {
    let content = include_str!("data/orders.json");
    Response::new(Body::from(content))
}

async fn index_handler() -> Response {
    let content = include_str!("public/index.html");
    Response::new(Body::from(content))
}

async fn health_handler() -> Response {
    let content = include_str!("public/health.html");
    Response::new(Body::from(content))
}

async fn fallback_handler() -> Response {
    let content = include_str!("public/404.html");
    Response::new(Body::from(content))
}
```

### 运行
1. 运行服务器
   ```shell
   cargo run
   Running on 127.0.0.1:3000
   ```
2. 在浏览器输入不同的 URL 测试程序是否正确运行：
    ```
    - localhost:3000/                       返回 index.html
    - localhost:3000/health                 返回 health.html
    - localhost:3000/api/shipping/orders    返回 orders.json
    - localhost:3000/foo                    返回 404.html
    ```

可以看出：对比我们之前的 HttpServer，使用框架实现只用了很少的代码。
因此做程序既要 *脚踏实地* 了解程序的运行原理，也要 *站在巨人的肩膀上* 更快速地构建程序。
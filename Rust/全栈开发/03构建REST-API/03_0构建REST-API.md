# 构建REST API

## REST API
不了解 REST API 的可以先阅读这篇文章：[一文彻底弄懂REST API](https://zhuanlan.zhihu.com/p/536437382)

REST API 使用标准的HTTP方法来执行不同的操作：
- 查：GET，用于检索资源。
- 增：POST，用于创建新资源。
- 改：PUT，用于更新现有资源。
- 删：DELETE，用于删除资源。

然而，直接使用 PUT 和 DELETE 存在安全漏洞。因为它们可以直接修改资源，如果被恶意用户利用，可能会导致数据泄露或破坏。常用的就是 POST 来替代实现 PUT、DELETE 的方法。

## 准备工作
我们在[02](../02使用axum框架复现HttpServer)中简单复现了 HttpServer，现在我们依旧使用这个 package。
1. 在 src 下新建 handlers\.rs, models\.rs, state\.rs, bin/teacher-server\.rs 。
2. 在 cargo\.toml 中添加：
    ```toml
    [package]
    default-run = "teacher-service"
    [[bin]]
    name = "teacher-service"
    ```
   使程序`cargo run`默认运行 bin/teacher-server\.rs 。
3. 在 teacher-server\.rs 中创建默认路由，编写服务器运行程序同时引入 mod：
    ```rust
    use axum::{routing::get, Router};

    #[path = "../handlers.rs"]
    mod handlers;

    #[path = "../state.rs"]
    mod state;

    use state::AppState;

    #[tokio::main]
    async fn main() {
        let app = Router::new().route("/", get(|| async { "Hello, Axum!" }));

        let addr = "127.0.0.1:3000";
        let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
        println!("Running on {}", addr);

        axum::serve(listener, app).await.unwrap();
    }
    ```
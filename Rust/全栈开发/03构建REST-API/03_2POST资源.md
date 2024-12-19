## POST 资源
我们希望教师可以通过 API 添加（POST）课程。

### 数据模型
#### 依赖项
- `serde`: 对 Rust 数据结构进行序列化和反序列化框架。
- `chrono`: 处理时间相关操作。

文件路径：hello_axum/cargo\.toml
```toml
chrono = { version = "0.4", features = ["serde"] }
serde = { version = "1.0", features = ["derive"] }
```

#### 定义课程数据模型
课程中应有以下属性：
- teacher_id：表示提供课程的老师，是老师的唯一标识符。
- id：课程的唯一标识符。在这里，id 对于每个 teacher_id 唯一，也就是不同的老师可以有相同的课程 id。
- name：老师提供的课程名称。
- posted_time：课程的发布时间。

文件路径：hello_axum/src/models\.rs
```rust
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
 
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Course {
    pub teacher_id: i32,
    pub id: Option<usize>,
    pub name: String,
    pub posted_time: Option<NaiveDateTime>,
}
```

#### 存储课程
程序状态可以在不同线程间共享，把课程存储在程序状态中是一种简单的选择。当然，它是保存在内存里的，服务器一旦关闭，就会丢失数据。目前就让我们这样做吧：
文件路径：hello_axum/src/statu\.rs
```rust
use std::sync::Mutex;
use super::models::Course;

pub struct AppState {
    pub health_check_response: String,
    pub visit_count: Mutex<u32>,
    pub courses: Mutex<Vec<Course>>,
}
```
文件路径：hello_axum/src/bin/teacher-service\.rs
```rust
// --snip--

#[path = "../models.rs"]
mod models;

#[tokio::main]
async fn main() {
    let app_state = Arc::new(AppState {
        health_check_response: "I'm good. You've already asked me".to_string(),
        visit_count: Mutex::new(0),
        courses: Mutex::new(vec![]),
    });
    // --snip--
}
```

#### 运行
为了保证我们的修改没有让程序出现问题，我们每编写一些代码就必须运行一次。
1. 运行服务器
   ```shell
   cargo run
   Running on 127.0.0.1:3000
   ```
2. 在浏览器输入`localhost:3000/health`，确保健康检查正确运行。

### 发布课程
#### 添加路由
把`localhost:3000/courses/` 作为发布课程的路径，使用`post`方法：
文件路径：hello_axum/src/bin/teacher-service\.rs
```rust
    let app = Router::new()
        .route("/health", get(health_check_handler))
        .route("/courses/", post(new_course))
        .layer(Extension(app_state));
```
#### 编写 handler
文件路径：hello_axum/src/bin/handlers\.rs
```rust
use super::models::Course;
use chrono::Utc;
use serde_json::{json, Value};
use super::state::AppState;
use axum::{extract::Extension, Json};
use std::sync::Arc;

pub async fn new_course(
    Extension(state): Extension<Arc<AppState>>,
    Json(new_course): Json<Course>,
) -> Json<Value> {
    println!("Received new course");
    let course_count = state
        .courses
        .lock()
        .unwrap()
        .clone()
        .into_iter()
        .filter(|course| course.teacher_id == new_course.teacher_id)
        .count();
    let new_course = Course {
        teacher_id: new_course.teacher_id,
        id: Some(course_count + 1),
        name: new_course.name.clone(),
        posted_time: Some(Utc::now().naive_utc()),
    };
    state.courses.lock().unwrap().push(new_course);
    Json(json!({"message": "Course Added"}))
}
```
此 handler 执行以下操作：
1. 使用[提取器](https://docs.rs/axum/0.7.9/axum/extract/index.html#the-order-of-extractors)获取程序 State 和请求 Body。
2. 通过计算老师的现有课程数量并递增 1 来生成新的课程 id
3. 创建新的课程实例并添加到 AppState

**提取器**：可以从请求中提取数据的类型和特征。
```rust
axum::extract::{Request, Json, Path, Extension, Query},
    http::header::HeaderMap,
    body::{Bytes, Body}
```
这些是常用的提取器，使用它们时，应该注意顺序：
提取器在函数参数中，是按参数的顺序运行的，因此，根据请求报文格式，提取器必须按照`method->path->headers->state->body`的顺序填入。`new_course`的传入参数为：
```rust
Extension(state): Extension<Arc<AppState>>, // state
Json(new_course): Json<Course>,             // body
```
顺序为：`state->body`。倘若我们换了一下它们的位置，编译器就会报错了。
`new_course`还需要使用`serde_json`包，它提供的`json!`宏使用非常自然的 JSON 语法构建对象，对象类型为`Value`。添加依赖：
文件路径：hello_axum/cargo\.toml
```toml
serde_json = "1.0"
```

#### 编写测试
文件路径：hello_axum/src/bin/teacher-service\.rs
```rust
#[cfg(test)]
mod test {
    use axum_test::TestServer;
    use axum::{http::StatusCode, routing::post, Router};

    use super::*;
    use std::sync::Mutex;

    #[tokio::test]
    async fn post_course_test() {
        // 课程实例
        let course = Course {
            teacher_id: 1,
            name: "Test course".into(),
            id: None,
            posted_time: None,
        };
        // 程序状态
        let app_state = Arc::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            courses: Mutex::new(vec![]),
        });
        // 路由器
        let app = Router::new()
            .route("/courses/", post(new_course))
            .layer(Extension(app_state));
        // 模拟服务器
        let server = TestServer::new(app).unwrap();
        // 响应
        let response = server.post("/courses/")
            .json(&course).await;

        assert_eq!(response.status_code(), StatusCode::OK);
    }
}
```
为了测试，我们还需要使用`axum_test`提供的模拟服务器，相比于服务器，我们可以直接在测试程序中就发起请求并得到响应。
`axum_test`是一个用于为使用`axum`编写的 Web 服务器编写测试的库。添加依赖：
文件路径：hello_axum/cargo\.toml
```toml
axum-test = "16.4"
```
在终端输入以下命令运行测试：
```shell
cargo test
```
测试通过。

#### 运行
在终端输入以下命令运行服务器：
```shell
cargo run
```
显示`Running on 127.0.0.1:3000`表明服务器开始运行。
然后新打开一个`shell`当作客户端，输入以下命令对服务器发送 POST 请求：
```shell
PS hello_axum > curl `
>> -Uri "http://localhost:3000/courses/" `
>> -Headers @{"Content-Type"="application/json"} `
>> -Body '{"teacher_id":1,"name":"First course"}' `
>> -Method Post
```
- 使用``` ` ```再敲击`Enter`键可以在 PowerShell 换行。

我们在服务器端有返回的消息：`Received new course`，同时，在客户端，会得到服务器发来的响应：
```shell
StatusCode        : 200
StatusDescription : OK
Content           : {"message":"Course Added"}
RawContent        : HTTP/1.1 200 OK
                    Content-Length: 26
                    Content-Type: application/json
                    Date: Thu, 19 Dec 2024 07:38:53 GMT

                    {"message":"Course Added"}
Forms             : {}
Headers           : {[Content-Length, 26], [Content-Type, application/json], [Date, Thu, 19 Dec 2024 07:38:53 GMT]}
Images            : {}
InputFields       : {}
Links             : {}
ParsedHtml        : mshtml.HTMLDocumentClass
RawContentLength  : 26
```
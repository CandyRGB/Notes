# Web 项目中使用数据库
在 [03](./03构建REST-API) 中，我们使用内存进行存储课程，这是一种极其浪费内存的行为，因此我们要对其进行改进，把数据存储到数据库中。

## 准备工作
1. 打开项目 hello_axum
2. 在 cargo\.toml 中添加 **依赖项**：
    ```toml
    [dependencies]
    dotenvy = "0.15"
    sqlx = {version = "0.6.2", features = [
        "postgres",
        "runtime-tokio-native-tls",
        "macros",
        "chrono",
    ]}
    ```

## 连接 PostgreSQL
### 环境变量配置
在项目根目录下新建 \.env 文件，输入以下内容，配置 **环境变量**。
```
DATABASE_URL=postgres://postgres:password@127.0.0.1:5432/ezytutors
```
- `postgres://postgres` 是数据库所在的目录
- `password` 是你的密码
- `127.0.0.1:5432` 是数据库端口
- `ezytutors` 是课程表名称

### 创建数据库连接池
在主程序创建数据库连接池并把它保存在 state 里。
文件路径：hello_axum/src/bin/teacher-service\.rs
```rust
// --snip--

use dotenvy::dotenv;
use sqlx::postgres::PgPool;
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL 没有在 .env 文件里设置");

    let db_pool = PgPool::connect(&database_url).await.unwrap();

    let app_state = Arc::new(AppState {
        health_check_response: "I'm good. You've already asked me".to_string(),
        visit_count: Mutex::new(0),
        // courses: Mutex::new(vec![]),
        db: db_pool,
    });

    // --snip--
}
```
文件路径：hello_axum/src/state\.rs
```rust
// 应用程序的状态
pub struct AppState {
    pub health_check_response: String,
    pub visit_count: Mutex<u32>,
    // pub courses: Mutex<Vec<Course>>,
    pub db: PgPool,
}
```

## 数据行解构
为 `Course` 添加 `sqlx::FromRow` 这个 trait 可以实现对数据行的解构，不过，我们的 `course.id` 应该改为 `Option<i32>`类型。
```rust
#[derive(Deserialize, Serialize, Debug, Clone, sqlx::FromRow)]
pub struct Course {
    pub teacher_id: i32,
    pub id: Option<i32>,
    pub name: String,
    pub posted_time: Option<NaiveDateTime>,
}
```

## 实现对数据库的操作
在 src 下新建 db_access\.rs 用于实现数据库的操作。同时在 teacher-service\.rs 引入 mod：
```rust
#[path = "../db_access.rs"]
mod db_access;
```
需要使用的函数或方法：
- `sqlx::query_as`：对数据库进行操作并解构数据行。
- `bind`：SQL 查询语句中占位符的内容。

### 添加新课程
这里我们不使用顺序 `id` 添加课程，并且希望在添加课程后能返回它。
文件路径：hello_axum/src/db_access\.rs
```rust
use super::models::*;
use sqlx::postgres::PgPool;

pub async fn post_new_course_db(pool: &PgPool, new_course: Course) -> Course {
    let course: Course = sqlx::query_as(
        r#"
        INSERT INTO ezy_course_c4 (id, teacher_id, name)
        VALUES ($1,$2,$3)
        RETURNING id, teacher_id, name, posted_time"#
    )
    .bind(new_course.id)
    .bind(new_course.teacher_id)
    .bind(new_course.name)
    .fetch_one(pool)
    .await
    .unwrap();
    course
}
```

### 获取某个老师的所有课程
文件路径：hello_axum/src/db_access\.rs
```rust
pub async fn get_courses_of_teacher_db(pool: &PgPool, teacher_id: i32) -> Vec<Course> {
    let select_courses: Vec<Course> = sqlx::query_as(
        r#"SELECT id, teacher_id, name, posted_time
        FROM ezy_course_c4
        WHERE teacher_id = $1"#
    )
    .bind(teacher_id)
    .fetch_all(pool)
    .await
    .unwrap();
    select_courses
}
```

### 获取某个课程的信息
文件路径：hello_axum/src/db_access\.rs
```rust
pub async fn get_course_detail_db(pool: &PgPool, teacher_id: i32, course_id: i32) -> Result<Course, sqlx::Error> {
    let select_course: Course = sqlx::query_as(
        r#"SELECT id, teacher_id, name, posted_time
        FROM ezy_course_c4
        WHERE teacher_id = $1 and id = $2"#,
    )
    .bind(teacher_id)
    .bind(course_id)
    .fetch_one(pool)
    .await?;
    Ok(select_course)
}
```
有可能并没有查询的这个课程，因此这个函数返回 `Result` 类型，用于在外部处理错误。

## 修改 handler
原本 handler 都是操作内存中的数据，现在它们使用数据库操作的函数。当然，我们需要导入它们：
文件路径：hello_axum/src/handlers\.rs
```rust
use super::db_access::*;
```

### 添加新课程
文件路径：hello_axum/src/handlers\.rs
```rust
pub async fn new_course(
    Extension(state): Extension<Arc<AppState>>,
    Json(new_course): Json<Course>,
) -> Json<Value> {
    let course = post_new_course_db(&state.db, new_course).await;
    Json(json!({"message": "Course Added", "new course": course}))
}
```

### 获取某个老师的所有课程
文件路径：hello_axum/src/handlers\.rs
```rust
pub async fn courses_of_teacher(
    Path(teacher_id): Path<i32>,
    Extension(state): Extension<Arc<AppState>>
) -> Json<Value> {
    let filtered_courses = get_courses_of_teacher_db(&state.db, teacher_id).await;
    if filtered_courses.len() > 0 {
        Json(json!(filtered_courses))
    } else {
        Json(json!("No Courses found for teacher"))
    }
}
```

### 获取某个课程的信息
文件路径：hello_axum/src/handlers\.rs
```rust
pub async fn course_detail(
    Path((teacher_id, course_id)): Path<(i32, i32)>,
    Extension(state): Extension<Arc<AppState>>
) -> Json<Value> {
    let select_course = get_course_detail_db(&state.db, teacher_id, course_id).await;
    match select_course {
        Ok(course) => Json(json!(course)),
        Err(_) => Json(json!({"error": "NotFoundCourse"}))
    }
}
```

### 修改 handler 的测试
在原先的基础上，我们只需要修改每个测试的 state ，也就是为其添加数据库连接池。不过，对于 post 测试，由于它会在数据库里添加课程，所以我们每次运行测试时，都需要保证它们的`teacher_id`和`course_id`的组合是不同的。
文件路径：hello_axum/src/handlers\.rs
```rust
#[cfg(test)]
mod test {
    // --snip--
    use dotenvy::dotenv;
    use sqlx::PgPool;
    use std::env;

    #[tokio::test]
    async fn post_course_test() {
        let course = Course {
            teacher_id: 1,
            name: "Test course".into(),
            id: Some(3),
            posted_time: None,
        };
        dotenv().ok();

        let database_url = env::var("DATABASE_URL")
            .expect("DATABASE_URL 没有在 .env 文件里设置");
    
        let db_pool = PgPool::connect(&database_url).await.unwrap();
    
        let app_state = Arc::new(AppState {
            health_check_response: "I'm good. You've already asked me".to_string(),
            visit_count: Mutex::new(0),
            // courses: Mutex::new(vec![]),
            db: db_pool,
        });
        // --snip--
    }
    #[tokio::test]
    async fn get_teacher_courses_test() {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL")
            .expect("DATABASE_URL 没有在 .env 文件里设置");
    
        let db_pool = PgPool::connect(&database_url).await.unwrap();
    
        let app_state = Arc::new(AppState {
            health_check_response: "I'm good. You've already asked me".to_string(),
            visit_count: Mutex::new(0),
            // courses: Mutex::new(vec![]),
            db: db_pool,
        });
        // --snip--
    }

    #[tokio::test]
    async fn get_course_test() {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL")
            .expect("DATABASE_URL 没有在 .env 文件里设置");
    
        let db_pool = PgPool::connect(&database_url).await.unwrap();
    
        let app_state = Arc::new(AppState {
            health_check_response: "I'm good. You've already asked me".to_string(),
            visit_count: Mutex::new(0),
            // courses: Mutex::new(vec![]),
            db: db_pool,
        });
        // --snip--
    }
}
```
在终端输入以下命令运行测试：
```shell
cargo test
```
测试通过。

## 运行
运行服务器：
```shell
PS hello_axum > cargo run
Running on 127.0.0.1:3000
```
1. 健康检查：在浏览器输入`localhost:3000/health`，确保健康检查正确运行。
2. 添加课程：新打开一个`shell`当作客户端，输入以下命令对服务器发送 POST 请求。
    ```shell
    PS hello_axum > curl `
    >> -Uri "http://localhost:3000/courses" `
    >> -Headers @{"Content-Type"="application/json"} `
    >> -Body '{"teacher_id":1,"name":"Just course"，"id":7}' `
    >> -Method Post
    ```
    我们在客户端，会得到服务器发来的响应：
    ```
    StatusCode        : 200
    StatusDescription : OK
    Content           : {"message":"Course Added","new cour
                        se":{"id":7,"name":"Just course","p 
                        osted_time":"2025-01-13T06:56:37.81 
                        3505","teacher_id":1}}
    RawContent        : HTTP/1.1 200 OK
                        Content-Length: 128
                        Content-Type: application/json      
                        Date: Mon, 13 Jan 2025 06:56:37 GMT 

                        {"message":"Course Added","new cour 
                        se":{"id":7,"name":"Just course"," 
                        posted_time":"2025-01...
    Forms             : {}
    Headers           : {[Content-Length, 128], [Content-Ty 
                        pe, application/json], [Date, Mon,  
                        13 Jan 2025 06:Images            : {}
    InputFields       : {}
    Links             : {}
    ntLength  : 128
    ```
3. 查询某个老师的所有课程：打开浏览器访问 `http://localhost:3000/courses/1`，就会得到下面这个 json 文件（`post_time` 由提交课程的时间决定）：
    ```json
    [
        {
            "id": 1,
            "name": "First course",
            "posted_time": "2020-12-17T05:40:00",
            "teacher_id": 1
        },
        {
            "id": 2,
            "name": "Second course",
            "posted_time": "2020-12-18T05:45:00",
            "teacher_id": 1
        },
        {
            "id": 3,
            "name": "Test course",
            "posted_time": "2025-01-13T06:49:38.355371",
            "teacher_id": 1
        },
        {
            "id": 7,
            "name": "Just course",
            "posted_time": "2025-01-13T06:56:37.813505",
            "teacher_id": 1
        }
    ]
    ```
    哎，假如我们访问无课程的老师或者不存在的 `teacher_id` 呢？比如访问 `http://localhost:3000/courses/5`，就会得到：
    ```json
    "No Courses found for teacher"
    ```
4. 查询某个课程：打开浏览器访问 `http://localhost:3000/courses/1/7`，就会得到下面这个 json 文件（`post_time` 由提交课程的时间决定）：
    ```json
    {
        "id": 7,
        "name": "Just course",
        "posted_time": "2025-01-13T06:56:37.813505",
        "teacher_id": 1
    }
    ```
    哎，假如我们访问不存在的课程呢？比如访问 `http://localhost:3000/courses/1/8`，就会得到：
    ```json
    {
        "error": "NotFoundCourse"
    }
    ```
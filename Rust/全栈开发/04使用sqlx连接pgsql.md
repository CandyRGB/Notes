# 使用 sqlx 连接 PostgreSQL

## 准备工作
1. 在 shell 中输入以下命令以 **新建** 项目。
    ```shell
    cargo new linksql
    ```
2. 进入项目，在 cargo\.toml 中添加 **依赖项**：
    ```toml
    [dependencies]
    axum = "0.7"
    tokio = { version = "1.0", features = ["full"] }
    chrono = { version = "0.4", features = ["serde"] }
    dotenvy = "0.15"
    sqlx = {version = "0.6.2", features = [
        "postgres",
        "runtime-tokio-native-tls",
        "macros",
        "chrono",
    ]}
    ```
    - `dotenvy`: 用于在项目根目录下的 .env 文件中加载 **环境变量**。
    - `sqlx`: 异步 SQL 执行库。
3. 在 src 下创建 database.sql 数据库文件，在 `ezy_course_c4` 中保存课程，课程的结构与我们之前创建的 `struct course` 相同：
    ```sql
    /* 如果表已经存在，则删除表 */
    drop table if exists ezy_course_c4;
    /* 创建一个表 */
    /* 注意：在最后一个字段后不要加逗号 */
    create table ezy_course_c4
    (
        id serial primary key,
        teacher_id INT not null,
        name varchar(140) not null,
        posted_time TIMESTAMP default now()
    );
    
    /* 加载测试用的种子数据 */
    insert into ezy_course_c4
    (id, teacher_id, name, posted_time)
    values(1, 1, 'First course', '2020-12-17 05:40:00');
    insert into ezy_course_c4
    (id, teacher_id, name, posted_time)
    values(2, 1, 'Second course', '2020-12-18 05:45:00');
    ```
    我们在 `ezy_course_c4` 添加了 2 个课程 `First course`, `Second course`。

## 连接 PostgreSQL
### pgAdmin 配置
确保你已经 **安装** 了 pgAdmin 并且知道你在安装时设置的 password。
打开 pgAdmin，点击 Servers，输入密码连接到服务器。在这个目录下，右击 **数据库**(Database) 创建新的 库，设置数据库名称为 **ezytutors**，保存，右击 **ezytutors**，选择 **PSQL工具**，输入以下命令将项目里的 sql 文件连接到数据库。
```shell
\i 项目路径/linksql/src/database.sql
```
再次右击 **ezytutors**，选择 **查询工具**，输入以下命令，运行以获得 ezy_course_c4 中保存的所有 course。
```shell
SELECT * FROM ezy_course_c4
```
<details>
    <summary>[Output]</summary>
    <pre><code>
course_id | tutor_id |  course_name  |     posted_time
----------+----------+---------------+---------------------
        1 |        1 | First course  | 2020-12-17 05:40:00
        2 |        1 | Second course | 2020-12-18 05:45:00
(2 行记录)</code></pre>
</details>
如果你也输出了以上内容，说明连接成功了。

### 环境变量配置
在项目根目录下新建 \.env 文件，输入以下内容，配置 **环境变量**。
```
DATABASE_URL=postgres://postgres:password@127.0.0.1:5432/ezytutors
```
- `postgres://postgres` 是数据库所在的目录
- `password` 是你的密码
- `127.0.0.1:5432` 是数据库端口
- `ezytutors` 是课程表名称

### 连接获取数据库中的内容
下面我们编写程序以获取 id=1 的课程，这需要两个步骤：
1. 连接数据库
2. SQL 查询

```rust
use chrono::NaiveDateTime;
use dotenvy::dotenv;
use sqlx::postgres::PgPool;
use std::env;
use std::io;

#[derive(Debug)]
pub struct Course {
    pub id: i32,
    pub teacher_id: i32,
    pub name: String,
    pub posted_time: Option<NaiveDateTime>,
}

#[tokio::main]
async fn main() -> io::Result<()> {
    // 加载 .env 文件中的环境变量，如果加载失败则忽略错误。
    dotenv().ok();

    // 从环境变量中获取 DATABASE_URL
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL 没有在 .env 文件里设置");

    // 使用 DATABASE_URL 创建一个数据库连接池
    let db_pool = PgPool::connect(&database_url).await.unwrap();

    // SQL 查询 id = 1 的 course
    let course_rows = sqlx::query!(
        r#"select id, teacher_id, name, posted_time from ezy_course_c4 where id = $1"#,
        1
    )
    .fetch_all(&db_pool)
    .await
    .unwrap();
    let mut courses_list = vec![];
    for course_row in course_rows {
        courses_list.push(Course {
            id: course_row.id,
            teacher_id: course_row.teacher_id,
            name: course_row.name,
            posted_time: Some(chrono::NaiveDateTime::from(course_row.posted_time.unwrap())),
        })
    }
    println!("Courses = {:?}", courses_list);
    Ok(())
}
```
在使用 SQL 查询 id = 1 的 course 时，我们使用了 `sqlx::query!(...)` 宏构建查询，并使用 `fetch_all()` 执行 SQL 查询，这有些像**迭代器**，它本身是**惰性**的，当调用像 `collect()` 这种方法时才会执行。

## 运行
在命令行输入 `cargo run` 以运行，正如期望一样，我们得到了 id=1 的课程信息。
```
Courses = [Course { id: 1, teacher_id: 1, name: "First course", posted_time: Some(2020-12-17T05:40:00) }]
```
## 解析 HTTP 请求

### workspace
创建项目：
```shell
cargo new hs
cd cs
cargo new http
cargo new httpserver
code .
```
更改 hs 目录下的 cargo\.toml：
```toml
[workspace]
members = ["http", "httpserver"]
```

### 需要实现的数据结构
根据请求报文结构，我们需要创建以下数据结构：
|数据结构名称|数据类型|描述|
|:-:|:-:|:-:|
|HttpRequest|struct|表示 HTTP 请求|
|Method|enum|指定所允许的 HTTP 方法|
|Path|&str|指定所允许的 HTTP 路径|
|Version|enum|指定所允许的 HTTP 版本|

### 需要实现的 Trait
|Trait|用途|
|:-|:-|
|`From<&str>`|把`&str`转化为`HttpRequest`中对应字段的类型|
|`Debug`|打印调试信息|
|`PartialEq`|解析内容与测试内容的比较|

### 实现
在 hs/http/src 下新建 httprequest\.rs ，作为 HTTP 请求模块。

#### Method
编写`Method`并编写对应测试：
文件路径：hs/http/src/httprequest\.rs
```rust
#[derive(Debug, PartialEq)]
pub enum Method {
    Get,
    Post,
    Uninitialized,
}

impl From<&str> for Method {
    fn from(s: &str) -> Self {
        match s {
            "GET" => Self::Get,
            "POST" => Self::Post,
            _ => Self::Uninitialized,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_method_into() {
        let m: Method = "GET".into();
        assert_eq!(m, Method::Get);
    }
}
```
`into`方法可以把`T`转换到实现了`From<T>`的类型。
运行测试：
```shell
cargo test -p http
```
测试通过。

#### Version
以同样的形式编写`Version`并在测试模块加入对应测试：
文件路径：hs/http/src/httprequest\.rs
```rust
#[derive(Debug, PartialEq)]
pub enum Version {
    V1_1,
    V2_0,
    Uninitialized,
}

impl From<&str> for Version {
    fn from(s: &str) -> Self {
        match s {
            "HTTP/1.1" => Self::V1_1,
            "HTTP/2.0" => Self::V2_0,
            _ => Self::Uninitialized,
        }
    }
}

    #[test]
    fn test_version_into() {
        let v: Version = "HTTP/1.1".into();
        assert_eq!(v, Version::V1_1);
    }
```
运行测试：
```shell
cargo test -p http
```
测试通过。

#### Path
文件路径：hs/http/src/httprequest\.rs
```rust
type Path = String;
```

#### HttpRequest
以同样的形式编写`HttpRequest`：
文件路径：hs/http/src/httprequest\.rs
```rust
#[derive(Debug)]
pub struct HttpRequest {
    pub method: Method,
    pub path: Path,
    pub version: Version,
    pub headers: HashMap<String, String>,
    pub msg_body: String,
}

impl From<String> for HttpRequest {
    fn from(s: String) -> Self {
        let mut parsed_method = Method::Uninitialized;
        let mut parsed_path = Path::new();
        let mut parsed_version = Version::V1_1;
        let mut parsed_headers: HashMap<String, String> = HashMap::new();
        let mut parsed_msg_body = String::new();

        let mut should_check_req_line = true;   // 请求行 匹配标志
        let mut should_check_header_line = true;// header 匹配标志
        for line in s.lines().filter(|line| !line.is_empty()) {
            if should_check_req_line && line.contains("HTTP") {
                (parsed_method, parsed_path, parsed_version) = process_req_line(line);
                should_check_req_line = false;  // 匹配成功过 请求行 就不再匹配
            } else if should_check_header_line && line.contains(": ") {
                let (k, v) = process_header_line(line);
                parsed_headers.insert(k, v);
            } else {
                // 请求中的 headers 是紧接在 请求行 后，并且 header 是连续的行，因此匹配到其它行就一定不会有 header
                should_check_header_line = false;
                parsed_msg_body = parsed_msg_body + line;
            }
        }

        Self {
            method: parsed_method,
            path: parsed_path,
            version: parsed_version,
            headers: parsed_headers,
            msg_body: parsed_msg_body,
        }
    }
}
```
我们假想了函数`process_req_line`和`process_header_line`，分别用来解析 请求行 与 header，并返回对应的类型。
接下来实现它们：
文件路径：hs/http/src/httprequest\.rs
```rust
fn process_req_line(line: &str) -> (Method, Path, Version) {
    let mut words = line.split_whitespace();
    let method = words.next().unwrap();
    let path = words.next().unwrap();
    let version = words.next().unwrap();
    (method.into(), path.to_string(), version.into())
}

fn process_header_line(line: &str) -> (String, String) {
    let mut words = line.split(": ");
    let k = words.next().unwrap();
    let v = words.next().unwrap();
    (k.to_string(), v.to_string())
}
```
编写测试：
```rust
#[test]
    fn test_read_http() {
        let http_request: HttpRequest = String::from("\
GET /sleep HTTP/1.1
Host: localhost:7878
Connection: keep-alive
User-Agent: Edg/131.0.0.0").into();
        let mut headers = HashMap::new();
        headers.insert("Host".into(), "localhost:7878".into());
        headers.insert("Connection".into(), "keep-alive".into());
        headers.insert("User-Agent".into(), "Edg/131.0.0.0".into());

        assert_eq!(Method::Get, http_request.method);
        assert_eq!("/sleep".to_string(), http_request.path);
        assert_eq!(Version::V1_1, http_request.version);
        assert_eq!(headers, http_request.headers);
        assert_eq!(String::new(), http_request.msg_body);
    }
```
运行测试：
```shell
cargo test -p http
```
测试通过。
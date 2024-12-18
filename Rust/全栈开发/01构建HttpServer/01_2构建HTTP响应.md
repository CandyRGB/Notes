## 构建 HTTP 响应
在 hs/http/src 下新建 httpresponse\.rs ，作为 HTTP 响应模块。

## 需要实现的数据结构
我们需要返回响应报文(`&str`)，根据响应报文结构，我们需要创建下面这个结构体：
文件路径：hs/http/src/httpresponse\.rs
```rust
#[derive(Debug, PartialEq, Clone)]
pub struct HttpResponse<'a> {
    version: &'a str,
    status_code: &'a str,
    status_text: &'a str,
    headers: Option<HashMap<&'a str, &'a str>>,
    body: Option<String>,
}
```

## 需要实现的 Trait
|Trait|用途|
|:-|:-|
|`From<HttpResponse>`|把`HttpResponse`转化为`&str`中对应字段的类型|
|`Default`|指定成员的默认值|

## 需要实现的方法
|Trait|用途|
|:-|:-|
|`new`|使用默认值创建一个新的结构体|
|`send_response`|构建响应，将原始字节通过 TCP 传送|
|`getter`方法|获得成员的值返回`&str`或`String`类型|

## 为`HttpResponse<'a>`实现 Trait 和 方法

### Default & new
文件路径：hs/http/src/httpresponse\.rs
```rust
use std::collections::HashMap;

impl<'a> Default for HttpResponse<'a> {
    fn default() -> Self {
        Self {
            version: "HTTP/1.1",
            status_code: "200",
            status_text: "OK",
            headers: None,
            body: None,
        }
    }
}

impl<'a> HttpResponse<'a> {
    pub fn new(
        status_code: &'a str,
        headers: Option<HashMap<&'a str, &'a str>>,
        body: Option<String>,
    ) -> Self {
        let mut response = Self::default();
        if status_code != "200" {
            response.status_code = status_code;
        }
        response.headers = match &headers {
            Some(_h) => headers,
            None => {
                let mut h = HashMap::new();
                h.insert("Content-Type", "Text/html");
                Some(h)
            }
        };
        response.status_text = match response.status_code {
            "200" => "OK",
            "400" => "Bad Request",
            "404" => "Not Found",
            "500" => "Internal Server Error",
            _ => "Not Found",
        };
        response.body = body;

        response
    }
}
```
为`new`编写测试单元：
文件路径：hs/http/src/httpresponse\.rs
```rust
// 测试 状态码 为 200 的情况
#[test]
fn test_res_new_200() {
    let res_actual = HttpResponse::new("200", None, Some("xxxx".to_string()));
    let res_expected = HttpResponse {
        version: "HTTP/1.1",
        status_code: "200",
        status_text: "OK",
        headers: {
            let mut h = HashMap::new();
            h.insert("Content-Type", "Text/html");
            Some(h)
        },
        body: Some("xxxx".to_string()),
    };
    assert_eq!(res_actual, res_expected);
}

// 测试 状态码 为 404 的情况
#[test]
fn test_res_new_404() {
    let res_actual = HttpResponse::new("404", None, Some("xxxx".to_string()));
    let res_expected = HttpResponse {
        version: "HTTP/1.1",
        status_code: "404",
        status_text: "Not Found",
        headers: {
            let mut h = HashMap::new();
            h.insert("Content-Type", "Text/html");
            Some(h)
        },
        body: Some("xxxx".to_string()),
    };
    assert_eq!(res_actual, res_expected);
}
```
运行测试：
```shell
cargo test -p http
```
测试通过。

### getter 方法
文件路径：hs/http/src/httpresponse\.rs
```rust
    fn version(&self) -> &str {
        self.version
    }
    fn status_code(&self) -> &str {
        self.status_code
    }
    fn status_text(&self) -> &str {
        self.status_text
    }
    fn headers(&self) -> String {
        let mut header_string = "".to_string();
        for (k, v) in self.headers.as_ref().unwrap().iter() {
            header_string = format!("{}{}: {}\r\n", header_string, k, v);
        }

        header_str
    }

    pub fn body(&self) -> &str {
        match &self.body {
            Some(b) => b.as_str(),
            None => "",
        }
    }
```
这里注意`unwrap`方法会取得所有权，我们需要使用`as_ref`把`Option`内部的变量加上引用，这样`unwrap`取出来的就是被引用的值。

### From
文件路径：hs/http/src/httpresponse\.rs
```rust
impl<'a> From<HttpResponse<'a>> for String {
    fn from(res: HttpResponse<'a>) -> Self {
        format!(
            "{} {} {}\r\n{}Content-Length: {}\r\n\r\n{}",
            res.version(),
            res.status_code(),
            res.status_text(),
            res.headers(),
            res.body.as_ref().unwrap().len(),
            res.body(),
        )
    }
}
```
实现了`From`后，我们就可以对`HttpResponse`使用`into`方法，将其转化为`String`。另外也可以将`HttpResponse`实例作为参数传入`String::from`中。
为`From`编写测试单元：
文件路径：hs/http/src/httpresponse\.rs
```rust
#[test]
fn test_res_into() {
    let res_string = "HTTP/1.1 404 Not Found\r\nContent-Type: Text/html\r\nContent-Length: 4\r\n\r\nxxxx".to_string();
    let res = HttpResponse {
        version: "HTTP/1.1",
        status_code: "404",
        status_text: "Not Found",
        headers: {
            let mut h = HashMap::new();
            h.insert("Content-Type", "Text/html");
            Some(h)
        },
        body: Some("xxxx".to_string()),
    };
    let res_into: String = res.into();
    assert_eq!(res_string, res_into);
}
```
运行测试：
```shell
cargo test -p http
```
测试通过。

### send_response
文件路径：hs/http/src/httpresponse\.rs
```rust
use std::io::{self, Write};

    pub fn send_response(&self, write_stream: &mut impl Write) -> io::Result<()> {
        let res = self.clone();
        let response_string = String::from(res);
        let _ = write!(write_stream, "{}", response_string);

        Ok(())
    }
```
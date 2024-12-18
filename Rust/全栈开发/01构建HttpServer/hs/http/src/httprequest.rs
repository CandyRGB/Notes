use std::collections::HashMap;

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

type Path = String;

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

        let mut should_check_req_line = true;
        let mut should_check_header_line = true;
        for line in s.lines().filter(|line| !line.is_empty()) {
            if should_check_req_line && line.contains("HTTP") {
                (parsed_method, parsed_path, parsed_version) = process_req_line(line);
                should_check_req_line = false;
            } else if should_check_header_line && line.contains(": ") {
                let (k, v) = process_header_line(line);
                parsed_headers.insert(k, v);
            } else {
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_method_into() {
        let m: Method = "GET".into();
        assert_eq!(m, Method::Get);
    }

    #[test]
    fn test_version_into() {
        let v: Version = "HTTP/1.1".into();
        assert_eq!(v, Version::V1_1);
    }

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
}

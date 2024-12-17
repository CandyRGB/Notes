use std::collections::HashMap;
use std::io::{self, Write};

#[derive(Debug, PartialEq, Clone)]
pub struct HttpResponse<'a> {
    version: &'a str,
    status_code: &'a str,
    status_text: &'a str,
    headers: Option<HashMap<&'a str, &'a str>>,
    body: Option<String>,
}

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
    pub fn send_response(&self, write_stream: &mut impl Write) -> io::Result<()> {
        let res = self.clone();
        let response_string = String::from(res);
        let _ = write!(write_stream, "{}", response_string)?;

        Ok(())
    }
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

        header_string
    }

    pub fn body(&self) -> &str {
        match &self.body {
            Some(b) => b.as_str(),
            None => "",
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

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
}

use std::cmp::PartialEq;
use std::fmt::{self, Display};
use std::str::FromStr;

// Method
#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Method {
    Get,
    Head,
    Post,
    Put,
    Delete,
    Options,
}
impl Method {
    pub fn from_string(string: &str) -> Option<Self> {
        match string.to_uppercase().as_str() {
            "GET" => Some(Self::Get),
            "HEAD" => Some(Self::Head),
            "POST" => Some(Self::Post),
            "PUT" => Some(Self::Put),
            "DELETE" => Some(Self::Delete),
            "OPTIONS" => Some(Self::Options),
            _ => None,
        }
    }
}
impl Display for Method {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Get => write!(f, "GET"),
            Self::Head => write!(f, "HEAD"),
            Self::Post => write!(f, "POST"),
            Self::Put => write!(f, "PUT"),
            Self::Delete => write!(f, "DELETE"),
            Self::Options => write!(f, "OPTIONS"),
        }
    }
}

// ResponseCode
#[derive(PartialEq, Debug, Clone)]
pub enum ResponseCode {
    Unknown = 0,
    Ok = 200,
    BadRequest = 400,
    Unauthorized = 401,
    Forbidden = 403,
    NotFound = 404,
    MethodNotAllowed = 405,
    InternalServerError = 500,
    NotImplemented = 501,
    BadGateway = 502,
    ServiceUnavailable = 503,
    GatewayTimeout = 504,
    HttpVersionNotSupported = 505,
}
impl ResponseCode {
    pub fn new(value: usize) -> Self {
        match value {
            200 => Self::Ok,
            400 => Self::BadRequest,
            401 => Self::Unauthorized,
            403 => Self::Forbidden,
            404 => Self::NotFound,
            405 => Self::MethodNotAllowed,
            500 => Self::InternalServerError,
            501 => Self::NotImplemented,
            502 => Self::BadGateway,
            503 => Self::ServiceUnavailable,
            504 => Self::GatewayTimeout,
            505 => Self::HttpVersionNotSupported,
            _ => Self::Unknown,
        }
    }
    pub fn to_string(&self) -> Option<&'static str> {
        match self {
            Self::Ok => Some("200 Ok"),
            Self::BadRequest => Some("400 Bad Request"),
            Self::Unauthorized => Some("401 Unauthorized"),
            Self::Forbidden => Some("403 Forbidden"),
            Self::NotFound => Some("404 Not Found"),
            Self::MethodNotAllowed => Some("405 Method Not Allowed"),
            Self::InternalServerError => Some("500 Internal Server Error"),
            Self::NotImplemented => Some("501 Not Implemented"),
            Self::BadGateway => Some("502 Bad Gateway"),
            Self::ServiceUnavailable => Some("503 Service Unavailable"),
            Self::GatewayTimeout => Some("504 Gateway Timeout"),
            Self::HttpVersionNotSupported => Some("505 Http Version Not Supported"),
            _ => None,
        }
    }

    pub fn reason_phrase(&self) -> Option<String> {
        match self {
            Self::Ok => Some(String::from("Ok")),
            Self::BadRequest => Some(String::from("Bad Request")),
            Self::Unauthorized => Some(String::from("Unauthorized")),
            Self::Forbidden => Some(String::from("Forbidden")),
            Self::NotFound => Some(String::from("Not Found")),
            Self::MethodNotAllowed => Some(String::from("Method Not Allowed")),
            Self::InternalServerError => Some(String::from("Internal Server Error")),
            Self::NotImplemented => Some(String::from("Not Implemented")),
            Self::BadGateway => Some(String::from("Bad Gateway")),
            Self::ServiceUnavailable => Some(String::from("Service Unavailable")),
            Self::GatewayTimeout => Some(String::from("Gate wayTime")),
            Self::HttpVersionNotSupported => Some(String::from("Http Version Not Supported")),
            _ => None,
        }
    }
}
// RequestLine
#[derive(Debug, Clone)]
pub struct RequestLine {
    pub version: HttpVersion,
    pub method: Method,
    pub resource: String,
}
impl RequestLine {
    pub fn new(version: HttpVersion, method: Method, resource: String) -> Self {
        Self {
            version,
            method,
            resource,
        }
    }

    pub fn from_string(string: &String) -> Option<Self> {
        let mut iter = string.split_whitespace();
        let size = iter.clone().count();
        if size != 3 {
            println!("expected size 3 , got {}", size);
            return None;
        }
        let method = Method::from_string(iter.next().unwrap());
        match method {
            None => {
                println!("unable to parse method");
                return None;
            }
            Some(_) => (),
        };
        let resource = iter.next().unwrap().to_string();
        let version = HttpVersion::from_string(iter.next().unwrap());
        match version {
            None => {
                println!("unable to parse version");
                return None;
            }
            Some(_) => (),
        };
        Some(Self {
            version: version.unwrap(),
            method: method.unwrap(),
            resource,
        })
    }
}
impl PartialEq for RequestLine {
    fn eq(&self, other: &Self) -> bool {
        self.version == other.version
            && self.method == other.method
            && self.resource == other.resource
    }
}
// StatusLine
#[derive(Debug, Clone)]
pub struct StatusLine {
    version: HttpVersion,
    response_code: ResponseCode,
}
impl StatusLine {
    pub fn new(version: HttpVersion, code: usize) -> Self {
        Self {
            version,
            response_code: ResponseCode::new(code),
        }
    }

    pub fn to_string(&self) -> String {
        let s = String::from(self.version.to_string()) + " ";
        let s = s + self.response_code.to_string().unwrap();
        s
    }
}
impl PartialEq for StatusLine {
    fn eq(&self, other: &Self) -> bool {
        self.version == other.version && self.response_code == other.response_code
    }
}

// Header
#[derive(Debug, Clone)]
pub struct Header {
    field_name: String,
    field_value: String,
}
impl Header {
    pub fn new(field_name: &str, field_value: &str) -> Self {
        Self {
            field_name: String::from(field_name),
            field_value: String::from(field_value),
        }
    }
    pub fn to_string(&self) -> String {
        format!("{}:{}", self.field_name, self.field_value)
    }
}
impl PartialEq for Header {
    fn eq(&self, other: &Header) -> bool {
        return other.field_name.trim() == self.field_name.trim()
            && other.field_value.trim() == self.field_value.trim();
    }
}
impl PartialEq<Header> for &Header {
    fn eq(&self, other: &Header) -> bool {
        return other.field_name.trim() == self.field_name.trim()
            && other.field_value.trim() == self.field_value.trim();
    }
}
impl PartialEq<Header> for &str {
    fn eq(&self, other: &Header) -> bool {
        let items: Vec<&str> = self.split(":").collect();
        return other.field_name.trim() == items[0].trim()
            && other.field_value.trim() == items[1].trim();
    }
}
impl Display for Header {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} : {}", self.field_name, self.field_value)
    }
}

// HttpVersion
#[derive(PartialEq, Debug, Clone)]
pub enum HttpVersion {
    HttpV1_0,
    HttpV1_1,
    HttpV2_0,
    HttpV3_0,
}
impl HttpVersion {
    pub fn from_string(string: &str) -> Option<Self> {
        match string {
            "HTTP/1.0" => Some(Self::HttpV1_0),
            "HTTP/1.1" => Some(Self::HttpV1_1),
            "HTTP/2.0" => Some(Self::HttpV2_0),
            "HTTP/3.0" => Some(Self::HttpV3_0),
            _ => None,
        }
    }
    pub fn to_string(&self) -> &'static str {
        match self {
            Self::HttpV1_0 => "HTTP/1.0",
            Self::HttpV1_1 => "HTTP/1.1",
            Self::HttpV2_0 => "HTTP/2.0",
            Self::HttpV3_0 => "HTTP/3.0",
        }
    }
}

// Request
#[derive(Debug)]
pub struct Request {
    pub request_line: RequestLine,
    pub headers: Vec<Header>,
    pub body: Vec<u8>,
}
impl Request {
    pub fn parse_from_string(request: &String) -> Option<Self> {
        let mut split = request.split("\r\n");
        let request_line = match split.next() {
            Some(maybe_request_line) => {
                match RequestLine::from_string(&maybe_request_line.to_string()) {
                    Some(valid_request_line) => valid_request_line,
                    None => return None,
                }
            }
            None => return None,
        };
        let mut headers = Vec::<Header>::new();
        loop {
            match split.next() {
                Some(header_string) => {
                    if header_string == "" {
                        break;
                    }
                    let header_split: Vec<&str> = header_string.split(':').collect();
                    if header_split.len() < 1 {
                        continue;
                    }
                    let header = Header {
                        field_name: String::from_str(header_split[0]).unwrap(),
                        field_value: if header_split.len() > 1 {
                            String::from_str(&header_split[1..].join(":").trim()).unwrap()
                        } else {
                            String::new()
                        },
                    };
                    headers.push(header);
                }
                None => {
                    break;
                }
            }
        }
        let body = Vec::<u8>::with_capacity(0);

        Some(Request {
            request_line,
            headers,
            body,
        })
    }
    pub fn parse_from_str(request: &'static str) -> Option<Self> {
        let mut split = request.split("\r\n");
        let request_line = match split.next() {
            Some(maybe_request_line) => {
                match RequestLine::from_string(&maybe_request_line.to_string()) {
                    Some(valid_request_line) => valid_request_line,
                    None => return None,
                }
            }
            None => return None,
        };
        let mut headers = Vec::<Header>::new();
        loop {
            match split.next() {
                Some(header_string) => {
                    if header_string == "" {
                        break;
                    }
                    let header_split: Vec<&str> = header_string.split(':').collect();
                    if header_split.len() < 1 {
                        continue;
                    }
                    let header = Header {
                        field_name: String::from_str(header_split[0]).unwrap(),
                        field_value: if header_split.len() > 1 {
                            String::from_str(&header_split[1..].join(":").trim()).unwrap()
                        } else {
                            String::new()
                        },
                    };
                    headers.push(header);
                }
                None => {
                    break;
                }
            }
        }
        let body = Vec::<u8>::with_capacity(0);

        Some(Request {
            request_line,
            headers,
            body,
        })
    }
    pub fn add_header(mut self, field_name: &String, field_value: &String) {
        self.headers.push(Header::new(field_name, field_value));
    }
}
// Response
#[derive(Debug, Clone)]
pub struct Response {
    status_line: StatusLine,
    headers: Vec<Header>,
    body: Vec<u8>,
}
impl Response {
    pub fn new(version: HttpVersion, response_code: ResponseCode, body: Vec<u8>) -> Self {
        Self {
            status_line: StatusLine {
                version,
                response_code,
            },
            headers: Vec::<Header>::new(),
            body,
        }
    }
    pub fn as_string(self) -> String {
        let output_string: String = self.status_line.to_string();
        let headers_string = self
            .headers
            .iter()
            .map(|header: &Header| header.to_string().trim().to_string())
            .collect::<Vec<String>>()
            .join("\r\n");
        let body_string = std::str::from_utf8(&self.body[..]).unwrap();
        format!(
            "{}\r\n{}\r\n\r\n{}",
            output_string, headers_string, body_string
        )
    }
    pub fn add_header(&mut self, field_name: &str, field_value: &str) {
        self.headers.push(Header::new(field_name, field_value));
    }

    pub fn body_length(&self) -> usize {
        self.body.len()
    }
}
// tests
#[cfg(test)]
mod tests {
    use super::{Header, HttpVersion, Method, RequestLine, ResponseCode, StatusLine};
    use crate::types::Request;

    #[test]
    pub fn method_to_string() {
        assert_eq!("GET", Method::Get.to_string());
        assert_eq!("HEAD", Method::Head.to_string());
        assert_eq!("POST", Method::Post.to_string());
        assert_eq!("PUT", Method::Put.to_string());
        assert_eq!("DELETE", Method::Delete.to_string());
        assert_eq!("OPTIONS", Method::Options.to_string());
    }

    #[test]
    pub fn match_method() {
        let get = "GET";
        let post = "Post";
        let unknown = "Random";

        assert_eq!(Some(Method::Get), Method::from_string(get));
        assert_eq!(Some(Method::Post), Method::from_string(post));
        assert_eq!(None, Method::from_string(unknown));
    }

    #[test]
    pub fn response_code_to_string() {
        let ok = ResponseCode::Ok;
        let bad_request = ResponseCode::BadRequest;
        let not_found = ResponseCode::NotFound;
        let internal_server_error = ResponseCode::InternalServerError;

        assert_eq!(ResponseCode::new(200), ok);
        assert_eq!(ResponseCode::new(400), bad_request);
        assert_eq!(ResponseCode::new(404), not_found);
        assert_eq!(ResponseCode::new(500), internal_server_error);
    }

    #[test]
    pub fn parse_request_line() {
        let request_line_string = "GET /home HTTP/1.1 ";
        let request_line =
            RequestLine::new(HttpVersion::HttpV1_1, Method::Get, "/home".to_string());
        assert_eq!(
            request_line,
            RequestLine::from_string(&request_line_string.to_string()).unwrap()
        )
    }

    #[test]
    pub fn parse_status_line() {
        let status_line_string = "HTTP/1.1 200 OK";
        let status_line = StatusLine::new(HttpVersion::HttpV1_1, 200);
        assert_eq!(
            status_line_string.to_lowercase(),
            status_line.to_string().to_lowercase()
        )
    }

    #[test]
    pub fn header_to_string() {
        let header = Header::new("Content-type", "application/json");
        assert_eq!("Content-type : application/json", header);
        assert_eq!("Content-type: application/json", header);
        assert_eq!("Content-type:application/json", header);
        assert_eq!(Header::new("Content-type", "application/json"), header);
    }

    #[test]
    pub fn parse_string_to_request() {
        const  REQUEST: &str = "GET / HTTP/1.1\r\nHost: localhost:50000\r\nConnection: keep-alive\r\nCache-Control: max-age=0\r\nsec-ch-ua: \"Not/A)Brand\";v=\"99\", \"Google Chrome\";v=\"115\", \"Chromium\";v=\"115\"\r\nsec-ch-ua-mobile: ?0\r\nsec-ch-ua-platform: \"macOS\"\r\nUpgrade-Insecure-Requests: 1\r\nUser-Agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/115.0.0.0 Safari/537.36\r\nAccept: text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;\r\n\r\nBody";
        let maybe_request = Request::parse_from_string(&REQUEST.to_string());
        assert_eq!(maybe_request.is_none(), false);
        assert_eq!(maybe_request.is_some(), true);
        let request = maybe_request.unwrap();
        assert_eq!(request.request_line.version, HttpVersion::HttpV1_1);
        assert_eq!(request.request_line.method, Method::Get);
        assert_eq!(request.headers.len(), 9);
        assert_eq!(
            request
                .headers
                .contains(&Header::new("Host", "localhost:50000")),
            true
        );
    }
}

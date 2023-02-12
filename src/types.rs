use std::cmp::PartialEq;
use std::fmt::{self, Display};

#[derive(PartialEq, Debug)]
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

#[derive(PartialEq, Debug)]
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

#[derive(Debug)]
pub struct RequestLine {
    version: HttpVersion,
    method: Method,
    resource: &'static str,
}

impl RequestLine {
    pub fn new(version: HttpVersion, method: Method, resource: &'static str) -> Self {
        Self {
            version,
            method,
            resource,
        }
    }

    pub fn from_string(string: &'static str) -> Option<Self> {
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
        let resource = iter.next().unwrap();
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

#[derive(Debug)]
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
#[derive(PartialEq, Debug)]
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

#[cfg(test)]
mod tests {
    use super::{Header, HttpVersion, Method, RequestLine, ResponseCode, StatusLine};

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
        let request_line = RequestLine::new(HttpVersion::HttpV1_1, Method::Get, "/home");
        assert_eq!(
            request_line,
            RequestLine::from_string(request_line_string).unwrap()
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
    }
}

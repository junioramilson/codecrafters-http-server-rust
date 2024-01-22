use core::fmt;

#[derive(PartialEq, Debug, Eq)]
pub enum StatusCodes {
    Ok = 200,
    NotFound = 404,
    Created = 201,
}

pub enum HttpMethod {
    Get,
    Post,
    Put,
    Delete,
}

impl fmt::Display for HttpMethod {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            HttpMethod::Get => write!(f, "GET"),
            HttpMethod::Post => write!(f, "POST"),
            HttpMethod::Put => write!(f, "PUT"),
            HttpMethod::Delete => write!(f, "DELETE"),
        }
    }
}

impl From<&str> for HttpMethod {
    fn from(method: &str) -> Self {
        match method {
            "GET" => HttpMethod::Get,
            "POST" => HttpMethod::Post,
            "PUT" => HttpMethod::Put,
            "DELETE" => HttpMethod::Delete,
            _ => HttpMethod::Get,
        }
    }
}

impl Into<String> for HttpMethod {
    fn into(self) -> String {
        match self {
            HttpMethod::Get => String::from("GET"),
            HttpMethod::Post => String::from("POST"),
            HttpMethod::Put => String::from("PUT"),
            HttpMethod::Delete => String::from("DELETE"),
        }
    }
}

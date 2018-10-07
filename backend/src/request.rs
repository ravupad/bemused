use http::request::Parts;
use hyper::Method;
use hyper::{header::HeaderValue, HeaderMap};

pub struct Request {
    pub uri: String,
    pub method: Method,
    pub headers: HeaderMap<HeaderValue>,
    pub path: Vec<String>,
    pub query_params: Vec<(String, String)>,
    pub body: Vec<u8>,
}

impl Request {
    pub fn new(parts: Parts, body: Vec<u8>) -> Self {
        let uri = parts.uri.to_string();
        let method = parts.method;
        let headers = parts.headers;
        let path = parts
            .uri
            .path()
            .split('/')
            .skip(1)
            .map(|p| p.to_owned())
            .collect();
        let query_params = parts
            .uri
            .query()
            .map(|q| {
                q.split('&')
                    .map(|kv| kv.splitn(2, '=').collect())
                    .filter(|kv: &Vec<_>| kv.len() == 2)
                    .map(|kv| (kv[0].to_owned(), kv[1].to_owned()))
                    .collect()
            })
            .unwrap_or_default();
        Self {
            uri,
            method,
            headers,
            path,
            query_params,
            body,
        }
    }

    pub fn path(&self) -> Vec<&str> {
        self.path.iter().map(|p| p.as_str()).collect()
    }
}

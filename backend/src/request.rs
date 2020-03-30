use http::header::HeaderMap;
use http::header::HeaderValue;
use http::method::Method;
use http::request::Parts;

pub struct Request<'a> {
    pub uri: String,
    pub method: Method,
    pub headers: HeaderMap<HeaderValue>,
    pub path: Vec<&'a str>,
    pub query_params: Vec<(String, String)>,
    pub body: Vec<u8>,
}

impl<'a> Request<'a> {
    pub fn new(parts: &'a Parts, body: Vec<u8>) -> Self {
        let uri = parts.uri.to_string();
        let method = parts.method.clone();
        let headers = parts.headers.clone();
        let path = parts
            .uri
            .path()
            .split('/')
            .skip(1)
            .collect::<Vec<&'a str>>();
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
}

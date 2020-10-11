use super::method::Method;
pub struct Request {
    path: String,
    query_string: Option<String>, // Means that query_string may be null or String
    method: Method,
}
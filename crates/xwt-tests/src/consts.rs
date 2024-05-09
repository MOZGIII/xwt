use crate::concat;

pub const BASE_URL: &str = "https://127.0.0.1:8080";

pub const ECHO_SERVER_URL: &str = concat!(BASE_URL, "/echo");
pub const ECHO_OPEN_BI_SERVER_URL: &str = concat!(BASE_URL, "/echo-open-bi");
pub const ECHO_CLOSE_SERVER_URL: &str = concat!(BASE_URL, "/close");

#[derive(Debug, Clone)]
pub struct CachePath(String);

impl CachePath {
    pub fn from(s: String) -> CachePath {
        CachePath(s)
    }
}

impl AsRef<str> for CachePath {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

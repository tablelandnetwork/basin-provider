#[derive(Debug, Clone)]
pub struct CachePath(String);

impl CachePath {
    pub fn from(s: String) -> CachePath {
        CachePath(s)
    }

    pub fn filename(&self) -> Option<String> {
        self.0.split('/').last().map(|x| x.to_string())
    }
}

impl AsRef<str> for CachePath {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
